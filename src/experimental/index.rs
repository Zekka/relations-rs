use experimental::handle::Handle;

use std::any::Any;

pub(crate) trait Indexer<A>: Any where A: 'static {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;

    fn handle_update(&mut self, Handle<A>, &A);
    fn handle_free(&mut self, Handle<A>, &A);
}