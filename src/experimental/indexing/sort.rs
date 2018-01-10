use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use experimental::handle::Handle;
use experimental::table::Table;
use experimental::index::Indexer;
use experimental::query::Query;

struct Sorter<K, A> where A: 'static, K: 'static + Ord {
    next_insertion: u64,
    grabkey: fn(Handle<A>, &A) -> K,

    ordered: BTreeMap<Insertion<K>, Handle<A>>, // TODO: Don't be a dope! Keep insertion order. To do this, probably replace key with (Key, usize) so insertion order is maintained
    old_spot: HashMap<Handle<A>, Insertion<K>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Insertion<K> {
    key: K,
    insert_position: u64
}

impl<A> Table<A> {
    pub fn create_sort<K>(&mut self, grabkey: fn(Handle<A>, &A) -> K) -> Sort<K, A> where A: 'static, K: 'static + Ord {
        let ix = self.indices.len();

        let mut index: Box<Sorter<K, A>> = Box::new(Sorter {
            next_insertion: 0, 
            grabkey: grabkey, 
            ordered: BTreeMap::new(),
            old_spot: HashMap::new(),
        });
        println!("going to add all of unordered"); // apparently empty?
        for (h, ref el) in self.unordered.iter() {
            index.handle_update(*h, el)
        }
        self.indices.push(index);

        Sort {phantom1: ::std::marker::PhantomData, phantom2: ::std::marker::PhantomData, ix: ix}
    }
}

impl<K, A> Sorter<K, A> where A: 'static, K: 'static + Ord {
    // determines if we're inserting something anew (in which case we make a new insertion) 
    // or updating something old (in which case we delete it and make a new insertion)
    fn handle_free_make_insertion(&mut self, handle: Handle<A>, value: &A) -> u64 {
        match self.old_spot.entry(handle) {
            Entry::Vacant(_) => {
                let insertion_id = self.next_insertion;
                self.next_insertion += 1;
                insertion_id
            },
            Entry::Occupied(occupy) => {
                let g = occupy.get();
                let insertion_id = g.insert_position;
                self.ordered.remove(g);
                insertion_id
            }
        }
    }
}

impl<K, A> Indexer<A> for Sorter<K, A> where A: 'static, K: 'static + Ord {
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }

    fn handle_update(&mut self, handle: Handle<A>, value: &A) {
        // free existing entry if present
        let insertion_id = self.handle_free_make_insertion(handle, value);

        self.old_spot.insert(handle, Insertion { key: (self.grabkey)(handle, value), insert_position: insertion_id});
        self.ordered.insert(Insertion { key : (self.grabkey)(handle, value), insert_position: insertion_id}, handle);
    }

    fn handle_free(&mut self, handle: Handle<A>, value: &A) {
        self.handle_free_make_insertion(handle, value);
    }
}

pub struct Sort<K, A> where A: 'static, K: 'static + Ord {
    phantom1: ::std::marker::PhantomData<K>,
    phantom2: ::std::marker::PhantomData<A>,
    ix: usize 
}

impl<K, A> Sort<K, A> where A: 'static, K: 'static + Ord {
    // get the original Sort back out of the Table
    fn _priv_find<'a>(ix: usize, indices: &'a Vec<Box<Indexer<A>>>) -> &'a Sorter<K, A> {
        indices[ix].as_any().downcast_ref::<Sorter<K, A>>().unwrap()
    }

    // PER RPJOHNST: it's possible the box could end up on the heap, but most likely in release builds it will be optimized out
    fn _priv_asc<'a>(ix: usize, indices: &'a Vec<Box<Indexer<A>>>) -> Box<Iterator<Item=(Handle<A>)> + 'a> {
        let sorter = Sort::<K, A>::_priv_find(ix, &indices);

        Box::new(sorter.ordered.values().map(move |v| { (*v) }))
    }

    fn _priv_desc<'a>(ix: usize, indices: &'a Vec<Box<Indexer<A>>>) -> Box<Iterator<Item=(Handle<A>)> + 'a> {
        let sorter = Sort::<K, A>::_priv_find(ix, &indices);

        Box::new(sorter.ordered.values().rev().map(move |v| { (*v) }))
    }

    pub fn asc(&self) -> Query<A> {
        Query { body: Sort::<K, A>::_priv_asc, ix: self.ix }
    }

    pub fn desc(&self) -> Query<A> {
        Query { body: Sort::<K, A>::_priv_desc, ix: self.ix }
    }
}