use std::ops::Deref;

use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `Deref` types
pub trait DerefList<'a> {
    type Output;

    fn deref_list(self) -> Self::Output;
}

impl<'a, Head, Tail> DerefList<'a> for HCons<&'a Head, Tail>
where
    Head: Deref,
    Head::Target: Sized,
    Tail: DerefList<'a>,
{
    type Output = HCons<&'a Head::Target, Tail::Output>;

    fn deref_list(self) -> Self::Output {
        HCons {
            head: self.head.deref(),
            tail: self.tail.deref_list(),
        }
    }
}

impl<'a> DerefList<'a> for HNil {
    type Output = HNil;

    fn deref_list(self) -> Self::Output {
        HNil
    }
}
