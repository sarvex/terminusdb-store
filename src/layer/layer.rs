use super::base::*;
use super::child::*;
use crate::structure::storage::*;

pub trait Layer {
    type PredicateObjectPairsForSubject: PredicateObjectPairsForSubject;
    type SubjectIterator: 'static+Iterator<Item=Self::PredicateObjectPairsForSubject>;

    fn node_and_value_count(&self) -> usize;
    fn predicate_count(&self) -> usize;

    fn subject_id(&self, subject: &str) -> Option<u64>;
    fn predicate_id(&self, predicate: &str) -> Option<u64>;
    fn object_node_id(&self, object: &str) -> Option<u64>;
    fn object_value_id(&self, object: &str) -> Option<u64>;
    fn id_subject(&self, id: u64) -> Option<String>;
    fn id_predicate(&self, id: u64) -> Option<String>;
    fn id_object(&self, id: u64) -> Option<ObjectType>;

    fn subjects(&self) -> Self::SubjectIterator;
    fn predicate_object_pairs_for_subject(&self, subject: u64) -> Option<Self::PredicateObjectPairsForSubject>;
    
    fn triple_exists(&self, subject: u64, predicate: u64, object: u64) -> bool {
        self.predicate_object_pairs_for_subject(subject)
            .and_then(|pairs| pairs.objects_for_predicate(predicate))
            .and_then(|objects| objects.triple(object))
            .is_some()
    }

    fn triples(&self) -> Box<dyn Iterator<Item=IdTriple>> {
        Box::new(self.subjects().map(|s|s.predicates()).flatten()
                 .map(|p|p.triples()).flatten())
    }
}

#[derive(Clone)]
pub enum ParentLayer<M:'static+AsRef<[u8]>+Clone> {
    Base(BaseLayer<M>),
    Child(ChildLayer<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> Layer for ParentLayer<M> {
    type PredicateObjectPairsForSubject = ParentPredicateObjectPairsForSubject<M>;
    type SubjectIterator = ParentSubjectIterator<M>;

    fn node_and_value_count(&self) -> usize {
        match self {
            Self::Base(b) => b.node_and_value_count(),
            Self::Child(c) => c.node_and_value_count()
        }
    }

    fn predicate_count(&self) -> usize {
        match self {
            Self::Base(b) => b.predicate_count(),
            Self::Child(c) => c.predicate_count()
        }
    }

    fn subject_id(&self, subject: &str) -> Option<u64> {
        match self {
            Self::Base(b) => b.subject_id(subject),
            Self::Child(c) => c.subject_id(subject)
        }
    }

    fn predicate_id(&self, predicate: &str) -> Option<u64> {
        match self {
            Self::Base(b) => b.predicate_id(predicate),
            Self::Child(c) => c.predicate_id(predicate)
        }
    }

    fn object_node_id(&self, node: &str) -> Option<u64> {
        match self {
            Self::Base(b) => b.object_node_id(node),
            Self::Child(c) => c.object_node_id(node)
        }
    }

    fn object_value_id(&self, value: &str) -> Option<u64> {
        match self {
            Self::Base(b) => b.object_value_id(value),
            Self::Child(c) => c.object_value_id(value)
        }
    }

    fn id_subject(&self, id: u64) -> Option<String> {
        match self {
            Self::Base(b) => b.id_subject(id),
            Self::Child(c) => c.id_subject(id)
        }
    }

    fn id_predicate(&self, id: u64) -> Option<String> {
        match self {
            Self::Base(b) => b.id_predicate(id),
            Self::Child(c) => c.id_predicate(id)
        }
    }

    fn id_object(&self, id: u64) -> Option<ObjectType> {
        match self {
            Self::Base(b) => b.id_object(id),
            Self::Child(c) => c.id_object(id)
        }
    }

    fn subjects(&self) -> ParentSubjectIterator<M> {
        match self {
            Self::Base(b) => ParentSubjectIterator::Base(b.subjects()),
            Self::Child(c) => ParentSubjectIterator::Child(c.subjects())
        }
    }

    fn predicate_object_pairs_for_subject(&self, subject: u64) -> Option<ParentPredicateObjectPairsForSubject<M>> {
        match self {
            Self::Base(b) => b.predicate_object_pairs_for_subject(subject).map(|b| ParentPredicateObjectPairsForSubject::Base(b)),
            Self::Child(c) => c.predicate_object_pairs_for_subject(subject).map(|c| ParentPredicateObjectPairsForSubject::Child(c)),
        }
    }
}

#[derive(Clone)]
pub enum ParentSubjectIterator<M:'static+AsRef<[u8]>+Clone> {
    Base(BaseSubjectIterator<M>),
    Child(ChildSubjectIterator<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> Iterator for ParentSubjectIterator<M> {
    type Item = ParentPredicateObjectPairsForSubject<M>;
    
    fn next(&mut self) -> Option<ParentPredicateObjectPairsForSubject<M>> {
        match self {
            Self::Base(b) => b.next().map(|b|ParentPredicateObjectPairsForSubject::Base(b)),
            Self::Child(c) => c.next().map(|c|ParentPredicateObjectPairsForSubject::Child(c))
        }
    }
}

pub trait PredicateObjectPairsForSubject {
    type Objects: ObjectsForSubjectPredicatePair;
    type PredicateIterator: 'static+Iterator<Item=Self::Objects>;

    fn subject(&self) -> u64;

    fn predicates(&self) -> Self::PredicateIterator;
    fn objects_for_predicate(&self, predicate: u64) -> Option<Self::Objects>;

    fn triples(&self) -> Box<dyn Iterator<Item=IdTriple>> {
        Box::new(self.predicates().map(|p|p.triples()).flatten())
    }
}

pub trait ObjectsForSubjectPredicatePair {
    type ObjectIterator: Iterator<Item=IdTriple>;

    fn subject(&self) -> u64;
    fn predicate(&self) -> u64;

    fn triples(&self) -> Self::ObjectIterator;
    fn triple(&self, object: u64) -> Option<IdTriple>;
}

#[derive(Clone,Copy,PartialEq,PartialOrd)]
pub struct IdTriple {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64
}

#[derive(Clone)]
pub enum ParentPredicateObjectPairsForSubject<M:'static+AsRef<[u8]>+Clone> {
    Base(BasePredicateObjectPairsForSubject<M>),
    Child(ChildPredicateObjectPairsForSubject<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> PredicateObjectPairsForSubject for ParentPredicateObjectPairsForSubject<M> {
    type Objects = ParentObjectsForSubjectPredicatePair<M>;
    type PredicateIterator = ParentPredicateIterator<M>;

    fn subject(&self) -> u64 {
        match self {
            Self::Base(b) => b.subject(),
            Self::Child(c) => c.subject(),
        }
    }

    fn predicates(&self) -> ParentPredicateIterator<M> {
        match self {
            Self::Base(b) => ParentPredicateIterator::Base(b.predicates()),
            Self::Child(c) => ParentPredicateIterator::Child(c.predicates())
        }
    }

    fn objects_for_predicate(&self, predicate: u64) -> Option<ParentObjectsForSubjectPredicatePair<M>> {
        match self {
            Self::Base(b) => b.objects_for_predicate(predicate).map(|b| ParentObjectsForSubjectPredicatePair::Base(b)),
            Self::Child(c) => c.objects_for_predicate(predicate).map(|c| ParentObjectsForSubjectPredicatePair::Child(c)),
        }
    }
}

#[derive(Clone)]
pub enum ParentPredicateIterator<M:'static+AsRef<[u8]>+Clone> {
    Base(BasePredicateIterator<M>),
    Child(ChildPredicateIterator<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> Iterator for ParentPredicateIterator<M> {
    type Item = ParentObjectsForSubjectPredicatePair<M>;

    fn next(&mut self) -> Option<ParentObjectsForSubjectPredicatePair<M>> {
        match self {
            Self::Base(b) => b.next().map(|b|ParentObjectsForSubjectPredicatePair::Base(b)),
            Self::Child(c) => c.next().map(|c|ParentObjectsForSubjectPredicatePair::Child(c)),
        }
    }
}

#[derive(Clone)]
pub enum ParentObjectsForSubjectPredicatePair<M:'static+AsRef<[u8]>+Clone> {
    Base(BaseObjectsForSubjectPredicatePair<M>),
    Child(ChildObjectsForSubjectPredicatePair<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> ObjectsForSubjectPredicatePair for ParentObjectsForSubjectPredicatePair<M> {
    type ObjectIterator = ParentObjectIterator<M>;

    fn subject(&self) -> u64 {
        match self {
            Self::Base(b) => b.subject(),
            Self::Child(c) => c.subject()
        }
    }

    fn predicate(&self) -> u64 {
        match self {
            Self::Base(b) => b.predicate(),
            Self::Child(c) => c.predicate()
        }
    }

    fn triples(&self) -> ParentObjectIterator<M> {
        match self {
            Self::Base(b) => ParentObjectIterator::Base(b.triples()),
            Self::Child(c) => ParentObjectIterator::Child(c.triples())
        }
    }

    fn triple(&self, object: u64) -> Option<IdTriple> {
        match self {
            Self::Base(b) => b.triple(object),
            Self::Child(c) => c.triple(object)
        }
    }
}

#[derive(Clone)]
pub enum ParentObjectIterator<M:'static+AsRef<[u8]>+Clone> {
    Base(BaseObjectIterator<M>),
    Child(ChildObjectIterator<M>)
}

impl<M:'static+AsRef<[u8]>+Clone> Iterator for ParentObjectIterator<M> {
    type Item = IdTriple;

    fn next(&mut self) -> Option<IdTriple> {
        match self {
            Self::Base(b) => b.next(),
            Self::Child(c) => c.next()
        }
    }
}

pub struct DictionaryFiles<F:'static+FileLoad+FileStore> {
    pub blocks_file: F,
    pub offsets_file: F
}

pub struct AdjacencyListFiles<F:'static+FileLoad+FileStore> {
    pub bits_file: F,
    pub blocks_file: F,
    pub sblocks_file: F,
    pub nums_file: F,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Node(String),
    Value(String)
}