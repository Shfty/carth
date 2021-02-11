use frunk_core::{
    hlist::{HCons, HNil},
    labelled::Field,
};

/// A `HList` of `Field` types
pub trait FieldList<'a> {
    type NameTypes;
    type ValueTypes;

    type NamesOutput;
    type ValuesOutput;

    fn names(&self) -> Self::NamesOutput;
    fn values(&'a self) -> Self::ValuesOutput;
}

impl<'a, N, T, Tail> FieldList<'a> for HCons<Field<N, T>, Tail>
where
    T: 'a,
    Tail: FieldList<'a>,
{
    type NameTypes = HCons<N, Tail::NameTypes>;
    type ValueTypes = HCons<T, Tail::ValueTypes>;

    type NamesOutput = HCons<&'static str, Tail::NamesOutput>;
    type ValuesOutput = HCons<&'a T, Tail::ValuesOutput>;

    fn names(&self) -> Self::NamesOutput {
        HCons {
            head: self.head.name,
            tail: self.tail.names(),
        }
    }

    fn values(&'a self) -> Self::ValuesOutput {
        HCons {
            head: &self.head.value,
            tail: self.tail.values(),
        }
    }
}

impl<'a> FieldList<'a> for HNil {
    type NameTypes = HNil;
    type ValueTypes = HNil;

    type NamesOutput = HNil;
    type ValuesOutput = HNil;

    fn names(&self) -> Self::NamesOutput {
        HNil
    }

    fn values(&self) -> Self::ValuesOutput {
        HNil
    }
}
