use crate::structure::storage::*;
use std::path::PathBuf;

struct LayerStorer<MFS:MultiFileStore> {
    store: MFS
}

struct StoredLayer<MFS:MultiFileStore> {
    store: MFS
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

enum LayerType {
    Base,
    Child
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
