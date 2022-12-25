use std::iter;

pub fn naturals() -> impl Iterator<Item=usize> { iter::successors(Some(0), |x| Some(x + 1)) }

pub fn fix<T, R, F: Fn(&F, T) -> R>(f: &F, t: T) -> R {
    f(f, t)
}

pub fn get_one<T>(mut i: impl Iterator<Item=T>) -> T {
    let res = i.next().unwrap();
    assert!(i.next().is_none());
    res
}