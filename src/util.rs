use std::fmt::Debug;
use std::iter;

pub fn naturals() -> impl Iterator<Item=usize> { iter::successors(Some(0), |x| Some(x + 1)) }

// pub fn fix<R>(f: &dyn Fn(&) -> R) -> R {
//     f(f)
// }

pub fn get_one<T>(mut i: impl Iterator<Item=T>) -> T {
    let res = i.next().unwrap();
    assert!(i.next().is_none());
    res
}

pub fn ensure_equal<T: PartialEq + Debug>(x: T, y: T) -> T {
    assert_eq!(x, y);
    return x;
}