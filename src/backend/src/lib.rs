#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::collections::HashMap;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Default)]
struct SkillExchangeEntry {
    id: u64,
    user_id: u64,
    title: String,
    description: String,
    category: String,
    skills_offered: Vec<String>,
    skills_wanted: Vec<String>,
    location: String,
    created_at: u64,
    updated_at: Option<u64>,
}

impl Storable for SkillExchangeEntry {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SkillExchangeEntry {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static SKILL_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static SKILL_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(SKILL_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for skill exchange entries")
    );

    static SKILL_STORAGE: RefCell<StableBTreeMap<u64, SkillExchangeEntry, Memory>> =
        RefCell::new(StableBTreeMap::init(
            SKILL_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

fn do_insert_skill_exchange_entry(entry: &SkillExchangeEntry) {
    SKILL_STORAGE.with(|service| service.borrow_mut().insert(entry.id, entry.clone()));
}

#[derive(candid::CandidType, Default)]
struct SkillExchangeUpdatePayload {
    title: String,
    description: String,
    category: String,
    skills_offered: Vec<String>,
    skills_wanted: Vec<String>,
    location: String,
}

#[ic_cdk::query]
fn get_skill_exchange_entry(id: u64) -> Result<SkillExchangeEntry, Error> {
    match _get_skill_exchange_entry(&id) {
        Some(entry) => Ok(entry),
        None => Err(Error::NotFound {
            msg: format!("a skill exchange entry with id={} not found", id),
        }),
    }
}

fn _get_skill_exchange_entry(id: &u64) -> Option<SkillExchangeEntry> {
    SKILL_STORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_skill_exchange_entry(entry: SkillExchangeUpdatePayload) -> Option<SkillExchangeEntry> {
    let id = SKILL_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .unwrap_or_else(|| panic!("cannot increment id counter for skill exchange entries"));

    let timestamp = time();
    let skill_exchange_entry = SkillExchangeEntry {
        id,
        user_id: 0, // Set the user_id accordingly
        title: entry.title,
        description: entry.description,
        category: entry.category,
        skills_offered: entry.skills_offered,
        skills_wanted: entry.skills_wanted,
        location: entry.location,
        created_at: timestamp,
        updated_at: None,
    }.unwrap_or_default();

    do_insert_skill_exchange_entry(&skill_exchange_entry);
    Some(skill_exchange_entry)
}

#[ic_cdk::update]
fn update_skill_exchange_entry(
    id: u64,
    payload: SkillExchangeUpdatePayload,
) -> Result<SkillExchangeEntry, Error> {
    match SKILL_STORAGE.with(|service| service.borrow_mut().get_mut(&id)) {
        Some(skill_exchange_entry) => {
            skill_exchange_entry.title = payload.title;
            skill_exchange_entry.description = payload.description;
            skill_exchange_entry.category = payload.category;
            skill_exchange_entry.skills_offered = payload.skills_offered;
            skill_exchange_entry.skills_wanted = payload.skills_wanted;
            skill_exchange_entry.location = payload.location;
            skill_exchange_entry.updated_at = Some(time());
            do_insert_skill_exchange_entry(skill_exchange_entry);
            Ok(skill_exchange_entry.clone())
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a skill exchange entry with id={}. entry not found",
                id
            ),
        }),
    }
}

#[ic_cdk::update]
fn delete_skill_exchange_entry(id: u64) -> Result<SkillExchangeEntry, Error> {
    match SKILL_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(skill_exchange_entry) => Ok(skill_exchange_entry),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a skill exchange entry with id={}. entry not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_all_skill_exchange_entries() -> Vec<SkillExchangeEntry> {
    SKILL_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.clone()).collect()
    })
}

#[ic_cdk::query]
fn search_skill_exchange_entries_by_skill_wanted(skill: String) -> Vec<SkillExchangeEntry> {
    SKILL_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, entry)| {
                if entry.skills_wanted.contains(&skill) {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn search_skill_exchange_entries_by_skill_offered(skill: String) -> Vec<SkillExchangeEntry> {
    SKILL_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, entry)| {
                if entry.skills_offered.contains(&skill) {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_skill_exchange_entries_by_location(location: String) -> Vec<SkillExchangeEntry> {
    SKILL_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, entry)| {
                if entry.location == location {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_skill_exchange_entries_by_category(category: String) -> Vec<SkillExchangeEntry> {
    SKILL_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow
            .iter()
            .filter_map(|(_, entry)| {
                if entry.category == category {
                    Some(entry.clone())
                } else {
                    None
                }
            })
            .collect()
    })
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
