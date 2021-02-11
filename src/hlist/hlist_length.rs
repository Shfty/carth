use frunk_core::{
    hlist::{HCons, HNil},
    indices::{Here, There},
};

/// A `HList` with a type-encoded length
pub trait HListLength<Length> {}

impl<Head, Tail, TailLength> HListLength<There<TailLength>> for HCons<Head, Tail> where
    Tail: HListLength<TailLength>
{
}

impl<Head, Tail, TailLength> HListLength<There<TailLength>> for &HCons<Head, Tail> where
    Tail: HListLength<TailLength>
{
}

impl HListLength<Here> for HNil {}

#[cfg(test)]
mod tests {
    use super::*;

    use frunk_core::hlist;

    #[test]
    fn test_hlist_length() {
        let len_0 = hlist![];
        let len_1 = hlist![1];
        let len_2 = hlist![1, 3];
        let len_3 = hlist![1, 2, 3];

        let _proof_0: &dyn HListLength<Here> = &len_0;
        let _proof_1: &dyn HListLength<There<Here>> = &len_1;
        let _proof_2: &dyn HListLength<There<There<Here>>> = &len_2;
        let _proof_3: &dyn HListLength<There<There<There<Here>>>> = &len_3;
    }
}
