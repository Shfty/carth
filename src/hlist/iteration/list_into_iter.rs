use super::{IntoIterList, IteratorList, IteratorListIterator};

/// A `HList` that can be converted into a `IteratorListIterator`
pub trait IntoIterListIntoIter<T, P>
where
    T: IntoIterList,
    <T as IntoIterList>::Output: IteratorList,
    P: FnOnce(&<<T as IntoIterList>::Output as IteratorList>::Output) -> bool,
{
    type Iter;

    fn into_iter(self, predicate: P) -> Self::Iter;
}

impl<T, P> IntoIterListIntoIter<T, P> for T
where
    T: IntoIterList,
    <T as IntoIterList>::Output: IteratorList,
    P: FnOnce(&<<T as IntoIterList>::Output as IteratorList>::Output) -> bool,
{
    type Iter = IteratorListIterator<<T as IntoIterList>::Output, P>;

    fn into_iter(self, predicate: P) -> Self::Iter {
        IteratorListIterator::new(self.list_into_iter(), predicate)
    }
}
