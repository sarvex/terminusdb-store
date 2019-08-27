//! Ferricstore stores data in layers. The base layer consists of a
//! dictionary, a set of triples, and indexes over these data
//! structures. A child layer consists of an optional dictionary
//! expansion, a set of triple additions, and a set of triple
//! deletions, as well as indexes.
//!
//! Layers are stored in a top-level directory containing a bunch of
//! subdirectories. Each layer is identified by an UUID, the string
//! representation of which is also the name of the
//! subdirectory. Layers contain a metadata file specifying their
//! parent (if any), or their logically equivalent replacement (if
//! any). Existence of this metadata file also signifies that a layer
//! is 'complete' (as in, it's not halfway constructed and nobody is
//! going to be modifying it anymore, except to specify a
//! replacement).
//!
//! A layer can be replaced by a logically equivalent layer. This
//! allows a background process to merge a couple of smaller layers
//! into one large layer.
use crate::structure::storage::*;
use std::path::PathBuf;
use yaml_rust::{YamlLoader, Yaml};
use yaml_rust::yaml::Hash;
use uuid::Uuid;

struct LayerStorer<MFS:MultiFileStore> {
    store: MFS
}

struct StoredLayer<MFS:MultiFileStore> {
    store: MFS
}

pub struct LayerMetadata {
    yaml: Hash
}

impl LayerMetadata {
    pub fn parent_name(&self) -> Option<Uuid> {
        self.yaml.get(&Yaml::from_str("parent"))
            .map(|y| Uuid::parse_str(y.as_str().unwrap()).expect("expected parent to be a proper uuid"))
    }

    pub fn layer_type(&self) -> LayerType {
        match self.parent_name().is_some() {
            true => LayerType::Child,
            false => LayerType::Base
        }
    }
}

impl<MFS:MultiFileStore> StoredLayer<MFS> {
    pub fn new(store: MFS) -> StoredLayer<MFS> {
        StoredLayer { store }
    }
}

impl StoredLayer<FileBackedMultiFileStore> {
    pub fn from_path<P:Into<PathBuf>>(path: P) -> StoredLayer<FileBackedMultiFileStore> {
        Self::new(FileBackedMultiFileStore::new(path))
    }
}

pub enum LayerType {
    Base,
    Child
}

pub struct Layer<M:AsRef<[u8]>> {
    layer_type: LayerType,
    metadata: LayerMetadata,
    base_triple_file: Option<M>,
    pos_triple_file: Option<M>,
    neg_triple_file: Option<M>,
    dictionary_file: Option<M>
}

impl<M:AsRef<[u8]>> Layer<M> {
    pub fn parent(&self) -> Option<Uuid> {
        self.metadata.parent_name()
    }
}

pub trait LayerStore {
    type Memory: AsRef<[u8]>;
    
    fn layer_names(&self) -> Vec<Uuid>;
    fn get_layer(&self, name: &Uuid) -> Option<Layer<Self::Memory>>;
    fn layer_children(&self, name: &Uuid) -> Vec<Uuid> {
        let mut result = Vec::new();
        for possible_child in self.layer_names().iter() {
            let layer = self.get_layer(possible_child).unwrap();
            if layer.parent() == Some(*name) {
                result.push(possible_child.clone());
            }
        }

        result
    }
}

enum StringOrigin {
    Node, Predicate, Value
}

struct DbString {
    origin: StringOrigin,
    id: u64,
    value: String
}



/*
pub struct Layer {
    fn parent(&self) -> Option<Self>;

    fn layer_type(&self) -> LayerType {
        if self.parent().is_some() {
            LayerType::Child
        }
        else {
            LayerType::Base
        }
    }

    fn node_id(&self, node: &str) -> Option<u64>;
    fn predicate_id(&self, predicate: &str) -> Option<u64>;
    fn value_id(&self, value: &str) -> Option<u64>;

    fn id_string(&self, id: u64) -> Option<DbString>;

    fn triples_by_subject(&self, id: u64);
}
*/
