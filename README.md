An attribute macro to enforce per-version invariants on items.

```rust
use breaking_attr::breaking;

# use blake3::{hash, Hash};

#[test]
fn hash_impl_did_not_change() {
    #[breaking(sha2 = "zEk8F98i1LX-Rr070lCztGerzO1-qlLbosetY1UvSww=")]
    const SEED: &str = "This value must not change between minor versions.";
    #[breaking("ipCxMeWK-ImjLQVZX7YQXqjguSC5jOc5oCexXl5Bw0g=")] // Hasher defaults to `blake3`
    const HASH: &[u8] = &[89, 216, 231, 42, 61, 80, 228, 91, 200, 17, 218, 190, 61, 187, 1, 13, 90, 195, 84, 57, 249, 109, 90, 79, 175, 103, 37, 195, 98, 32, 157, 102];
    let hash = hash(SEED.as_bytes());
    debug_assert_eq!(hash.as_bytes(), HASH);
}
```
