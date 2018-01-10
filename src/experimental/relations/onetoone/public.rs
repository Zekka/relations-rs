use experimental::loops::*;
use experimental::relations::onetoone::interfaces::*;
use experimental::relations::onetoone::implementation::*;
use experimental::table::*;
use experimental::handle::*;

use std::collections::HashMap;
use std::collections::HashSet;

pub struct Rel<TFrom, TTo> {
    inner: Box<Abstract<TFrom, TTo>>,
}

pub struct RelSelf<TSelf> {
    inner: Box<AbstractSelf<TSelf>>,
}

pub struct View<'q, TFrom, TTo> {
    inner: Box<ReifiedView<TFrom, TTo> + 'q>,
}

pub struct Mut<'q, TFrom, TTo> {
    inner: Box<ReifiedMut<TFrom, TTo> + 'q>,
}

impl<TFrom, TTo> Rel<TFrom, TTo> {
    pub fn reify<'q>(&'q self, from: &'q Table<TFrom>, to: &'q Table<TTo>) -> View<'q, TFrom, TTo> {
        View {inner: self.inner.reify::<'q>(from, to)}
    }
    pub fn reify_mut<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>) -> Mut<'q, TFrom, TTo> {
        Mut {inner: self.inner.reify_mut::<'q>(from, to)}
    }

    pub fn mutate_to<'q, F, E>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        tos: Box<Iterator<Item=Handle<TTo>>+'q>, 
        cb: F
    ) where F: Fn(Handle<TFrom>, &mut TFrom) -> E, E: End
    {
        self.inner.mutate_to(from, to, tos, &|h, m| { cb(h, m).to_break_continue() })
    }

    pub fn mutate_from<'q, F, E>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        froms: Box<Iterator<Item=Handle<TFrom>>+'q>, 
        cb: F
    ) where F: Fn(Handle<TTo>, &mut TTo) -> E, E: End
    {
        self.inner.mutate_from(from, to, froms, &|h, m| { cb(h, m).to_break_continue() });
    }
}

impl<TSelf> RelSelf<TSelf> {
    pub fn reify<'q>(&'q self, tself: &'q Table<TSelf>) -> View<'q, TSelf, TSelf> {
        View {inner: self.inner.reify1::<'q>(tself)}
    }

    pub fn reify_mut<'q>(&'q mut self, tself: &'q mut Table<TSelf>) -> Mut<'q, TSelf, TSelf> {
        Mut {inner: self.inner.reify1_mut::<'q>(tself)}
    }

    pub fn mutate_to<'q, F, E>(&'q mut self, tself: &'q mut Table<TSelf>, 
        froms: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: F
    ) where F: Fn(Handle<TSelf>, &mut TSelf) -> E, E: End
    {
        self.inner.mutate1_to(tself, froms, &|h, m| { cb(h, m).to_break_continue() });
    }

    pub fn mutate_from<'q, F, E>(&'q mut self, tself: &'q mut Table<TSelf>, 
        tos: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: F
    ) where F: Fn(Handle<TSelf>, &mut TSelf) -> E, E: End 
    {
        self.inner.mutate1_from(tself, tos, &|h, m| { cb(h, m).to_break_continue() });
    }
}

pub trait CanView<TFrom, TTo> {
    fn has(&self, Handle<TFrom>, Handle<TTo>) -> bool;
    fn has_from(&self, Handle<TFrom>) -> bool;
    fn has_to(&self, Handle<TTo>) -> bool;

    fn from(&self, Handle<TFrom>) -> Option<Handle<TTo>>;
    fn froms<'b>(&'b self, Handle<TFrom>) -> Box<Iterator<Item=Handle<TTo>> + 'b> where TTo: 'b;

    fn to(&self, h: Handle<TTo>) -> Option<Handle<TFrom>>;
    fn tos<'b>(&'b self, h: Handle<TTo>) -> Box<Iterator<Item=Handle<TFrom>> + 'b> where TFrom: 'b;
}

impl<'q, TFrom, TTo> CanView<TFrom, TTo> for View<'q, TFrom, TTo> {
    fn has(&self, tfrom: Handle<TFrom>, tto: Handle<TTo>) -> bool { self.inner.has(tfrom, tto) }

    fn has_from(&self, h: Handle<TFrom>) -> bool { self.inner.has_from(h) }
    fn has_to(&self, h: Handle<TTo>) -> bool { self.inner.has_to(h) }

    fn from(&self, h: Handle<TFrom>) -> Option<Handle<TTo>> { self.inner.from(h) }
    fn froms<'b>(&'b self, h: Handle<TFrom>) -> Box<Iterator<Item=Handle<TTo>> + 'b> where TTo: 'b {
        self.inner.froms(h)
    }
    fn to(&self, h: Handle<TTo>) -> Option<Handle<TFrom>> { self.inner.to(h) }
    fn tos<'b>(&'b self, h: Handle<TTo>) -> Box<Iterator<Item=Handle<TFrom>> + 'b> where TFrom: 'b {
        self.inner.tos(h)
    }
}

impl <'q, TFrom, TTo> CanView<TFrom, TTo> for Mut<'q, TFrom, TTo> {
    fn has(&self, tfrom: Handle<TFrom>, tto: Handle<TTo>) -> bool { self.inner.has(tfrom, tto) }

    fn has_from(&self, h: Handle<TFrom>) -> bool { self.inner.has_from(h) }
    fn has_to(&self, h: Handle<TTo>) -> bool { self.inner.has_to(h) }

    fn from(&self, h: Handle<TFrom>) -> Option<Handle<TTo>> { self.inner.from(h) }
    fn froms<'b>(&'b self, h: Handle<TFrom>) -> Box<Iterator<Item=Handle<TTo>> + 'b> where TTo: 'b {
        self.inner.froms(h)
    }
    fn to(&self, h: Handle<TTo>) -> Option<Handle<TFrom>> { self.inner.to(h) }
    fn tos<'b>(&'b self, h: Handle<TTo>) -> Box<Iterator<Item=Handle<TFrom>> + 'b> where TFrom: 'b {
        self.inner.tos(h)
    }
}

impl<'q, TFrom, TTo> Mut<'q, TFrom, TTo> {
    pub fn gc(&mut self) { self.inner.gc() }

    pub fn map(&mut self, tfrom: Handle<TFrom>, tto: Handle<TTo>) { self.inner.map(tfrom, tto) }
    pub fn unmap(&mut self, tfrom: Handle<TFrom>, tto: Handle<TTo>) { self.inner.map(tfrom, tto) }

    pub fn unmap_from(&mut self, tfrom: Handle<TFrom>) { self.inner.unmap_from(tfrom) }
    pub fn unmap_to(&mut self, tto: Handle<TTo>) { self.inner.unmap_to(tto) }
}

impl<A> Table<A> {
    pub fn one_to_one_self(&mut self) -> RelSelf<A> {
        let new_ix = self.relation_freelists.len();
        self.relation_freelists.push(HashSet::new());

        RelSelf {
            inner: Box::new(Abs {
                from_2_to: HashMap::new(),
                to_2_from: HashMap::new(),
                ix_from: new_ix, 
                ix_to: new_ix, 
            })
        }
    }
    pub fn one_to_one<B>(&mut self, target: &mut Table<B>) -> Rel<A, B> {
        let new_ix_from = self.relation_freelists.len();
        let new_ix_to = target.relation_freelists.len();
        self.relation_freelists.push(HashSet::new());
        target.relation_freelists.push(HashSet::new());

        Rel {
            inner: Box::new(Abs {
                from_2_to: HashMap::new(),
                to_2_from: HashMap::new(),
                ix_from: new_ix_from, 
                ix_to: new_ix_to, 
            })
        }
    }
}