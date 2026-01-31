# Upgrade Assistant Delegate

A general-purpose delegate that helps ANY delegate upgrade gracefully by tracking delegate key mappings.

## Problem

When a delegate's WASM code changes, its delegate key changes (derived from code hash). This creates a new, empty delegate storage, making data from the previous version inaccessible.

## Solution

The Upgrade Assistant stores delegate key mappings, partitioned by origin:
- Maps namespace strings to their current delegate key
- Each UI contract has its own isolated namespace
- Origin is cryptographically verified by Freenet (cannot be spoofed)

## Usage

### On Startup (in your delegate)

```rust
// 1. Query Upgrade Assistant for previous key
let request = UpgradeAssistantRequest::GetPreviousKey {
    namespace: None,  // Use None for single-delegate apps, or Some("name") for multi-delegate
};
// Send to Upgrade Assistant delegate...

// 2. Compare with current key
let current_key = get_current_delegate_key();
if let Some(prev_key) = response.delegate_key {
    if prev_key != current_key.bytes() {
        // Migrate data from old delegate
        migrate_from_previous_delegate(prev_key).await;
    }
}

// 3. Update Upgrade Assistant with current key
let request = UpgradeAssistantRequest::SetCurrentKey {
    namespace: None,
    delegate_key: current_key.bytes(),
    code_hash: current_key.code_hash().bytes(),
};
// Send to Upgrade Assistant delegate...
```

## API

### Request Types

```rust
enum UpgradeAssistantRequest {
    /// Get the stored delegate key for a namespace
    GetPreviousKey { namespace: Option<String> },

    /// Store/update the delegate key for a namespace
    SetCurrentKey {
        namespace: Option<String>,
        delegate_key: [u8; 32],
        code_hash: [u8; 32],
    },
}
```

### Response Types

```rust
enum UpgradeAssistantResponse {
    /// Response to GetPreviousKey
    PreviousKey {
        namespace: Option<String>,
        delegate_key: Option<[u8; 32]>,  // None if never registered
        code_hash: Option<[u8; 32]>,
    },

    /// Response to SetCurrentKey
    KeyUpdated { namespace: Option<String> },
}
```

## Security Model

- Storage is partitioned by **attested origin** (contract key of requesting UI)
- Different UIs cannot see or modify each other's data
- The origin is cryptographically verified by Freenet and cannot be spoofed

## Building

```bash
cargo build --release --target wasm32-unknown-unknown
```

The compiled WASM will be at `target/wasm32-unknown-unknown/release/upgrade_assistant.wasm`.

## WASM Binary

The `wasm/upgrade_assistant.wasm` file is the canonical binary. This is committed to the repo because:
- The delegate key is derived from the WASM hash
- Everyone must use the exact same binary to get the same key
- Prevents "works on my machine" issues with different compiler versions

## Self-Migration

The Upgrade Assistant can upgrade itself using the same pattern it provides to others. See `src/previous_versions.rs` for the upgrade process.
