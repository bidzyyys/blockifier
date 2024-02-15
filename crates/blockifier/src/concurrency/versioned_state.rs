use std::any::Any;

use starknet_api::core::{ClassHash, CompiledClassHash, ContractAddress, Nonce};
use starknet_api::hash::StarkFelt;
use starknet_api::state::StorageKey;

use super::versioned_cell::VersionId;
use super::versioned_storage::VersionedStorage;
use crate::state::cached_state::CachedState;
use crate::state::state_api::{State, StateResult};
use crate::test_utils::dict_state_reader::DictStateReader;

#[cfg(test)]
#[path = "versioned_state_test.rs"]
pub mod test;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum StorageType {
    ContractStorage,
    ClassHash,
    Nonce,
    CompiledClassHash,
}

pub struct VersionedState {
    /// A collection of versioned storages.
    // pub state: CachedState<DictStateReader>,
    // pub versioned_storage: HashMap<StorageType, Box<VersionedStorage<K, V>>>,
    pub contract_storage_versioned_storage:
        VersionedStorage<(ContractAddress, StorageKey), StarkFelt>,
    pub class_hash_versioned_storage: VersionedStorage<ContractAddress, ClassHash>,
    pub nonce_versioned_storage: VersionedStorage<ContractAddress, Nonce>,
    pub compiled_class_hash_versioned_storage: VersionedStorage<ClassHash, CompiledClassHash>,
}

impl VersionedState {
    pub fn new<'a>(mut state: &'a CachedState<DictStateReader>) -> Self {
        // initial the hashmap (versioned_storage) with the functions from state
        let get_nonce = |address: ContractAddress| -> StateResult<Nonce> {
            state.get_nonce_initial_value(address)
        };

        let get_compiled_class_hash = |class_hash: ClassHash| -> StateResult<CompiledClassHash> {
            state.get_compiled_class_hash_initial_value(class_hash)
        };
        let get_class_hash = |address: ContractAddress| -> StateResult<ClassHash> {
            state.get_class_hash_initial_value(address)
        };
        let get_storage =
            |address_key_pair: (ContractAddress, StorageKey)| -> StateResult<StarkFelt> {
                state.get_storage_initial_value(address_key_pair.0, address_key_pair.1)
            };

        let contract_storage_versioned_storage = VersionedStorage::new(Box::new(get_storage));
        let class_hash_versioned_storage = VersionedStorage::new(Box::new(get_class_hash));
        let nonce_versioned_storage = VersionedStorage::new(Box::new(get_nonce));
        let compiled_class_hash_versioned_storage =
            VersionedStorage::new(Box::new(get_compiled_class_hash));

        VersionedState {
            contract_storage_versioned_storage,
            class_hash_versioned_storage,
            nonce_versioned_storage,
            compiled_class_hash_versioned_storage,
        }
    }

    // fn get_storage_version(&mut self, storage_type: &StorageType) {
    //     match storage_type {
    //         StorageType::ContractStorage => Box::new(self.contract_storage_versioned_storage),
    //         StorageType::ClassHash => Box::new(self.class_hash_versioned_storage),
    //         StorageType::Nonce => Box::new(self.nonce_versioned_storage),
    //         StorageType::CompiledClassHash =>
    // Box::new(self.compiled_class_hash_versioned_storage),     }
    // }

    pub fn read(
        &mut self,
        storage_type: StorageType,
        cell_id: Box<dyn Any>,
        version: VersionId,
    ) -> &dyn Any {
        match storage_type {
            StorageType::ContractStorage => {
                if let Some(cell_id) = cell_id.downcast_ref::<(ContractAddress, StorageKey)>() {
                    &self.contract_storage_versioned_storage.read(*cell_id, version) as &dyn Any
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.contract_storage_versioned_storage.read(cell_id, version)
            }
            StorageType::ClassHash => {
                if let Some(cell_id) = cell_id.downcast_ref::<ContractAddress>() {
                    &self.class_hash_versioned_storage.read(*cell_id, version) as &dyn Any
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.class_hash_versioned_storage.read(cell_id, version)
            }
            StorageType::Nonce => {
                if let Some(cell_id) = cell_id.downcast_ref::<ContractAddress>() {
                    &self.nonce_versioned_storage.read(*cell_id, version) as &dyn Any
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.nonce_versioned_storage.read(cell_id, version)
            }
            StorageType::CompiledClassHash => {
                if let Some(cell_id) = cell_id.downcast_ref::<ClassHash>() {
                    &self.compiled_class_hash_versioned_storage.read(*cell_id, version) as &dyn Any
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.compiled_class_hash_versioned_storage.read(cell_id, version)
            }
        }
    }
    //     let mut versioned_storage = self.get_storage_version(&storage_type);
    //     versioned_storage.read(cell_id, version)
    // }

    pub fn write(
        &mut self,
        storage_type: StorageType,
        cell_id: Box<dyn Any>,
        key_id: Box<dyn Any>,
        version: VersionId,
        value: Box<dyn Any>,
    ) {
        match storage_type {
            StorageType::ContractStorage => {
                if let Some(cell_id) = cell_id.downcast_ref::<ContractAddress>() {
                    if let Some(key_id) = key_id.downcast_ref::<StorageKey>() {
                        if let Some(value) = value.downcast_ref::<StarkFelt>() {
                            self.contract_storage_versioned_storage.write(
                                (*cell_id, *key_id),
                                version,
                                *value,
                            )
                        } else {
                            // Handle the case when value is not of the expected type
                            unimplemented!()
                        }
                    } else {
                        // Handle the case when cell_id is not of the expected type
                        unimplemented!()
                    }
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.contract_storage_versioned_storage.read(cell_id, version)
            }
            StorageType::ClassHash => {
                if let Some(cell_id) = cell_id.downcast_ref::<ContractAddress>() {
                    if let Some(value) = value.downcast_ref::<ClassHash>() {
                        self.class_hash_versioned_storage.write(*cell_id, version, *value)
                    } else {
                        // Handle the case when value is not of the expected type
                        unimplemented!()
                    }
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.class_hash_versioned_storage.read(cell_id, version)
            }
            StorageType::Nonce => {
                if let Some(cell_id) = cell_id.downcast_ref::<ContractAddress>() {
                    if let Some(value) = value.downcast_ref::<Nonce>() {
                        self.nonce_versioned_storage.write(*cell_id, version, *value)
                    } else {
                        // Handle the case when value is not of the expected type
                        unimplemented!()
                    }
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.nonce_versioned_storage.read(cell_id, version)
            }
            StorageType::CompiledClassHash => {
                if let Some(cell_id) = cell_id.downcast_ref::<ClassHash>() {
                    if let Some(value) = value.downcast_ref::<CompiledClassHash>() {
                        self.compiled_class_hash_versioned_storage.write(*cell_id, version, *value)
                    } else {
                        // Handle the case when value is not of the expected type
                        unimplemented!()
                    }
                } else {
                    // Handle the case when cell_id is not of the expected type
                    unimplemented!()
                }
                // self.compiled_class_hash_versioned_storage.read(cell_id, version)
            }
        }

        // let mut versioned_storage = self.get_storage_version(&storage_type);
        // versioned_storage.write(cell_id, version, value)
    }
}
