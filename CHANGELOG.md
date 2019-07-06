## [0.2.0]
- Move `linked_hash_map` into its own module
- Add `LruCache` type ported from `lru-cache` crate into its own module
- Add `LruCache` entry and raw-entry API
- Add `linked_hash_map` `IntoIter` iterator that is different from `Drain` iterator
- Make `Drain` iterator recycle freed linked list nodes

## [0.1.0]
- Initial release
