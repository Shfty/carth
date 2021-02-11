//! A recursive tree structure based on nested `HList`s

use frunk_core::hlist::{HCons, HNil};

/// Recursive heterogeneous tree structure of `HList` types containing other `HList` types
pub trait HTree<C>
where
    C: Copy,
{
    type Output;

    /// Entrypoint for `HTree` implementors.
    /// A generic context parameter whose type is shared across the whole tree can be provided to control execution.
    fn recurse(&self, context: C) -> Self::Output;
}

/// Main blanket implementation, allows for recursive dispatch over a tree of `HList` types
impl<C, Head, Tail> HTree<C> for HCons<Head, Tail>
where
    C: Copy,
    Head: HTree<C>,
    Tail: HTree<C>,
{
    type Output = HCons<Head::Output, Tail::Output>;

    fn recurse(&self, context: C) -> Self::Output {
        HCons {
            head: self.head.recurse(context),
            tail: self.tail.recurse(context),
        }
    }
}

/// HNil implementation
impl<C> HTree<C> for HNil
where
    C: Copy,
{
    type Output = HNil;

    fn recurse(&self, _: C) -> Self::Output {
        HNil
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use frunk_core::hlist;

    use super::*;

    /// Simple example trait for a `Hlist![i32; _]` type
    pub trait IntList {
        fn int_list(&self);
    }

    impl<Tail> IntList for HCons<i32, Tail>
    where
        Tail: IntList + Debug,
    {
        fn int_list(&self) {
            println!("Int list: {:?}", self)
        }
    }

    impl IntList for HNil {
        fn int_list(&self) {}
    }

    /// Simple example trait for a `Hlist![f32; _]` type
    pub trait FloatList {
        fn float_list(&self);
    }

    impl<Tail> FloatList for HCons<f32, Tail>
    where
        Tail: FloatList + Debug,
    {
        fn float_list(&self) {
            println!("Float list: {:?}", self)
        }
    }

    impl FloatList for HNil {
        fn float_list(&self) {}
    }

    /// Simple example trait for a `Hlist![char, _]` type
    pub trait CharList {
        fn char_list(&self);
    }

    impl<Tail> CharList for HCons<char, Tail>
    where
        Tail: CharList + Debug,
    {
        fn char_list(&self) {
            println!("Char list: {:?}", self)
        }
    }

    impl CharList for HNil {
        fn char_list(&self) {}
    }

    /// Simple example trait for a `Hlist![&str, ...]` type
    pub trait StringSliceList {
        fn string_list(&self);
    }

    impl<Tail> StringSliceList for HCons<&str, Tail>
    where
        Tail: StringSliceList + Debug,
    {
        fn string_list(&self) {
            println!("String slice list: {:?}", self)
        }
    }

    impl StringSliceList for HNil {
        fn string_list(&self) {}
    }

    // `HTree` isn't implemented for concrete `HList` types, so they can be used as an entrypoint to break out a `HTree` subtree
    // Concrete types are given as the `C` parameter of `HTree` and head parameter of HCons to avoid conflict with the blanket implementation
    // Traits are used as self predicates so execution can be directed away from recursion and into business logic

    /// Example trait for breaking `Hlist![i32, ...]` types out of a `HTree` structure
    impl<Tail> HTree<()> for HCons<i32, Tail>
    where
        Self: IntList,
        Tail: HTree<()>,
    {
        type Output = ();

        fn recurse(&self, _: ()) {
            self.int_list();
        }
    }

    /// Example trait for breaking `Hlist![f32, ...]` types out of a `HTree` structure
    impl<Tail> HTree<()> for HCons<f32, Tail>
    where
        Self: FloatList,
        Tail: HTree<()>,
    {
        type Output = ();

        fn recurse(&self, _: ()) {
            self.float_list();
        }
    }

    /// Example trait for breaking `Hlist![char, ...]` types out of a `HTree` structure
    impl<Tail> HTree<()> for HCons<char, Tail>
    where
        Self: CharList,
        Tail: HTree<()>,
    {
        type Output = ();

        fn recurse(&self, _: ()) {
            self.char_list();
        }
    }

    /// Example trait for breaking `Hlist![&str, ...]` types out of a `HTree` structure
    impl<Tail> HTree<()> for HCons<&str, Tail>
    where
        Self: StringSliceList,
        Tail: HTree<()>,
    {
        type Output = ();

        fn recurse(&self, _: ()) {
            self.string_list();
        }
    }

    #[test]
    fn htree() {
        let tree = hlist![
            hlist![hlist![1, 2, 3], hlist![4.0, 5.0, 6.0]],
            hlist![hlist!['7', '8', '9']],
            hlist![hlist![10.0, 11.0, 12.0], hlist!["13", "14", "15"]]
        ];

        let _proof: &dyn HTree<_, Output = _> = &tree;

        tree.recurse(());
    }
}
