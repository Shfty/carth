use frunk_core::hlist::{HCons, HNil};

use crate::hlist::homogenous_list::HomogenousList;

/// A `HomogenousList` type that can overwrite its values with the contents of an iterator
///
/// On account of the fixed size of `HList` types, this is not guaranteed to consume the entire iterator
pub trait HomogenousListCollect<V, I>: HomogenousList<V>
where
    I: Iterator<Item = V>,
{
    fn collect(&mut self, iter: &mut I);
}

impl<I, V, Tail> HomogenousListCollect<V, I> for HCons<V, Tail>
where
    Self: HomogenousList<V>,
    I: Iterator<Item = V>,
    Tail: HomogenousListCollect<V, I>,
{
    fn collect(&mut self, iter: &mut I) {
        if let Some(next) = iter.next() {
            self.head = next;
            self.tail.collect(iter);
        }
    }
}

impl<V, I> HomogenousListCollect<V, I> for HNil
where
    Self: HomogenousList<V>,
    I: Iterator<Item = V>,
{
    fn collect(&mut self, _: &mut I) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk_core::hlist;

    #[test]
    fn test_list_collect() {
        let mut iter = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].into_iter();
        let mut list = hlist![0, 0, 0, 0, 0, 0, 0, 0, 0];
        list.collect(&mut iter);
        println!("Collected: {:?}", list);
    }
}
