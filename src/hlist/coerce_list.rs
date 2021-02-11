use frunk_core::traits::ToRef;

use crate::hlist::DerefList;

/// A `HList` of types that can be dereferenced via `ToRef` / `DerefList` coercion
pub trait CoerceList<'a> {
    type Output;

    fn coerce(&'a self) -> Self::Output;
}

impl<'a, T> CoerceList<'a> for T
where
    T: ToRef<'a>,
    <T as ToRef<'a>>::Output: DerefList<'a>,
{
    type Output = <<T as ToRef<'a>>::Output as DerefList<'a>>::Output;

    fn coerce(&'a self) -> Self::Output {
        self.to_ref().deref_list()
    }
}
