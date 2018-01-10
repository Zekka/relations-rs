use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::ops;

use experimental::query::Query;
use experimental::handle::Handle;
use experimental::index::Indexer;
use experimental::plan::TablePlan;

use experimental::loops::*;

pub struct Table<A> where A: 'static {
    pub(crate) plan: TablePlan<'static, A>,
    pub(crate) indices: Vec<Box<Indexer<A>>>,

    pub(crate) next_handle: u64,
    pub(crate) unordered: HashMap<Handle<A>, A>,

    pub(crate) relation_freelists: Vec<HashSet<Handle<A>>>,
}

impl<A> Table<A> {
    pub fn new() -> Table<A> {
        Table {
            indices: Vec::new(),
            next_handle: 0,
            unordered: HashMap::new(),
            relation_freelists: Vec::new(),
            plan: TablePlan::new(),
        }
    }

    pub(crate) fn handle_update(&mut self, handle: Handle<A>) {
        let value = self.unordered.get(&handle);
        if value.is_none() { return; }
        let value2 = value.unwrap();
        for index in self.indices.iter_mut() {
            index.handle_update(handle, value2);
        }
    }

    pub fn store(&mut self, item: A) -> Handle<A> {
        let handle = Handle { phantom: PhantomData, ix: self.next_handle };
        self.next_handle += 1;
        self.unordered.insert(handle, item);
        self.handle_update(handle);
        handle
    }

    pub fn free(&mut self, handle: Handle<A>) -> Option<A> {
        let val = self.unordered.remove(&handle);
        match val {
            None => None,
            Some(x) =>  {
                for index in self.indices.iter_mut() {
                    index.handle_free(handle, &x);
                };
                for list in self.relation_freelists.iter_mut() {
                    list.insert(handle);
                }
                Some(x)
            }
        }
    }

    pub fn seek(&self, handle: Handle<A>) -> Option<&A> {
        self.unordered.get(&handle)
    }

    pub fn seek_mut<'a>(&'a mut self, handle: Handle<A>) -> Option<TableMutRef<'a, A>> {
        {
            let value = self.unordered.get_mut(&handle);
            if value.is_none() { return None; };
        }

        Some(TableMutRef {
            table: self,
            handle: handle
        })
    }

    pub fn resolve<'a>(&'a self, base: Query<A>) -> Box<Iterator<Item=(Handle<A>, &A)>+'a>  where A: 'a {
        Box::new(base.run(&self.indices).map(move |a| { (a, self.unordered.get(&a).unwrap()) }))
    }

    // The default iterator api lets you hold onto your refs. It's obviously too dangerous for us to expose it like that.
    pub fn mutate< F, E>(&mut self, base: Query<A>, cb: F) where 
        F: Fn(Handle<A>, &mut A, &mut TablePlan<'static, A>) -> E, 
        E: End,
    {
        // We would ordinarily create a new plan, but instead with unsafe we reuse the existing plan. Don't fear! 
        // execute_and_clear() is guaranteed to leave the plan blank of old, dangerous state.
        let mut plan = unsafe {
            &mut *(&mut self.plan as *mut TablePlan<'static, A>)
        };
        {
            let mut handles_altered: Vec<Handle<A>> = Vec::new();
            for h in base.run(&self.indices) {
                let break_after = cb(h, self.unordered.get_mut(&h).unwrap(), &mut plan).breaks_loop();
                if break_after { break; }
                handles_altered.push(h);
            };
            for h in handles_altered {
                self.handle_update(h);
            }
        }
        plan.execute_and_clear(self);
    }
}

// TODO: For future performance, instead of storing the handle, store a direct mutable ref to the A
pub struct TableMutRef<'a, A> where A: 'static {
    table: &'a mut Table<A>,
    handle: Handle<A>,
}

impl<'a, A> ops::Deref for TableMutRef<'a, A> {
    type Target = A;

    fn deref(&self) -> &A {
        self.table.unordered.get(&self.handle).unwrap()
    }
}

impl<'a, A> ops::DerefMut for TableMutRef<'a, A> {
    fn deref_mut(&mut self) -> &mut A {
        self.table.unordered.get_mut(&self.handle).unwrap()
    }
}

impl<'a, A> Drop for TableMutRef<'a, A> {
    fn drop(&mut self) {
        self.table.handle_update(self.handle)
    }
}