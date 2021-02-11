use frunk_core::traits::ToMut;

use crate::hlist::DerefMutList;

/// A `HList` of types that can be dereferenced via `ToMut` / `DerefMutList` coercion
pub trait CoerceMutList<'a> {
    type Output;

    fn coerce_mut(&'a mut self) -> Self::Output;
}

impl<'a, T> CoerceMutList<'a> for T
where
    T: ToMut<'a>,
    <T as ToMut<'a>>::Output: DerefMutList<'a>,
{
    type Output = <<T as ToMut<'a>>::Output as DerefMutList<'a>>::Output;

    fn coerce_mut(&'a mut self) -> Self::Output {
        self.to_mut().deref_mut_list()
    }
}
