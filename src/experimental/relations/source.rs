use std::marker::PhantomData;
use std::collections::HashMap;
use std::collections::HashSet;
use experimental::handle::Handle;

// == Invariants ==
// If item X is present in a relation and item X was freed, then X is in the free list.
// If item X was freed and we don't know if it was present in a relation, then X might not be in the free list.
// So, if you just got an item out of a relation and you want to know if it was freed or not, use in_freelist.
// If you just got an item from the user, you have basically no idea what the user did, so use in_store.
pub(crate) trait SourceV<L, R>: Sized {
    fn in_freelist_l(&self, Handle<L>) -> bool;
    fn in_freelist_r(&self, Handle<R>) -> bool;

    fn in_store_l(&self, Handle<L>) -> bool;
    fn in_store_r(&self, Handle<R>) -> bool;

    fn is_free_l(&self, h: Handle<L>) -> bool { !self.in_store_l(h) }
    fn is_free_r(&self, h: Handle<R>) -> bool { !self.in_store_r(h) }
}

pub(crate) trait SourceM<L, R>: SourceV<L, R> {
    fn do_frees<TL, TR>(&mut self, &mut HashMap<Handle<L>, TR>, &mut HashMap<Handle<R>, TL>, fn(TR, &mut HashMap<Handle<R>, TL>), fn(TL, &mut HashMap<Handle<L>, TR>));
}

pub(crate) struct JoinV<'j, L, R> where L: 'j, R: 'j {
    pub(crate) l_type: PhantomData<L>,
    pub(crate) r_type: PhantomData<R>,

    pub(crate) l_freelist: &'j HashSet<Handle<L>>,
    pub(crate) r_freelist: &'j HashSet<Handle<R>>,

    // TODO: Make this a ref, store it inside the Table
    pub(crate) l_instore: Box<Fn(Handle<L>) -> bool + 'j>,
    pub(crate) r_instore: Box<Fn(Handle<R>) -> bool + 'j>,
}

pub(crate) struct SelfJoinV<'j, S> where S: 'j {
    pub(crate) self_type: PhantomData<S>,

    pub(crate) self_freelist: &'j HashSet<Handle<S>>,
    pub(crate) self_instore: Box<Fn(Handle<S>) -> bool + 'j>,
}

pub(crate) struct JoinM<'j, L, R> where L: 'j, R: 'j {
    pub(crate) l_type: PhantomData<L>,
    pub(crate) r_type: PhantomData<R>,

    pub(crate) l_freelist: &'j mut HashSet<Handle<L>>,
    pub(crate) r_freelist: &'j mut HashSet<Handle<R>>,

    // TODO: Make this a ref, store it inside the Table
    pub(crate) l_instore: Box<Fn(Handle<L>) -> bool + 'j>,
    pub(crate) r_instore: Box<Fn(Handle<R>) -> bool + 'j>,
}

pub(crate) struct SelfJoinM<'j, S> where S: 'j {
    pub(crate) self_type: PhantomData<S>,

    pub(crate) self_freelist: &'j mut HashSet<Handle<S>>,
    pub(crate) self_instore: Box<Fn(Handle<S>) -> bool + 'j>,
}

impl<'j, L, R> SourceV<L, R> for JoinV<'j, L, R> {
    fn in_freelist_l(&self, h: Handle<L>) -> bool {
        self.l_freelist.contains(&h)
    }
    fn in_freelist_r(&self, h :Handle<R>) -> bool {
        self.r_freelist.contains(&h)
    }

    fn in_store_l(&self, h: Handle<L>) -> bool {
        (self.l_instore)(h)
    }
    fn in_store_r(&self, h: Handle<R>) -> bool {
        (self.r_instore)(h)
    }
}

impl<'j, S> SourceV<S, S> for SelfJoinV<'j, S> {
    fn in_freelist_l(&self, h: Handle<S>) -> bool {
        self.self_freelist.contains(&h)
    }
    fn in_freelist_r(&self, h :Handle<S>) -> bool {
        self.self_freelist.contains(&h)
    }

    fn in_store_l(&self, h: Handle<S>) -> bool {
        (self.self_instore)(h)
    }
    fn in_store_r(&self, h: Handle<S>) -> bool {
        (self.self_instore)(h)
    }
}

impl<'j, L, R> SourceV<L, R> for JoinM<'j, L, R> {
    fn in_freelist_l(&self, h: Handle<L>) -> bool {
        self.l_freelist.contains(&h)
    }
    fn in_freelist_r(&self, h :Handle<R>) -> bool {
        self.r_freelist.contains(&h)
    }

    fn in_store_l(&self, h: Handle<L>) -> bool {
        (self.l_instore)(h)
    }
    fn in_store_r(&self, h: Handle<R>) -> bool {
        (self.r_instore)(h)
    }
}

impl<'j, S> SourceV<S, S> for SelfJoinM<'j, S> {
    fn in_freelist_l(&self, h: Handle<S>) -> bool {
        self.self_freelist.contains(&h)
    }
    fn in_freelist_r(&self, h :Handle<S>) -> bool {
        self.self_freelist.contains(&h)
    }

    fn in_store_l(&self, h: Handle<S>) -> bool {
        (self.self_instore)(h)
    }
    fn in_store_r(&self, h: Handle<S>) -> bool {
        (self.self_instore)(h)
    }
}

impl<'j, L, R> SourceM<L, R> for JoinM<'j, L, R> {
    fn do_frees<TL, TR>(&mut self, ls: &mut HashMap<Handle<L>, TR>, rs: &mut HashMap<Handle<R>, TL>, f1: fn(TR, &mut HashMap<Handle<R>, TL>), f2: fn(TL, &mut HashMap<Handle<L>, TR>)) {
        for el in self.l_freelist.iter() { 
            match ls.remove(el) {
                None => {}
                Some(rbunch) => { f1(rbunch, rs); }
            }
        }
        for er in self.r_freelist.iter() { 
            match rs.remove(er) {
                None => {}
                Some(lbunch) => { f2(lbunch, ls); }
            }
        }
        self.l_freelist.clear();
        self.r_freelist.clear();
    }
}

impl<'j, S> SourceM<S, S> for SelfJoinM<'j, S> {
    fn do_frees<TL, TR>(&mut self, ls: &mut HashMap<Handle<S>, TR>, rs: &mut HashMap<Handle<S>, TL>, f1: fn(TR, &mut HashMap<Handle<S>, TL>), f2: fn(TL, &mut HashMap<Handle<S>, TR>)) {
        for e in self.self_freelist.iter() { 
            match ls.remove(e) {
                None => {}
                Some(rbunch) => { f1(rbunch, rs); }
            }
            match rs.remove(e) {
                None => {}
                Some(lbunch) => { f2(lbunch, ls); }
            }
        }
        self.self_freelist.clear();
    }
}