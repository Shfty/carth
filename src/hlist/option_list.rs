use frunk_core::hlist::{HCons, HNil};

/// A `HList` of `Option` types
pub trait OptionList {
    type UnwrapOutput;

    fn any_some(&self) -> bool;
    fn any_none(&self) -> bool;
    fn all_some(&self) -> bool;
    fn all_none(&self) -> bool;
    fn unwrap(self) -> Self::UnwrapOutput;
}

impl<'a, T, Tail> OptionList for HCons<Option<T>, Tail>
where
    Tail: OptionList,
{
    type UnwrapOutput = HCons<T, Tail::UnwrapOutput>;

    fn any_some(&self) -> bool {
        self.head.is_some() || self.tail.any_some()
    }

    fn any_none(&self) -> bool {
        self.head.is_none() || self.tail.any_none()
    }

    fn all_some(&self) -> bool {
        self.head.is_some() && self.tail.all_some()
    }

    fn all_none(&self) -> bool {
        self.head.is_none() && self.tail.all_none()
    }

    fn unwrap(self) -> Self::UnwrapOutput {
        HCons {
            head: self.head.unwrap(),
            tail: self.tail.unwrap(),
        }
    }
}

impl OptionList for HNil {
    type UnwrapOutput = HNil;

    fn any_some(&self) -> bool {
        false
    }

    fn any_none(&self) -> bool {
        false
    }

    fn all_some(&self) -> bool {
        true
    }

    fn all_none(&self) -> bool {
        true
    }

    fn unwrap(self) -> Self::UnwrapOutput {
        HNil
    }
}
