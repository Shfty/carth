use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `Iterator` types
pub trait IteratorList {
    type Output;

    fn list_next(&mut self) -> Self::Output;
}

impl<Head, Tail> IteratorList for HCons<Head, Tail>
where
    Head: Iterator,
    Tail: IteratorList,
{
    type Output = HCons<Option<Head::Item>, Tail::Output>;

    fn list_next(&mut self) -> Self::Output {
        HCons {
            head: self.head.next(),
            tail: self.tail.list_next(),
        }
    }
}

impl IteratorList for HNil {
    type Output = HNil;

    fn list_next(&mut self) -> Self::Output {
        HNil
    }
}