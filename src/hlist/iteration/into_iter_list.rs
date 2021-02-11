use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `IntoIter` types
pub trait IntoIterList {
    type Output;

    fn list_into_iter(self) -> Self::Output;
}

impl<Head, Tail> IntoIterList for HCons<Head, Tail>
where
    Head: IntoIterator,
    Tail: IntoIterList,
{
    type Output = HCons<Head::IntoIter, Tail::Output>;

    fn list_into_iter(self) -> Self::Output {
        HCons {
            head: self.head.into_iter(),
            tail: self.tail.list_into_iter(),
        }
    }
}

impl IntoIterList for HNil {
    type Output = HNil;

    fn list_into_iter(self) -> Self::Output {
        HNil
    }
}
