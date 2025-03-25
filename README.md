[![Crates.io Version](https://img.shields.io/crates/v/breaking-attr)](https://crates.io/crates/breaking-attr)
[![docs.rs](https://img.shields.io/docsrs/breaking-attr)](https://docs.rs/breaking-attr)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Waridley/breaking-attr/.github%2Fworkflows%2Fci.yml)](https://github.com/Waridley/breaking-attr/actions)


An attribute macro to enforce per-version invariants on items.

```rust
use breaking_attr::breaking;

#[test]
fn hash_impl_did_not_change() {
    #[breaking(sha2 = "zEk8F98i1LX-Rr070lCztGerzO1-qlLbosetY1UvSww=")]
    const SEED: &str = "This value must not change between minor versions.";

    #[breaking("3zIKlGMq-shMmThdYSOntWhl9QCx3A23i8tnEfNPWBY=")] // defaults to `blake3`
    const HASH: u64 = 42;
    
    // Just for example:
    let mut hasher = std::hash::DefaultHasher::new();
    hasher.write(SEED.as_bytes());
    debug_assert_eq!(hasher.finish(), HASH);
}
```
