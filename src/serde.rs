mod size_hint {
    use core::cmp;

    /// This presumably exists to prevent denial of service attacks.
    ///
    /// Original discussion: https://github.com/serde-rs/serde/issues/1114.
    #[cfg_attr(feature = "inline-more", inline)]
    pub(super) fn cautious(hint: Option<usize>) -> usize {
        cmp::min(hint.unwrap_or(0), 4096)
    }
}

// LinkedHashMap impls
mod map {
    use core::{
        fmt::{self, Formatter},
        hash::{BuildHasher, Hash},
        marker::PhantomData,
    };

    use serde_::{
        de::{MapAccess, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use crate::linked_hash_map::LinkedHashMap;

    use super::size_hint;

    impl<K, V, H> Serialize for LinkedHashMap<K, V, H>
    where
        K: Serialize + Eq + Hash,
        V: Serialize,
        H: BuildHasher,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_map(self)
        }
    }

    #[derive(Debug)]
    pub struct LinkedHashMapVisitor<K, V, S> {
        marker: PhantomData<LinkedHashMap<K, V, S>>,
    }

    impl<K, V, S> LinkedHashMapVisitor<K, V, S> {
        fn new() -> Self {
            LinkedHashMapVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<K, V, S> Default for LinkedHashMapVisitor<K, V, S> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<'de, K, V, S> Visitor<'de> for LinkedHashMapVisitor<K, V, S>
    where
        K: Deserialize<'de> + Eq + Hash,
        V: Deserialize<'de>,
        S: BuildHasher + Default,
    {
        type Value = LinkedHashMap<K, V, S>;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            write!(formatter, "a map")
        }

        #[cfg_attr(feature = "inline-more", inline)]
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
            let mut values = LinkedHashMap::with_capacity_and_hasher(
                size_hint::cautious(map.size_hint()),
                S::default(),
            );

            while let Some((k, v)) = map.next_entry()? {
                values.insert(k, v);
            }

            Ok(values)
        }
    }

    impl<'de, K, V, S> Deserialize<'de> for LinkedHashMap<K, V, S>
    where
        K: Deserialize<'de> + Eq + Hash,
        V: Deserialize<'de>,
        S: BuildHasher + Default,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_map(LinkedHashMapVisitor::default())
        }
    }
}

// LinkedHashSet impls
mod set {
    use core::{
        fmt::{self, Formatter},
        hash::{BuildHasher, Hash},
        marker::PhantomData,
    };

    use serde_::{
        de::{SeqAccess, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use crate::linked_hash_set::LinkedHashSet;

    use super::size_hint;

    impl<T, H> Serialize for LinkedHashSet<T, H>
    where
        T: Serialize + Eq + Hash,
        H: BuildHasher,
    {
        #[cfg_attr(feature = "inline-more", inline)]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(self)
        }
    }

    #[derive(Debug)]
    pub struct LinkedHashSetVisitor<T, S> {
        marker: PhantomData<LinkedHashSet<T, S>>,
    }

    impl<T, S> LinkedHashSetVisitor<T, S> {
        fn new() -> Self {
            LinkedHashSetVisitor {
                marker: PhantomData,
            }
        }
    }

    impl<T, S> Default for LinkedHashSetVisitor<T, S> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<'de, T, H> Visitor<'de> for LinkedHashSetVisitor<T, H>
    where
        T: Deserialize<'de> + Eq + Hash,
        H: BuildHasher + Default,
    {
        type Value = LinkedHashSet<T, H>;

        fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
            write!(formatter, "a sequence")
        }

        #[cfg_attr(feature = "inline-more", inline)]
        fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
            let mut values = LinkedHashSet::with_capacity_and_hasher(
                size_hint::cautious(seq.size_hint()),
                H::default(),
            );

            while let Some(v) = seq.next_element()? {
                values.insert(v);
            }

            Ok(values)
        }
    }

    impl<'de, T, S> Deserialize<'de> for LinkedHashSet<T, S>
    where
        T: Deserialize<'de> + Eq + Hash,
        S: BuildHasher + Default,
    {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserializer.deserialize_seq(LinkedHashSetVisitor::default())
        }
    }
}
