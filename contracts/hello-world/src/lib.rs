#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, String, Vec};

// Structure to store encrypted password entry
#[contracttype]
#[derive(Clone)]
pub struct PasswordEntry {
    pub entry_id: u64,
    pub service_name: String,
    pub username: String,
    pub encrypted_password: String,
    pub created_at: u64,
    pub updated_at: u64,
}

// Mapping user address to their password entries
#[contracttype]
pub enum PasswordVault {
    UserEntry(Address, u64), // (user_address, entry_id)
    UserCount(Address),      // Track number of entries per user
}

#[contract]
pub struct PasswordManagerContract;

#[contractimpl]
impl PasswordManagerContract {

    /// Store a new encrypted password entry
    /// User must encrypt the password client-side before calling this function
    pub fn store_password(
        env: Env,
        user: Address,
        service_name: String,
        username: String,
        encrypted_password: String,
    ) -> u64 {
        // Verify the caller is the user
        user.require_auth();

        // Get current entry count for user
        let mut entry_count: u64 = env
            .storage()
            .persistent()
            .get(&PasswordVault::UserCount(user.clone()))
            .unwrap_or(0);

        entry_count += 1;

        let timestamp = env.ledger().timestamp();

        // Create new password entry
        let new_entry = PasswordEntry {
            entry_id: entry_count,
            service_name,
            username,
            encrypted_password,
            created_at: timestamp,
            updated_at: timestamp,
        };

        // Store the entry
        env.storage()
            .persistent()
            .set(
                &PasswordVault::UserEntry(user.clone(), entry_count),
                &new_entry,
            );

        // Update user's entry count
        env.storage()
            .persistent()
            .set(&PasswordVault::UserCount(user.clone()), &entry_count);

        env.storage().persistent().extend_ttl(
            &PasswordVault::UserEntry(user.clone(), entry_count),
            5000,
            5000,
        );
        env.storage().persistent().extend_ttl(
            &PasswordVault::UserCount(user.clone()),
            5000,
            5000,
        );

        log!(&env, "Password stored successfully for entry_id: {}", entry_count);

        entry_count
    }

    /// Retrieve an encrypted password entry by entry_id
    pub fn get_password(env: Env, user: Address, entry_id: u64) -> PasswordEntry {
        // Verify the caller is the user
        user.require_auth();

        let key = PasswordVault::UserEntry(user.clone(), entry_id);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(PasswordEntry {
                entry_id: 0,
                service_name: String::from_str(&env, "Not_Found"),
                username: String::from_str(&env, "Not_Found"),
                encrypted_password: String::from_str(&env, "Not_Found"),
                created_at: 0,
                updated_at: 0,
            })
    }

    /// Update an existing password entry
    pub fn update_password(
        env: Env,
        user: Address,
        entry_id: u64,
        encrypted_password: String,
    ) {
        // Verify the caller is the user
        user.require_auth();

        let key = PasswordVault::UserEntry(user.clone(), entry_id);

        let mut entry: PasswordEntry = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Password entry not found");

        // Update the password and timestamp
        entry.encrypted_password = encrypted_password;
        entry.updated_at = env.ledger().timestamp();

        // Store updated entry
        env.storage().persistent().set(&key, &entry);

        env.storage().persistent().extend_ttl(&key, 5000, 5000);

        log!(&env, "Password updated for entry_id: {}", entry_id);
    }

    /// Get total number of password entries for a user
    pub fn get_entry_count(env: Env, user: Address) -> u64 {
        user.require_auth();

        env.storage()
            .persistent()
            .get(&PasswordVault::UserCount(user))
            .unwrap_or(0)
    }
}