use experimental::handle::Handle;
use experimental::relations::source::*;
use experimental::relations::onetoone::implementation::*; // we could make ReiV and ReiM type parameters, but why?
use experimental::table::Table;

use experimental::loops::BreakContinue;

pub(crate) trait Abstract<TFrom, TTo> {
    fn reify<'q>(&'q self, from: &'q Table<TFrom>, to: &'q Table<TTo>) -> Box<ReiV<'q, JoinV<TFrom, TTo>, TFrom, TTo>>;
    fn reify_mut<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>) -> Box<ReiM<'q, JoinM<TFrom, TTo>, TFrom, TTo>>;

    fn mutate_to<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        tos: Box<Iterator<Item=Handle<TTo>>+'q>, 
        cb: &Fn(Handle<TFrom>, &mut TFrom) -> BreakContinue // can't make this generic because that would break trait objecting for Abstract
    );
    fn mutate_from<'q>(&'q mut self, from: &'q mut Table<TFrom>, to: &'q mut Table<TTo>, 
        froms: Box<Iterator<Item=Handle<TFrom>>+'q>, 
        cb: &Fn(Handle<TTo>, &mut TTo) -> BreakContinue // can't make this generic because that would break trait objecting for Abstract
    );
}

pub(crate) trait AbstractSelf<TSelf> {
    fn reify1<'q>(&'q self, tself: &'q Table<TSelf>) -> Box<ReiV<'q, SelfJoinV<TSelf>, TSelf, TSelf>>;
    fn reify1_mut<'q>(&'q mut self, tself: &'q mut Table<TSelf>) -> Box<ReiM<'q, SelfJoinM<TSelf>, TSelf, TSelf>>;

    fn mutate1_to<'q>(&'q mut self, tself: &'q mut Table<TSelf>, 
        tos: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: &Fn(Handle<TSelf>, &mut TSelf) -> BreakContinue // can't make this generic because that would break trait objecting for AbstractSelf
    );
    fn mutate1_from<'q>(&'q mut self, tself: &'q mut Table<TSelf>, 
        froms: Box<Iterator<Item=Handle<TSelf>>+'q>, 
        cb: &Fn(Handle<TSelf>, &mut TSelf) -> BreakContinue // can't make this generic because that would break trait objecting for AbstractSelf
    );
}

pub(crate) trait ReifiedView<TFrom, TTo> {
    fn has(&self, Handle<TFrom>, Handle<TTo>) -> bool;

    fn has_from(&self, h: Handle<TFrom>) -> bool { self.from(h).is_some() }
    fn has_to(&self, h: Handle<TTo>) -> bool { self.to(h).is_some() }

    fn from(&self, Handle<TFrom>) -> Option<Handle<TTo>>;
    fn froms<'q>(&'q self, hand: Handle<TFrom>) -> Box<Iterator<Item=Handle<TTo>> + 'q> where TTo: 'q {
        Box::new(self.from(hand).into_iter())
    }
    fn to(&self, Handle<TTo>) -> Option<Handle<TFrom>>;
    fn tos<'q>(&'q self, hand: Handle<TTo>) -> Box<Iterator<Item=Handle<TFrom>> + 'q> where TFrom: 'q {
        Box::new(self.to(hand).into_iter())
    }
}

pub(crate) trait ReifiedMut<TFrom, TTo>: ReifiedView<TFrom, TTo> {
    fn gc(&mut self); // does freeing, making the other methods more performant

    fn map(&mut self, Handle<TFrom>, Handle<TTo>);
    fn unmap(&mut self, Handle<TFrom>, Handle<TTo>);

    fn unmap_from(&mut self, Handle<TFrom>);
    fn unmap_to(&mut self, Handle<TTo>);
}

// TODO: OneToMany, ManyToMany.