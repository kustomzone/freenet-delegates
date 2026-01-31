//! Known previous Upgrade Assistant versions for self-migration.
//!
//! When upgrading the Upgrade Assistant itself, add the current version's
//! delegate key and code hash here BEFORE releasing the new version.
//!
//! ## Upgrade Process for the Upgrade Assistant
//!
//! 1. Record current delegate key and code hash
//! 2. Add them to `PREVIOUS_UPGRADE_ASSISTANT_KEYS`
//! 3. Make code changes (bug fix, new feature, etc.)
//! 4. Build new WASM: `cargo build --release --target wasm32-unknown-unknown`
//! 5. Copy new WASM to `wasm/upgrade_assistant.wasm`
//! 6. Commit both the code changes AND the new WASM
//! 7. New version will automatically migrate data from old version

/// Information about a previous version of the Upgrade Assistant
pub struct PreviousVersion {
    /// The delegate key (derived from WASM hash + parameters)
    pub delegate_key: [u8; 32],
    /// The code hash of the WASM
    pub code_hash: [u8; 32],
    /// Version number for logging/debugging
    pub version: u32,
}

/// Known previous Upgrade Assistant keys (for self-migration).
///
/// IMPORTANT: Add the current key here BEFORE releasing a new version!
///
/// The Upgrade Assistant is designed to be extremely stable and rarely change.
/// If it must change, this array enables self-migration.
pub const PREVIOUS_UPGRADE_ASSISTANT_KEYS: &[PreviousVersion] = &[
    // Uncomment and add entries when upgrading:
    //
    // PreviousVersion {
    //     delegate_key: [
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     ],
    //     code_hash: [
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     ],
    //     version: 1,
    // },
];
