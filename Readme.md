# Skill Exchange Service

This repository contains a Canister for managing skill exchange entries. The service provides functionalities to add, delete, and retrieve information about skill exchange entries. It includes features like listing all entries, searching by category, location, and skills, and updating existing entries.

## Data Structures

### `Error`
Represents error types, including a `NotFound` variant with a descriptive message.

### `Result`
A variant representing the result of operations. Includes an `Ok` variant with a `SkillExchangeEntry` or an `Err` variant with an `Error`.

### `SkillExchangeEntry`
A struct representing a skill exchange entry with attributes such as ID, title, creation and update timestamps, user ID, skills wanted, category, skills offered, and location.

### `SkillExchangeUpdatePayload`
A payload structure for updating skill exchange entries, including title, description, skills wanted, category, skills offered, and location.

## Service Functions

1. **add_skill_exchange_entry:**
   - Adds a new skill exchange entry with automatically generated ID and creation timestamp.

2. **delete_skill_exchange_entry:**
   - Deletes a skill exchange entry based on the provided ID.

3. **get_all_skill_exchange_entries:**
   - Retrieves a list of all stored skill exchange entries.

4. **get_skill_exchange_entries_by_category:**
   - Retrieves skill exchange entries that match a given category.

5. **get_skill_exchange_entries_by_location:**
   - Retrieves skill exchange entries that match a given location.

6. **get_skill_exchange_entry:**
   - Retrieves detailed information about a specific skill exchange entry based on its ID.

7. **search_skill_exchange_entries_by_skill_offered:**
   - Retrieves skill exchange entries that match a given offered skill.

8. **search_skill_exchange_entries_by_skill_wanted:**
   - Retrieves skill exchange entries that match a given wanted skill.

9. **update_skill_exchange_entry:**
   - Updates a skill exchange entry based on the provided ID and `SkillExchangeUpdatePayload`.

## Candid Interface

The canister exports its Candid interface definitions using the `ic_cdk::export_candid!()` macro.

## Error Handling

Errors are represented using the `Error` enum, which includes a `NotFound` variant with a descriptive message.

Feel free to explore and integrate this canister into your Internet Computer project for efficient skill exchange management!