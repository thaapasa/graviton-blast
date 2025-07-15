use std::fmt::Debug;

#[allow(unused)]
fn assert_lte<A: Ord + Debug>(a: A, b: A) {
    assert!(a <= b, "Expected {a:?} <= {b:?}");
}
