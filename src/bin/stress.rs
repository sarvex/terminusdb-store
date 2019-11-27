use terminus_store::*;
pub fn main() {
    loop {
        let store = open_sync_directory_store("/tmp/foo");
        let graph = store.open("asdf").unwrap().unwrap();
        let layer = graph.head();
    }
}
