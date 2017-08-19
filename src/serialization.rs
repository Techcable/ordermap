//! Enables serde serialization support for `OrderMap`
use std::fmt::{self, Formatter};
use std::hash::{Hash, BuildHasher};
use std::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{SerializeMap, Serializer, Serialize};

use super::OrderMap;

struct OrderMapVisitor<K, V, S>(PhantomData<OrderMap<K, V, S>>);

impl<'de, K, V, S> Visitor<'de> for OrderMapVisitor<K, V, S>
    where K: Hash, K: Eq, K: Deserialize<'de>, V: Deserialize<'de>, S: Default, S: BuildHasher {
    type Value = OrderMap<K, V, S>;
    #[inline]
    fn expecting(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("an OrderMap")
    }
    #[inline]
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error> where M: MapAccess<'de> {
        let mut result = OrderMap::with_capacity_and_hasher(access.size_hint().unwrap_or(0), Default::default());
        while let Some((key, value)) = access.next_entry()? {
            result.insert(key, value);
        }
        Ok(result)
    }
}
impl<'de, K, V, S> Deserialize<'de> for OrderMap<K, V, S>
    where K: Hash, K: Eq, K: Deserialize<'de>, V: Deserialize<'de>, S: Default, S: BuildHasher {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(OrderMapVisitor(PhantomData))
    }
}
impl<K, V, S> Serialize for OrderMap<K, V, S> 
    where K: Eq, K: Hash, K: Serialize, V: Serialize, S: BuildHasher {
    #[inline]
    fn serialize<T: Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }   
}
