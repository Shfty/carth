use frunk_core::hlist::HFoldLeftable;

use super::HomogenousList;

/// Homogenous `HList` that can be converted into a Vec
pub trait ToVecList<T>: HomogenousList<T> {
    fn list_to_vec(self) -> Vec<T>;
}

impl<HL, T> ToVecList<T> for HL
where
    HL: HomogenousList<T> + HFoldLeftable<fn(Vec<T>, T) -> Vec<T>, Vec<T>, Output = Vec<T>>,
{
    fn list_to_vec(self) -> Vec<T> {
        self.foldl(
            |mut acc: Vec<T>, next| {
                acc.push(next);
                acc
            },
            vec![],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk_core::hlist;

    #[test]
    fn to_vec() {
        let list = hlist![1, 2, 3, 4, 5, 6];
        let vec = list.list_to_vec();
        println!("Vec: {:#?}", vec);
    }
}
