use std::fmt::Debug;

fn assert_lte<A: Ord + Debug>(a: A, b: A) {
    assert!(a <= b, "Expected {a:?} <= {b:?}");
}
