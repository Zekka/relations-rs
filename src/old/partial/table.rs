use std::collections::HashMap;
use std::collections::hash_map;

// -- table --
// (another word might be Universe)
struct Table<A> {
    next_ix: u64,
    values: HashMap<Handle<A>, A>
}

impl <A> Table<A> {
    fn store(&mut self, a: A) -> Handle<A> {
        let ix = self.next_ix;
        self.next_ix = self.next_ix + 1;
        let key = Handle{phantom: ::std::marker::PhantomData, ix: ix};
        self.values.insert(key, a);
        key
    }

    fn seek<'a>(&'a self, handle: &Handle<A>) -> Option<&'a A> {
        self.values.get(handle)
    }

    fn seek_mut<'a>(&'a mut self, handle: &Handle<A>) -> Option<&'a mut A> {
        self.values.get_mut(handle)
    }

    fn iter_mut(&mut self) -> hash_map::IterMut<Handle<A>, A> {
        self.values.iter_mut()
    }
}

// -- handle --
struct Handle<A> {
    phantom: ::std::marker::PhantomData<A>,
    ix: u64
}

impl<A> PartialEq for Handle<A> {
    fn eq(&self, other: &Handle<A>) -> bool {
        self.ix == other.ix
    }
}

impl<A> Eq for Handle<A> { }
impl<A> ::std::hash::Hash for Handle<A> { 
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        self.ix.hash(state);
    }
}

impl<A> Clone for Handle<A> {
    fn clone(&self) -> Handle<A> {
        Handle{phantom: ::std::marker::PhantomData, ix: self.ix}
    }
}

impl<A> Copy for Handle<A> {

}