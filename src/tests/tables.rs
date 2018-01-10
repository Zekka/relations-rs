use experimental::*;
use tests::utils::assert_iter_equal;

#[test]
fn seek_and_store_and_free() {
    let mut t1 = Table::<&'static str>::new();

    let handle1 = t1.store("hello");
    assert_eq!(t1.seek(handle1), Some(&"hello"));

    let handle2 = t1.store("goodbye");
    assert_eq!(t1.seek(handle2), Some(&"goodbye"));

    assert!(handle1 != handle2);
    assert_eq!(t1.seek(handle1), Some(&"hello"));

    t1.free(handle1);
    assert_eq!(t1.seek(handle1), None);
    assert_eq!(t1.seek(handle2), Some(&"goodbye"));
}

#[test]
fn seek_mut() {
    let mut t1 = Table::<&'static str>::new();

    let handle1 = t1.store("hello");
    assert_eq!(t1.seek(handle1), Some(&"hello"));

    {
        let mut hello_ref = t1.seek_mut(handle1).unwrap();
        *hello_ref = "goodbye";
    }

    assert_eq!(t1.seek(handle1), Some(&"goodbye"));
}

#[test] 
fn indexing_mutate() {
    let mut t1: Table<&'static str> = Table::<&'static str>::new();

    let handle0 = t1.store("malta");

    let alphabetize = t1.create_sort(|_, s| *s);
    // this is not a reverse alphabetization, it's an alphabetization of the reversed version of the strings
    let alphabetize_reverse = t1.create_sort::<String>(|_, s| s.chars().rev().collect());

    let handle1 = t1.store("hello");
    let handle2 = t1.store("goodbye");
    let handle3 = t1.store("scout");

    let mut i = 0;
    assert_iter_equal(
        t1.resolve(alphabetize.asc()), 
        vec!((handle2, &"goodbye"), (handle1, &"hello"), (handle0, &"malta"), (handle3, &"scout"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.asc()), 
        vec!((handle0, &"malta"), (handle2, &"goodbye"), (handle1, &"hello"), (handle3, &"scout"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize.desc()), 
        vec!((handle3, &"scout"), (handle0, &"malta"), (handle1, &"hello"), (handle2, &"goodbye"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.desc()), 
        vec!((handle3, &"scout"), (handle1, &"hello"), (handle2, &"goodbye"), (handle0, &"malta"))
    );

    t1.mutate(alphabetize.asc(), |_, reference: &mut &'static str, _| {
        if *reference == "malta" {
            *reference = "golf";
        }
    });

    assert_iter_equal(
        t1.resolve(alphabetize.asc()), 
        vec!((handle0, &"golf"), (handle2, &"goodbye"), (handle1, &"hello"), (handle3, &"scout"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.asc()), 
        vec!((handle2, &"goodbye"), (handle0, &"golf"), (handle1, &"hello"), (handle3, &"scout"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize.desc()), 
        vec!((handle3, &"scout"), (handle1, &"hello"), (handle2, &"goodbye"), (handle0, &"golf"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.desc()), 
        vec!((handle3, &"scout"), (handle1, &"hello"), (handle0, &"golf"), (handle2, &"goodbye"))
    );

    t1.free(handle3);

    assert_iter_equal(
        t1.resolve(alphabetize.asc()), 
        vec!((handle0, &"golf"), (handle2, &"goodbye"), (handle1, &"hello"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.asc()), 
        vec!((handle2, &"goodbye"), (handle0, &"golf"), (handle1, &"hello"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize.desc()), 
        vec!((handle1, &"hello"), (handle2, &"goodbye"), (handle0, &"golf"))
    );
    assert_iter_equal(
        t1.resolve(alphabetize_reverse.desc()), 
        vec!((handle1, &"hello"), (handle0, &"golf"), (handle2, &"goodbye"))
    );
}

fn plans() {
    // TODO: Test *using* plans.
}

// TODO: Test should-panics