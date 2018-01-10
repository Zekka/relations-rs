use experimental::handle::Handle;
use experimental::table::Table;

use std::ops::DerefMut;

pub struct TablePlan<'q, A> {
    store: Vec<A>,
    store_and: Vec<(A, Box<Fn(Handle<A>, &mut A) + 'q>)>,
    free: Vec<Handle<A>>,
    free_and: Vec<(Handle<A>, Box<Fn(A) + 'q>)>,
}

impl<'q, A> TablePlan<'q, A> {
    pub fn new() -> TablePlan<'q, A> {
        TablePlan {
            store: Vec::new(),
            store_and: Vec::new(),
            free: Vec::new(),
            free_and: Vec::new(),
        }
    }

    pub fn store(&mut self, a: A) {
        self.store.push(a)
    }

    pub fn store_and<F>(&mut self, a: A, callback: F)
        where F: Fn(Handle<A>, &mut A) + 'q
    {
        self.store_and.push((a, Box::new(callback)))
    }

    pub fn free(&mut self, a: Handle<A>) {
        self.free.push(a)
    }

    pub fn free_and<F>(&mut self, a: Handle<A>, callback: F)
        where F: Fn(A) + 'q
    {
        self.free_and.push((a, Box::new(callback)))
    }

    // executes the plan and, as a side effect, clears the plan
    // NOTE!!!!!: Due to unsafe in table.rs, this *will* make Rust segfault if you don't completely clear out all old Boxes.
    pub(crate) fn execute_and_clear(&mut self, table: &mut Table<A>) {
        for item in self.store.drain(..) {
            table.store(item);
        }
        for (item, cb) in self.store_and.drain(..) {
            let h = table.store(item);
            let mut seek = table.seek_mut(h).unwrap();
            cb(h, seek.deref_mut())
        }
        for item in self.free.drain(..) {
            table.free(item);
        }
    }
}