use frunk_core::{
    hlist::{HCons, HNil},
    indices::There,
};

use crate::hlist::HListLength;

use super::pop_column::PopColumn;

/// A `HList` of `PopColumn` types that can be transposed
pub trait Transpose<Width> {
    type Output;

    fn transpose(self) -> Self::Output;
}

impl<Width, Head, Tail> Transpose<There<Width>> for HCons<Head, Tail>
where
    Head: HListLength<Width>,
    Self: PopColumn,
    <Self as PopColumn>::Remainder: Transpose<Width>,
{
    type Output = HCons<
        <Self as PopColumn>::Column,
        <<Self as PopColumn>::Remainder as Transpose<Width>>::Output,
    >;

    fn transpose(self) -> Self::Output {
        let (head, tail) = self.pop_column();

        HCons {
            head,
            tail: tail.transpose(),
        }
    }
}

impl<Width, Tail> Transpose<Width> for HCons<HNil, Tail> {
    type Output = HNil;

    fn transpose(self) -> Self::Output {
        HNil
    }
}

impl<Width> Transpose<Width> for HNil {
    type Output = HNil;

    fn transpose(self) -> Self::Output {
        HNil
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use frunk_core::{hlist, indices::Here};

    #[test]
    fn test_transpose_identity() {
        let identity = hlist![];
        let transposed = Transpose::<Here>::transpose(identity);
        assert!(transposed == hlist![]);
    }

    #[test]
    fn test_transpose_3x2() {
        let identity = hlist![hlist![1, 2, 3], hlist![4, 5, 6]];

        let transposed = identity.transpose();
        assert!(transposed == hlist![hlist![1, 4], hlist![2, 5], hlist![3, 6]]);

        let transposed = transposed.transpose();
        assert!(transposed == hlist![hlist![1, 2, 3], hlist![4, 5, 6]]);
    }

    #[test]
    fn test_transpose_3x2_ref() {
        let identity = hlist![hlist![&1, &2, &3], hlist![&4, &5, &6]];

        let transposed = identity.transpose();
        assert!(transposed == hlist![hlist![&1, &4], hlist![&2, &5], hlist![&3, &6]]);

        let transposed = transposed.transpose();
        assert!(transposed == hlist![hlist![&1, &2, &3], hlist![&4, &5, &6]]);
    }

    #[test]
    fn test_transpose_3x3() {
        let transposable = hlist![
            hlist![1, 2, 3],
            hlist![4.0, 5.0, 6.0],
            hlist!['7', '8', '9']
        ];
        let transposed = transposable.transpose();
        assert!(transposed == hlist![hlist![1, 4.0, '7'], hlist![2, 5.0, '8'], hlist![3, 6.0, '9']]);
    }
}
