use frunk_core::{
    hlist::{HCons, HNil},
    labelled::{field_with_name, Field},
};

/// `Hlist` of `Field` types whose names can be mapped over
pub trait FieldNameMappableList<'a, NamesMapper> {
    type MapNamesOutput;

    fn list_map_field_names(&self, mapper: NamesMapper) -> Self::MapNamesOutput;
}

impl<'a, NamesMapper, N, T, Tail, R> FieldNameMappableList<'a, NamesMapper>
    for HCons<Field<N, T>, Tail>
where
    NamesMapper: Fn(&'static str) -> R,
    Tail: FieldNameMappableList<'a, NamesMapper>,
{
    type MapNamesOutput = HCons<Field<N, R>, Tail::MapNamesOutput>;

    fn list_map_field_names(&self, mapper: NamesMapper) -> Self::MapNamesOutput {
        HCons {
            head: field_with_name(self.head.name, mapper(self.head.name)),
            tail: self.tail.list_map_field_names(mapper),
        }
    }
}

impl<'a, NM> FieldNameMappableList<'a, NM> for HNil {
    type MapNamesOutput = HNil;

    fn list_map_field_names(&self, _: NM) -> Self::MapNamesOutput {
        HNil
    }
}

/// `Hlist` of `Field` types whose values can be mapped over
pub trait FieldValueMappableList<'a, ValuesMapper> {
    type MapValuesOutput;

    fn list_map_field_values(&'a self, mapper: ValuesMapper) -> Self::MapValuesOutput;
}

impl<'a, ValuesMapper, N, T, Tail, R> FieldValueMappableList<'a, ValuesMapper>
    for HCons<Field<N, T>, Tail>
where
    ValuesMapper: Fn(&T) -> R,
    Tail: FieldValueMappableList<'a, ValuesMapper>,
{
    type MapValuesOutput = HCons<Field<N, R>, Tail::MapValuesOutput>;

    fn list_map_field_values(&'a self, mapper: ValuesMapper) -> Self::MapValuesOutput {
        HCons {
            head: field_with_name(self.head.name, mapper(&self.head.value)),
            tail: self.tail.list_map_field_values(mapper),
        }
    }
}

impl<'a, VM> FieldValueMappableList<'a, VM> for HNil {
    type MapValuesOutput = HNil;

    fn list_map_field_values(&self, _: VM) -> Self::MapValuesOutput {
        HNil
    }
}

#[cfg(test)]
mod tests {
    use frunk_core::{field, hlist};

    use super::*;

    #[test]
    fn map_field_names() {
        let mappable = hlist![field!(i32, 1), field!(i32, 2), field!(i32, 3)];
        let names_mapped = mappable.list_map_field_names(|name: &'static str| name.to_string());
        println!("\nNames mapped:\n{:#?}", names_mapped);
    }

    #[test]
    fn map_field_values() {
        let mappable = hlist![field!(i32, 1), field!(i32, 2), field!(i32, 3)];
        let values_mapped = mappable.list_map_field_values(|name: &i32| name.to_string());
        println!("\nValues mapped:\n{:#?}", values_mapped);
    }
}
