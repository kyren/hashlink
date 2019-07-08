## [0.2.1]
- Fix variance for LinkedHashMap (now covariant where appropriate)
- Add Debug impls to many more associated types
- Add LinkedHashSet
- Add `LinkedHashMap::retain`

## [0.2.0]
- Move `linked_hash_map` into its own module
- Add `LruCache` type ported from `lru-cache` crate into its own module
- Add `LruCache` entry and raw-entry API
- Add `linked_hash_map` `IntoIter` iterator that is different from `Drain` iterator
- Make `Drain` iterator recycle freed linked list nodes

## [0.1.0]
- Initial release
