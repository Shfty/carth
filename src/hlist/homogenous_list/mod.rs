//! Traits for `HList` type containing homogenous values

mod into_iter;
mod iter;
mod iterator;
mod to_vec_list;

pub use into_iter::*;
pub use iter::*;
pub use iterator::*;
pub use to_vec_list::*;

use frunk_core::hlist::{HCons, HList, HNil};

/// A `HList` containing homogenous values
pub trait HomogenousList<T> {
    fn get_index(&self, index: usize) -> &T;
}

impl<T, Tail> HomogenousList<T> for HCons<T, Tail>
where
    Tail: HList + HomogenousList<T>,
{
    fn get_index(&self, index: usize) -> &T {
        if index == 0 {
            &self.head
        } else {
            self.tail.get_index(index - 1)
        }
    }
}

impl<T> HomogenousList<T> for HNil {
    fn get_index(&self, _: usize) -> &T {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use frunk_core::hlist;

    use super::{HomogenousList, HomogenousListIntoIter, HomogenousListIter};

    #[test]
    fn homogenous_list_iterator() {
        let homogenous_list = hlist![1, 2, 4, 8, 16, 32, 64];
        println!("\nList:\n{:#?}", &homogenous_list);
        let _proof: &dyn HomogenousList<i32> = &homogenous_list;
        let iter = HomogenousListIter::iter(&homogenous_list);
        for int in iter {
            println!("Int: {:?}", int);
        }
    }

    #[test]
    fn homogenous_list_into_iterator() {
        let homogenous_list = hlist![1, 2, 4, 8, 16, 32, 64];
        println!("\nList:\n{:#?}", &homogenous_list);
        let _proof: &dyn HomogenousList<i32> = &homogenous_list;
        let iter = homogenous_list.into_iter();
        for int in iter {
            println!("Int: {:?}", int);
        }
    }
}
