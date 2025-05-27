# Deterministic `rustdoc_types`

This program transforms the JSON outputted by `rustdoc` to make it deterministic across environments and compiler versions when a crate is unchanged.

## Passes

- Sorts all maps by key
- Makes item IDs stable
- Makes crate IDs stable
- Removes target-specific information
