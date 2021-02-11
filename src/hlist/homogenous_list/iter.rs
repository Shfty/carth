use frunk_core::hlist::{HCons, HList, HNil};

use super::{HomogenousList, HomogenousListIterator};

/// A `HomogenousList` type that can be iterated over
pub trait HomogenousListIter<'a, T>
where
    T: 'a,
{
    type Iter: Iterator<Item = &'a T>;

    fn iter(&'a self) -> Self::Iter;
}

impl<'a, T, Tail> HomogenousListIter<'a, T> for HCons<T, Tail>
where
    Self: HomogenousList<T>,
    T: 'a,
    Tail: 'a + HList + HomogenousListIter<'a, T>,
{
    type Iter = HomogenousListIterator<'a, Self, T>;

    fn iter(&'a self) -> Self::Iter {
        HomogenousListIterator::new(self)
    }
}

impl<'a, T> HomogenousListIter<'a, T> for HNil
where
    T: 'a,
{
    type Iter = HomogenousListIterator<'a, Self, T>;

    fn iter(&'a self) -> Self::Iter {
        panic!()
    }
}