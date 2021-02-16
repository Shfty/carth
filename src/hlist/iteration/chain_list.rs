use frunk_core::hlist::{HCons, HNil};

pub trait ChainList<T> {
    type Output: Iterator<Item = T>;
    fn list_chain(self) -> Self::Output;
}

impl<T, Head, Tail> ChainList<T> for HCons<Head, Tail>
where
    Head: Iterator<Item = T>,
    Tail: ChainList<T>,
{
    type Output = std::iter::Chain<Head, Tail::Output>;

    fn list_chain(self) -> Self::Output {
        self.head.chain(self.tail.list_chain())
    }
}

impl<T> ChainList<T> for HNil {
    type Output = std::iter::Empty<T>;

    fn list_chain(self) -> Self::Output {
        std::iter::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use frunk_core::hlist;

    #[test]
    fn test_chain_list() {
        let chain_list = hlist![
            0..3, [2, 6, 8].iter().copied(), 6..7
        ];

        let chained = ChainList::list_chain(chain_list);
        assert!(chained.collect::<Vec<i32>>() == vec![0, 1, 2, 2, 6, 8, 6]);
    }
}