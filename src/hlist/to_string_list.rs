use frunk_core::hlist::{HCons, HNil};

/// `HList` of `ToString` types
pub trait ToStringList {
    type Output;

    fn list_to_string(&self) -> Self::Output;
}

impl<Head, Tail> ToStringList for HCons<Head, Tail>
where
    Head: ToString,
    Tail: ToStringList,
{
    type Output = HCons<String, Tail::Output>;

    fn list_to_string(&self) -> Self::Output {
        HCons {
            head: self.head.to_string(),
            tail: self.tail.list_to_string(),
        }
    }
}

impl ToStringList for HNil {
    type Output = HNil;

    fn list_to_string(&self) -> Self::Output {
        HNil
    }
}
