use terminus_store::*;
use terminus_store::store::*;
use futures::prelude::*;
use futures::future;


fn do_things(store: &Store) -> impl Future<Item=future::Loop<(),()>, Error=()>+Send {
    store.open("asdf")
        .and_then(|g| g.unwrap().head())
        .map(|_|future::Loop::Continue(()))
        .map_err(|_|())
}

fn main() {
    let store = open_directory_store("/tmp/foo");
    tokio::run(future::loop_fn((), move |_|do_things(&store)));
}
