An attribute macro to enforce per-version invariants on items.

```rust
use breaking_attr::breaking;

# use blake3::{hash, Hash};

#[test]
fn hash_impl_did_not_change() {
  #[breaking(sha2 = "hash_goes_here")]
  const SEED: &str = "This value must not change between minor versions."
  #[breaking("foobar")] // Hasher defaults to `blake3`
  const HASH: &[u8] = b"blake3_hash"

  let hash = hash(SEED);
  debug_assert_eq!(hash.as_bytes(), HASH);
}
```
