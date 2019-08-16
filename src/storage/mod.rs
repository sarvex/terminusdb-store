//! Ferricstore stores everything in read-only layers. A database is no more than a reference to a layer (a label).
//! A 'write' is no more than building a new layer, then altering a label to point at that new layer.

pub mod layer;
//pub mod label;
