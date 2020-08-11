//! Rollup layer implementation
//!
//! A rollup layer replaces a stack of layers with one consolidated layer.

use super::layer::*;
use crate::structure::*;

pub struct RollupLayer {
    name: [u32; 5],
    original: [u32; 5],
    node_value_remap_table: Option<IdRemap>,
    predicate_remap_table: Option<IdRemap>,
    inner_layer: Box<dyn Layer>
}

impl Layer for RollupLayer {
    fn name(&self) -> [u32; 5] {
        self.original
    }
    fn names(&self) -> Vec<[u32; 5]> {
        self.inner_layer.names()
    }
    fn parent(&self) -> Option<&dyn Layer> {
        self.inner_layer.parent()
    }
    fn node_and_value_count(&self) -> usize {
        self.inner_layer.node_and_value_count()
    }
    fn predicate_count(&self) -> usize {
        self.inner_layer.predicate_count()
    }
    fn predicate_dict_get(&self, id: usize) -> Option<String> {
        self.inner_layer.predicate_dict_get(id)
    }
    fn predicate_dict_len(&self) -> usize {
        self.inner_layer.predicate_dict_len()
    }
    fn predicate_dict_id(&self, predicate: &str) -> Option<u64> {
        self.inner_layer.predicate_dict_id(predicate)
    }
    fn node_dict_id(&self, subject: &str) -> Option<u64> {
        self.inner_layer.node_dict_id(subject)
    }
    fn node_dict_get(&self, id: usize) -> Option<String> {
        self.inner_layer.node_dict_get(id)
    }
    fn node_dict_len(&self) -> usize {
        self.inner_layer.node_dict_len()
    }
    fn value_dict_id(&self, value: &str) -> Option<u64> {
        self.inner_layer.value_dict_id(value)
    }
    fn value_dict_len(&self) -> usize {
        self.inner_layer.value_dict_len()
    }
    fn value_dict_get(&self, id: usize) -> Option<String> {
        self.inner_layer.value_dict_get(id)
    }
    fn subject_id(&self, subject: &str) -> Option<u64> {
        self.inner_layer.subject_id(subject)
    }
    fn predicate_id(&self, predicate: &str) -> Option<u64> {
        self.inner_layer.predicate_id(predicate)
    }
    fn object_node_id(&self, object: &str) -> Option<u64> {
        self.inner_layer.object_node_id(object)
    }
    fn object_value_id(&self, object: &str) -> Option<u64> {
        self.inner_layer.object_value_id(object)
    }
    fn id_subject(&self, id: u64) -> Option<String> {
        self.inner_layer.id_subject(id)
    }
    fn id_predicate(&self, id: u64) -> Option<String> {
        self.inner_layer.id_predicate(id)
    }
    fn id_object(&self, id: u64) -> Option<ObjectType> {
        self.inner_layer.id_object(id)
    }
    fn subject_additions(&self) -> Box<dyn Iterator<Item = Box<dyn LayerSubjectLookup>>> {
        self.inner_layer.subject_additions()
    }
    fn subject_removals(&self) -> Box<dyn Iterator<Item = Box<dyn LayerSubjectLookup>>> {
        self.inner_layer.subject_removals()
    }
    fn lookup_subject_addition(&self, subject: u64) -> Option<Box<dyn LayerSubjectLookup>> {
        self.inner_layer.lookup_subject_addition(subject)
    }
    fn lookup_subject_removal(&self, subject: u64) -> Option<Box<dyn LayerSubjectLookup>> {
        self.inner_layer.lookup_subject_removal(subject)
    }
    fn object_additions(&self) -> Box<dyn Iterator<Item = Box<dyn LayerObjectLookup>>> {
        self.inner_layer.object_additions()
    }
    fn object_removals(&self) -> Box<dyn Iterator<Item = Box<dyn LayerObjectLookup>>> {
        self.inner_layer.object_removals()
    }
    fn lookup_object_addition(&self, object: u64) -> Option<Box<dyn LayerObjectLookup>> {
        self.inner_layer.lookup_object_addition(object)
    }
    fn lookup_object_removal(&self, object: u64) -> Option<Box<dyn LayerObjectLookup>> {
        self.inner_layer.lookup_object_removal(object)
    }
    fn lookup_predicate_addition(&self, predicate: u64) -> Option<Box<dyn LayerPredicateLookup>> {
        self.inner_layer.lookup_predicate_addition(predicate)
    }
    fn lookup_predicate_removal(&self, predicate: u64) -> Option<Box<dyn LayerPredicateLookup>> {
        self.inner_layer.lookup_predicate_removal(predicate)
    }
    fn clone_boxed(&self) -> Box<dyn Layer> {
        Box::new(Self {
            name: self.name,
            original: self.original,
            node_value_remap_table: self.node_value_remap_table.clone(),
            predicate_remap_table: self.predicate_remap_table.clone(),
            inner_layer: self.inner_layer.clone_boxed()
        })
    }
    fn triple_layer_addition_count(&self) -> usize {
        self.inner_layer.triple_layer_addition_count()
    }
    fn triple_layer_removal_count(&self) -> usize {
        self.inner_layer.triple_layer_removal_count()
    }
}
