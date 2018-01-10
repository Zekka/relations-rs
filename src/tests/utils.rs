use std::fmt;

// copied from itertools
pub fn assert_iter_equal<I, J>(a: I, b: J)
    where I: IntoIterator,
          J: IntoIterator,
          I::Item: fmt::Debug + PartialEq<J::Item>,
          J::Item: fmt::Debug,
{
    let mut ia = a.into_iter();
    let mut ib = b.into_iter();
    let mut i = 0;
    loop {
        match (ia.next(), ib.next()) {
            (None, None) => return,
            (a, b) => {
                let equal = match (&a, &b) {
                    (&Some(ref a), &Some(ref b)) => a == b,
                    _ => false,
                };
                assert!(equal, "Failed assertion {a:?} == {b:?} for iteration {i}",
                        i=i, a=a, b=b);
                i += 1;
            }
        }
    }
}