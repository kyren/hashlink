use core::{
    fmt::{self, Formatter},
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::{LinkedHashMap, LinkedHashSet, LruCache};

// LinkedHashMap impls

impl<K, V, S> Serialize for LinkedHashMap<K, V, S>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
    S: BuildHasher,
{
    #[inline]
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut map_serializer = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map_serializer.serialize_key(k)?;
            map_serializer.serialize_value(v)?;
        }
        map_serializer.end()
    }
}

impl<'de, K, V, S> Deserialize<'de> for LinkedHashMap<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: BuildHasher + Default,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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

            #[inline]
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let mut values = LinkedHashMap::with_capacity_and_hasher(
                    map.size_hint().unwrap_or(0),
                    S::default(),
                );

                while let Some((k, v)) = map.next_entry()? {
                    values.insert(k, v);
                }

                Ok(values)
            }
        }

        deserializer.deserialize_map(LinkedHashMapVisitor::default())
    }
}

// LinkedHashSet impls

impl<T, S> Serialize for LinkedHashSet<T, S>
where
    T: Serialize + Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn serialize<U: Serializer>(&self, serializer: U) -> Result<U::Ok, U::Error> {
        let mut seq_serializer = serializer.serialize_seq(Some(self.len()))?;
        for v in self {
            seq_serializer.serialize_element(v)?;
        }
        seq_serializer.end()
    }
}

impl<'de, T, S> Deserialize<'de> for LinkedHashSet<T, S>
where
    T: Deserialize<'de> + Eq + Hash,
    S: BuildHasher + Default,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
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

        impl<'de, T, S> Visitor<'de> for LinkedHashSetVisitor<T, S>
        where
            T: Deserialize<'de> + Eq + Hash,
            S: BuildHasher + Default,
        {
            type Value = LinkedHashSet<T, S>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(formatter, "a sequence")
            }

            #[inline]
            fn visit_seq<SA: SeqAccess<'de>>(self, mut seq: SA) -> Result<Self::Value, SA::Error> {
                let mut values = LinkedHashSet::with_capacity_and_hasher(
                    seq.size_hint().unwrap_or(0),
                    S::default(),
                );

                while let Some(v) = seq.next_element()? {
                    values.insert(v);
                }

                Ok(values)
            }
        }

        deserializer.deserialize_seq(LinkedHashSetVisitor::default())
    }
}

// LruCache impls

impl<K, V, S> Serialize for LruCache<K, V, S>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
    S: BuildHasher + Default,
{
    #[inline]
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut state = serializer.serialize_struct("LruCache", 2)?;
        state.serialize_field("map", &self.map)?;
        state.serialize_field("max_size", &self.max_size)?;
        state.end()
    }
}

impl<'de, K, V, S> Deserialize<'de> for LruCache<K, V, S>
where
    K: Deserialize<'de> + Eq + Hash,
    V: Deserialize<'de>,
    S: BuildHasher + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Map,
            MaxSize,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`map` or `max_size`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "map" => Ok(Field::Map),
                            "max_size" => Ok(Field::MaxSize),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        #[derive(Debug)]
        struct LruCacheVisitor<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher + Default,
        {
            marker: PhantomData<LruCache<K, V, S>>,
        }

        impl<K, V, S> LruCacheVisitor<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher + Default,
        {
            fn new() -> Self {
                LruCacheVisitor {
                    marker: PhantomData,
                }
            }
        }

        impl<K, V, S> Default for LruCacheVisitor<K, V, S>
        where
            K: Eq + Hash,
            S: BuildHasher + Default,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'de, K, V, S> Visitor<'de> for LruCacheVisitor<K, V, S>
        where
            K: Deserialize<'de> + Eq + Hash,
            V: Deserialize<'de>,
            S: BuildHasher + Default,
        {
            type Value = LruCache<K, V, S>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct LruCache")
            }

            fn visit_seq<M>(self, mut outseq: M) -> Result<LruCache<K, V, S>, M::Error>
            where
                M: SeqAccess<'de>,
            {
                let map = outseq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let max_size = outseq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(LruCache::<K, V, S> { map, max_size })
            }

            fn visit_map<M>(self, mut outmap: M) -> Result<LruCache<K, V, S>, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = None;
                let mut max_size = None;
                while let Some(key) = outmap.next_key()? {
                    match key {
                        Field::Map => {
                            if map.is_some() {
                                return Err(de::Error::duplicate_field("map"));
                            }
                            map = Some(outmap.next_value()?);
                        }
                        Field::MaxSize => {
                            if max_size.is_some() {
                                return Err(de::Error::duplicate_field("max_size"));
                            }
                            max_size = Some(outmap.next_value()?);
                        }
                    }
                }
                let map = map.ok_or_else(|| de::Error::missing_field("map"))?;
                let max_size = max_size.ok_or_else(|| de::Error::missing_field("max_size"))?;
                Ok(LruCache::<K, V, S> { map, max_size })
            }
        }

        const FIELDS: &'static [&'static str] = &["map", "max_size"];
        deserializer.deserialize_struct("LruCache", FIELDS, LruCacheVisitor::default())
    }
}
