use std::marker::PhantomData;

use frunk_core::hlist::HList;

use super::HomogenousList;

/// Iterator over a `HomogenousList` type
pub struct HomogenousListIterator<'a, HL, T>
where
    T: 'a,
    HL: HomogenousList<T>,
{
    hlist: &'a HL,
    index: usize,
    _phantom: PhantomData<T>,
}

impl<'a, HL, T> HomogenousListIterator<'a, HL, T>
where
    T: 'a,
    HL: HomogenousList<T>,
{
    pub fn new(hlist: &'a HL) -> Self {
        HomogenousListIterator {
            hlist,
            index: 0,
            _phantom: Default::default(),
        }
    }
}

impl<'a, HL, T> Iterator for HomogenousListIterator<'a, HL, T>
where
    HL: HList + HomogenousList<T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < <HL as HList>::LEN {
            let value = self.hlist.get_index(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = HL::LEN - self.index;
        (len, Some(len))
    }
}
