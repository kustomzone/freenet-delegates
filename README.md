# Freenet Delegates

A collection of standard, useful delegates for the Freenet ecosystem.

## Available Delegates

### [upgrade-assistant](./upgrade-assistant/)

A general-purpose delegate that helps ANY delegate upgrade gracefully by tracking delegate key mappings.

**Problem:** When a delegate's WASM code changes, its delegate key changes (derived from code hash). This creates a new, empty delegate storage, making data from the previous version inaccessible.

**Solution:** The Upgrade Assistant stores delegate key mappings, partitioned by origin, so delegates can find and migrate from their previous versions.

## Building

```bash
# Build all delegates
cargo build --release --target wasm32-unknown-unknown

# Build specific delegate
cargo build --release --target wasm32-unknown-unknown -p upgrade-assistant
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific delegate
cargo test -p upgrade-assistant
```

## License

LGPL-3.0-only
