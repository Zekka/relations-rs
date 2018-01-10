use experimental::index::Indexer;
use experimental::handle::Handle;

pub struct Query<A> where {
    pub(crate) body: for<'a> fn(ix: usize, &'a Vec<Box<Indexer<A>>>) -> Box<Iterator<Item=Handle<A>> + 'a>,
    pub(crate) ix: usize
}

impl<A> Query<A> {
    pub(crate) fn run<'a> (&self, ixers: &'a Vec<Box<Indexer<A>>>) -> Box<Iterator<Item=Handle<A>> + 'a> {
        (self.body)(self.ix, ixers)
    }
}