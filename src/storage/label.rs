use futures::prelude::*;

#[derive(Clone,PartialEq,Eq,Debug)]
pub struct Label {
    pub name: String,
    pub layer: Option<[u32;5]>,
    pub version: u64
}

impl Label {
    pub fn new_empty(name: &str) -> Label {
        Label {
            name: name.to_owned(),
            layer: None,
            version: 0
        }
    }
    pub fn new(name: &str, layer: [u32;5]) -> Label {
        Label {
            name: name.to_owned(),
            layer: Some(layer),
            version: 0
        }
    }

    pub fn updated(previous: &Label, layer: Option<[u32;5]>) -> Label {
        Label {
            name: previous.name.clone(),
            layer,
            version: previous.version+1
        }
    }
}

pub trait LabelStore: Send+Sync {
    fn labels(&self) -> Box<dyn Future<Item=Vec<String>,Error=std::io::Error>+Send>;
    fn create_label(&self, name: &str) -> Box<dyn Future<Item=Label, Error=std::io::Error>+Send>;
    fn get_labels(&self, names: Vec<String>) -> Box<dyn Future<Item=Option<Vec<Label>>,Error=std::io::Error>+Send>;
    fn set_labels(&self, labels: Vec<Label>) -> Box<dyn Future<Item=bool,Error=std::io::Error>+Send>;

    fn get_label(&self, name: &str) -> Box<dyn Future<Item=Option<Label>, Error=std::io::Error>+Send> {
        Box::new(self.get_labels(vec![name.to_owned()])
                 .map(|l|l.map(|mut l|l.pop().unwrap())))
    }

    fn set_label(&self, label: Label) -> Box<dyn Future<Item=bool,Error=std::io::Error>+Send> {
        self.set_labels(vec![label])
    }
}
