use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use std::fmt;

// TODO: Write my own better Debug
pub struct Handle<A> {
    pub(crate) phantom: PhantomData<A>,
    pub(crate) ix: u64
}

impl<A> PartialEq for Handle<A> {
    fn eq(&self, other: &Handle<A>) -> bool {
        self.ix == other.ix
    }
}

impl<A> Eq for Handle<A> { }
impl<A> Hash for Handle<A> { 
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ix.hash(state);
    }
}

impl<A> Clone for Handle<A> {
    fn clone(&self) -> Handle<A> {
        Handle{phantom: PhantomData, ix: self.ix}
    }
}

impl<A> Copy for Handle<A> {

}

impl<A> fmt::Debug for Handle<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Handle {{ {} }}", self.ix)
    }
}