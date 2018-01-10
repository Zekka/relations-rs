use std::marker::PhantomData;

use experimental::handle::Handle;
use experimental::loops::BreakContinue;
use experimental::relations::onetoone::interfaces::*;
use experimental::relations::source::*;
use experimental::table::Table;

use std::collections::HashMap;
use std::ops::DerefMut;

pub(crate) struct Abs<TFrom, TTo> {
    pub(crate) ix_from: usize,
    pub(crate) ix_to: usize,
    pub(crate) from_2_to: HashMap<Handle<TFrom>, Handle<TTo>>,
    pub(crate) to_2_from: HashMap<Handle<TTo>, Handle<TFrom>>,
}

pub(crate) struct ReiV<'r, S, TFrom, TTo> where S: SourceV<TFrom, TTo>, TFrom: 'r, TTo: 'r{
    abs: &'r Abs<TFrom, TTo>,
    source: S,
}

pub(crate) struct ReiM<'r, S, TFrom, TTo> where S: SourceM<TFrom, TTo>, TFrom: 'r, TTo: 'r{
    abs: &'r mut Abs<TFrom, TTo>,
    source: S,
}

impl<TFrom, TTo> Abstract<TFrom, TTo> for Abs<TFrom, TTo> {
    fn reify<'q>(&'q self, from: &'q Table<TFrom>, to: &'q Table<TTo>) -> Box<ReiV<'q, JoinV<TFrom, TTo>, TFrom, TTo>> {
        Box::new(ReiV{
            abs: self,
            source: JoinV{
                l_type: PhantomData,
                r_type: PhantomData,
                l_freelist: &from.relation_freelists[self.ix_from],
                r_freelist: &to.relation_freelists[self.ix_to],
                l_instore: Box::new(move |h| { from.unordered.contains_key(&h) }),
                r_instore: Box::new(move |h| { to.unordered.contains_key(&h) }),
            }
        })
    }

    fn reify_mut<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>) -> Box<ReiM<'q, JoinM<TFrom, TTo>, TFrom, TTo>> {
        let ix_from = self.ix_from;
        let ix_to = self.ix_from;
        let from_u = &from.unordered;
        let to_u = &to.unordered;
        Box::new(ReiM{
            abs: self,
            source: JoinM{
                l_type: PhantomData,
                r_type: PhantomData,
                l_freelist: &mut from.relation_freelists[ix_from],
                r_freelist: &mut to.relation_freelists[ix_to],
                l_instore: Box::new(move |h| { from_u.contains_key(&h) }),
                r_instore: Box::new(move |h| { to_u.contains_key(&h) }),
            }
        })
    }

    fn mutate_to<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        tos: Box<Iterator<Item=Handle<TTo>>+'q>, 
        cb: &Fn(Handle<TFrom>, &mut TFrom) -> BreakContinue 
    ) {
        // this is `unsafe`, but we know the keyset of the tables will never change, nor will the contents of its freelists, so reified versions should not break this
        unsafe {
            let from2 = from as *mut Table<TFrom>;
            let from3 = from as *mut Table<TFrom>;
            let to3 = to as *mut Table<TTo>;
            let reim = self.reify_mut(&mut *from3, &mut *to3);
            for to in tos {
                for from in reim.tos(to) {
                    let mut found = (*from2).seek_mut(from).unwrap();
                    let borrow = found.deref_mut();
                    cb(from, borrow);
                }
            }
        }
    }

    fn mutate_from<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        froms: Box<Iterator<Item=Handle<TFrom>>+'q>, 
        cb: &Fn(Handle<TTo>, &mut TTo) -> BreakContinue 
    ) {
        // this is `unsafe`, but we know the keyset of the tables will never change, nor will the contents of its freelists, so reified versions should not break this
        unsafe {
            let from3 = from as *mut Table<TFrom>;
            let to2 = to as *mut Table<TTo>;
            let to3 = to as *mut Table<TTo>;
            let reim = self.reify_mut(&mut *from3, &mut *to3);
            for from in froms {
                for to in reim.froms(from) {
                    let mut found = (*to2).seek_mut(to).unwrap();
                    let borrow = found.deref_mut();
                    cb(to, borrow);
                }
            }
        }
    }
}

impl <TSelf> AbstractSelf<TSelf> for Abs<TSelf, TSelf> {
    // TODO: In this case ix_from should always equal ix_to. Can I prove it?
    fn reify1<'q>(&'q self, tself: &'q Table<TSelf>) -> Box<ReiV<'q, SelfJoinV<TSelf>, TSelf, TSelf>> {
        if self.ix_from != self.ix_to { panic!("self-join with different ix_from and ix_to"); };

        Box::new(ReiV{
            abs: self,
            source: SelfJoinV{
                self_type: PhantomData,
                self_freelist: &tself.relation_freelists[self.ix_from],
                self_instore: Box::new(move |h| { tself.unordered.contains_key(&h) })
            }
        })
    }

    fn reify1_mut<'q>(&'q mut self, tself: &'q mut Table<TSelf>) -> Box<ReiM<'q, SelfJoinM<TSelf>, TSelf, TSelf>> {
        if self.ix_from != self.ix_to { panic!("self-join with different ix_from and ix_to"); };
        let ix_from = self.ix_from;
        let self_u = &tself.unordered;

        Box::new(ReiM{
            abs: self,
            source: SelfJoinM{
                self_type: PhantomData,
                self_freelist: &mut tself.relation_freelists[ix_from],
                self_instore: Box::new(move |h| { self_u.contains_key(&h) })
            }
        })
    }

    fn mutate1_to<'q>(&'q mut self, tself: &'q mut Table<TSelf>, 
        tos: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: &Fn(Handle<TSelf>, &mut TSelf) -> BreakContinue 
    ) {
        // this is `unsafe`, but we know the keyset of the table will never change, nor will the contents of its freelists, so reified versions should not break this
        unsafe {
            let tself2 = tself as *mut Table<TSelf>;
            let tself3 = tself as *mut Table<TSelf>;
            let reim = self.reify1_mut(&mut *tself3);
            for to in tos {
                for from in reim.tos(to) {
                    let mut found = (*tself2).seek_mut(from).unwrap();
                    let borrow = found.deref_mut();
                    cb(from, borrow);
                }
            }
        }
    }
    fn mutate1_from<'q>(&'q mut self, tself: &'q mut Table<TSelf>, 
        froms: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: &Fn(Handle<TSelf>, &mut TSelf) -> BreakContinue 
    ) {
        // this is `unsafe`, but we know the keyset of the table will never change, nor will the contents of its freelists, so reified versions should not break this
        unsafe {
            let tself2 = tself as *mut Table<TSelf>;
            let tself3 = tself as *mut Table<TSelf>;
            let reim = self.reify1_mut(&mut *tself3);
            for from in froms {
                for to in reim.froms(from) {
                    let mut found = (*tself2).seek_mut(to).unwrap();
                    let borrow = found.deref_mut();
                    cb(to, borrow);
                }
            }
        }
    }
}

impl<'r, S, TFrom, TTo> ReifiedView<TFrom, TTo> for ReiV<'r, S, TFrom, TTo> where S: SourceV<TFrom, TTo> {
    // these values come from the user initially
    fn has(&self, from: Handle<TFrom>, to: Handle<TTo>) -> bool {
        if self.abs.from_2_to.get(&from) != Some(&to) { return false; }
        // proven to be in the store: we can just check the freelist
        if self.source.in_freelist_l(from) { return false; }
        if self.source.in_freelist_r(to) { return false; }
        return true;
    }

    fn from(&self, from: Handle<TFrom>) -> Option<Handle<TTo>> {
        match self.abs.from_2_to.get(&from) {
            None => None,
            Some(&existing) =>{
                if self.source.in_freelist_r(existing) { return None; };

                Some(existing)
            }
        }
    }

    fn to(&self, to: Handle<TTo>) -> Option<Handle<TFrom>> {
        match self.abs.to_2_from.get(&to) {
            None => None,
            Some(&existing) =>{
                if self.source.in_freelist_l(existing) { return None; };

                Some(existing)
            }
        }
    }
}

impl<'r, S, TFrom, TTo> ReifiedView<TFrom, TTo> for ReiM<'r, S, TFrom, TTo> where S: SourceM<TFrom, TTo> {
    // these values come from the user initially
    fn has(&self, from: Handle<TFrom>, to: Handle<TTo>) -> bool {
        if self.abs.from_2_to.get(&from) != Some(&to) { return false; }
        // proven to be in the store: we can just check the freelist
        if self.source.in_freelist_l(from) { return false; }
        if self.source.in_freelist_r(to) { return false; }
        return true;
    }

    fn from(&self, from: Handle<TFrom>) -> Option<Handle<TTo>> {
        match self.abs.from_2_to.get(&from) {
            None => None,
            Some(&existing) =>{
                if self.source.in_freelist_r(existing) { return None; };

                Some(existing)
            }
        }
    }

    fn to(&self, to: Handle<TTo>) -> Option<Handle<TFrom>> {
        match self.abs.to_2_from.get(&to) {
            None => None,
            Some(&existing) =>{
                if self.source.in_freelist_l(existing) { return None; };

                Some(existing)
            }
        }
    }
}

impl<'r, S, TFrom, TTo> ReifiedMut<TFrom, TTo> for ReiM<'r, S, TFrom, TTo> where S: SourceM<TFrom, TTo> {
    fn gc(&mut self) {
        self.source.do_frees(&mut self.abs.from_2_to, &mut self.abs.to_2_from, |r, rs| { rs.remove(&r); }, |l, ls| { ls.remove(&l); })
    }

    fn map(&mut self, from: Handle<TFrom>, to: Handle<TTo>) {
        if self.source.is_free_l(from) { panic!("user attempted to map a free `from` handle"); };
        if self.source.is_free_r(to) { panic!("user attempted to map a free `to` handle"); };

        self.abs.from_2_to.insert(from, to);
        self.abs.to_2_from.insert(to, from);
    }

    fn unmap(&mut self, from: Handle<TFrom>, to: Handle<TTo>) {
        // TODO: We effectively double this check right now
        if self.has(from, to) {
            self.abs.from_2_to.remove(&from);
            self.abs.to_2_from.remove(&to);
        }
    }

    fn unmap_from(&mut self, from: Handle<TFrom>) {
        match self.from(from) {
            None => {},
            Some(to) => {
                self.abs.from_2_to.remove(&from);
                self.abs.to_2_from.remove(&to);
            }
        }
    }

    fn unmap_to(&mut self, to: Handle<TTo>) {
        match self.to(to) {
            None => {},
            Some(from) => {
                self.abs.from_2_to.remove(&from);
                self.abs.to_2_from.remove(&to);
            }
        }
    }
}