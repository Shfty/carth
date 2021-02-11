use std::ops::DerefMut;

use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `DerefMut` types
pub trait DerefMutList<'a> {
    type Output;

    fn deref_mut_list(self) -> Self::Output;
}

impl<'a, Head, Tail> DerefMutList<'a> for HCons<&'a mut Head, Tail>
where
    Head: DerefMut,
    Head::Target: Sized,
    Tail: DerefMutList<'a>,
{
    type Output = HCons<&'a mut Head::Target, Tail::Output>;

    fn deref_mut_list(self) -> Self::Output {
        HCons {
            head: self.head.deref_mut(),
            tail: self.tail.deref_mut_list(),
        }
    }
}

impl<'a> DerefMutList<'a> for HNil {
    type Output = HNil;

    fn deref_mut_list(self) -> Self::Output {
        HNil
    }
}
