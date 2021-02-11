//! Two-dimensional table structure based on nested `HList`s

mod pop_column;
mod transpose;

pub use pop_column::*;
pub use transpose::*;

use frunk_core::hlist::{HCons, HList, HNil};

use crate::hlist::HListLength;

/// A `HList` of `HListLength` types with the same length
pub trait HTable<Width>: HList {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn width(&self) -> usize {
        Self::WIDTH
    }

    fn height(&self) -> usize {
        Self::HEIGHT
    }
}

impl<Width, Head, Tail> HTable<Width> for HCons<Head, Tail>
where
    Head: HList + HListLength<Width>,
    Tail: HTable<Width>,
{
    const WIDTH: usize = <Head as HList>::LEN;
    const HEIGHT: usize = <Self as HList>::LEN;
}

impl<Width> HTable<Width> for HNil {
    const WIDTH: usize = 0;
    const HEIGHT: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk_core::{
        hlist,
        indices::{Here, There},
    };

    #[test]
    fn hlist_length() {
        let len_0 = hlist![];
        let len_1 = hlist![1];
        let len_2 = hlist![1, 3];
        let len_3 = hlist![1, 2, 3];

        let _proof_0: &dyn HListLength<Here> = &len_0;
        let _proof_1: &dyn HListLength<There<Here>> = &len_1;
        let _proof_2: &dyn HListLength<There<There<Here>>> = &len_2;
        let _proof_3: &dyn HListLength<There<There<There<Here>>>> = &len_3;
    }

    #[test]
    fn htable() {
        let table = hlist![hlist![1, 2, 3], hlist![4, 5, 6], hlist![7, 8, 9]];

        let width = table.width();
        let height = table.height();

        println!("Width: {}, Height: {}", width, height);
    }
}
