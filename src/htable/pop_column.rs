use frunk_core::hlist::{HCons, HNil};

/// `HList` of `HList`s that can have their head elements popped into a new `HList`
pub trait PopColumn {
    type Column;
    type Remainder;

    fn pop_column(self) -> (Self::Column, Self::Remainder);
}

impl<InnerHead, InnerTail, Tail> PopColumn for HCons<HCons<InnerHead, InnerTail>, Tail>
where
    Tail: PopColumn,
{
    type Column = HCons<InnerHead, Tail::Column>;
    type Remainder = HCons<InnerTail, Tail::Remainder>;

    fn pop_column(self) -> (Self::Column, Self::Remainder) {
        let (head_head, head_tail) = self.head.pop();
        let (tail_head, tail_tail) = self.tail.pop_column();

        (
            HCons {
                head: head_head,
                tail: tail_head,
            },
            HCons {
                head: head_tail,
                tail: tail_tail,
            },
        )
    }
}

impl<InnerHead, InnerTail, Tail> PopColumn for HCons<&HCons<InnerHead, InnerTail>, Tail>
where
    HCons<InnerHead, InnerTail>: Copy,
    Tail: PopColumn,
{
    type Column = HCons<InnerHead, Tail::Column>;
    type Remainder = HCons<InnerTail, Tail::Remainder>;

    fn pop_column(self) -> (Self::Column, Self::Remainder) {
        let (head_head, head_tail) = self.head.pop();
        let (tail_head, tail_tail) = self.tail.pop_column();

        (
            HCons {
                head: head_head,
                tail: tail_head,
            },
            HCons {
                head: head_tail,
                tail: tail_tail,
            },
        )
    }
}

impl PopColumn for HNil {
    type Column = HNil;
    type Remainder = HNil;

    fn pop_column(self) -> (Self::Column, Self::Remainder) {
        (HNil, HNil)
    }
}

#[cfg(test)]
mod tests {
    use frunk_core::hlist;

    use super::PopColumn;

    #[test]
    fn test_pop_column() {
        let table = hlist![hlist![1, 2, 3], hlist![4, 5, 6], hlist![7, 8, 9]];
        let (_column, table) = table.pop_column();
        let (_column, table) = table.pop_column();
        let (_column, _table) = table.pop_column();
    }
}
