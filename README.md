This is a (fairly divergent) fork of
[linked-hash-map](https://github.com/contain-rs/linked-hash-map) that adds some
missing APIs.

It is based on the `HashMap` directly from the `hashbrown` crate, rather than
the one in `std`, and it offers a "raw" `HashMap` API that mimics the one in
`hashbrown`.

The most important API change is that when a `LinkedHashMap` is used as a LRU
cache, it allows you to do things like this to avoid repeated key hashing and
lookups:

``` rust
let mut lru_cache = LinkedHashMap::new();
// Try to find my expensive to hash key
let cached_val = match lru_cache.entry("key") {
    Entry::Occupied(occupied) => {
        // Cache hit, move entry to the back.
        occupied.to_back();
        occupied.get_mut()
    }
    Entry::Vacant(vacant) => {
        // Insert expensive to compute value, automatically inserted at the back.
        vacant.insert(42)
    }
};
```

Right now is extremely WIP and only contains `LinkedHashMap`, but eventually it
should also have `LinkedHashSet` and a `LruCahe` wrapper type.

This library contains a decent amount of unsafe code from handling its internal
linked list.  Probably do not use this for anything important until I and others
have had more time to carefully review it.

## License

This library is licensed the same as
[linked-hash-map](https://github.com/contain-rs/linked-hash-map), it is licensed
under either of:

* MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
* Apache License 2.0 [LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0

at your option.
