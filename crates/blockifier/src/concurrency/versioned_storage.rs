use std::collections::{BTreeMap, HashMap};

use crate::state::state_api::StateResult;

pub(crate) type CellId = u64;
pub(crate) type Version = u64;

#[cfg(test)]
#[path = "versioned_storage_test.rs"]
pub mod test;

type ReadCallback<K, V> = dyn Fn(K) -> StateResult<V>;

pub struct VersionedStorage<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    pub base_value_read_callback: Box<ReadCallback<K, V>>,
    pub writes: HashMap<K, BTreeMap<Version, V>>,
}

impl<K, V> VersionedStorage<K, V>
where
    K: Eq + std::hash::Hash,
    V: Clone,
{
    pub fn new(base_value_read_callback: Box<ReadCallback<K, V>>) -> Self {
        VersionedStorage { base_value_read_callback, writes: HashMap::new() }
    }

    pub fn read(&mut self, cell_id: K, version: Version) -> V {
        match self.writes[&cell_id].range(..=version).next_back() {
            Some((_, value)) => value.clone(),
            None => {
                let base_value = (self.base_value_read_callback)(cell_id);
                let base_value = base_value.expect("Base value read callback returned an error");
                base_value
            }
        }
    }

    pub fn write(&mut self, cell_id: K, version: Version, value: V) {
        let writes_map = self.writes.entry(cell_id).or_insert_with(BTreeMap::new);
        writes_map.insert(version, value);
    }
}
