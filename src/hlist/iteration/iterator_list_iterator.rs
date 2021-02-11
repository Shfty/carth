use crate::hlist::{iteration::IteratorList, OptionList};

/// An iterator over an `IteratorList` type
pub struct IteratorListIterator<T, P>
where
    T: IteratorList,
    P: FnOnce(&<T as IteratorList>::Output) -> bool,
{
    iterator_list: T,
    predicate: P,
}

impl<T, P> IteratorListIterator<T, P>
where
    T: IteratorList,
    P: FnOnce(&<T as IteratorList>::Output) -> bool,
{
    pub fn new(iterator_list: T, predicate: P) -> Self {
        IteratorListIterator {
            iterator_list,
            predicate,
        }
    }
}

impl<T, P> Iterator for IteratorListIterator<T, P>
where
    T: IteratorList,
    T::Output: OptionList,
    P: FnOnce(&<T as IteratorList>::Output) -> bool + Copy,
{
    type Item = <T as IteratorList>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        let result = IteratorList::list_next(&mut self.iterator_list);
        if (self.predicate)(&result) {
            Some(result)
        } else {
            None
        }
    }
}
