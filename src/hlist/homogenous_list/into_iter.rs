use frunk_core::hlist::{HCons, HList, HNil};

use crate::hlist::homogenous_list::ToVecList;

/// A `HomogenousList` type that can be converted into a by-value iterator
pub trait HomogenousListIntoIter<T> {
    fn into_iter(self) -> std::vec::IntoIter<T>;
}

impl<T, Tail> HomogenousListIntoIter<T> for HCons<T, Tail>
where
    Self: ToVecList<T>,
    Tail: HList + HomogenousListIntoIter<T>,
{
    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.list_to_vec().into_iter()
    }
}

impl<T> HomogenousListIntoIter<T> for HNil {
    fn into_iter(self) -> std::vec::IntoIter<T> {
        panic!()
    }
}