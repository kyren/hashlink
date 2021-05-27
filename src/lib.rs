#![no_std]

extern crate alloc;

mod map;
mod set;

pub mod linked_hash_map {
    //! A linked hash map implementation. The order of entries defaults to "insertion order".
    pub use crate::map::*;
}
pub mod linked_hash_set {
    //! A linked hash set implementation. The order of entries defaults to "insertion order".
    pub use crate::set::*;
}

#[cfg(feature = "lru")]
pub mod lru_cache;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "lru")]
pub use crate::lru_cache::LruCache;
pub use crate::map::LinkedHashMap;
pub use crate::set::LinkedHashSet;

#[doc(inline)]
pub use hashbrown::hash_map::DefaultHashBuilder;
pub use hashbrown::TryReserveError;
