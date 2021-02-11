//! Traits for traversing a `HList` in parallel using `rayon`

use frunk_core::{
    hlist::{HCons, HNil},
    traits::{Func, Poly},
};
use rayon::join;

pub trait HJoinMappable<Mapper> {
    type Output;

    fn map_join_lifo(self, f: Mapper) -> Self::Output;
    fn map_join_fifo(self, f: Mapper) -> Self::Output;
}

impl<F, R, Head, Tail> HJoinMappable<F> for HCons<Head, Tail>
where
    Head: Send + Sync,
    Tail: HJoinMappable<F> + Send + Sync,
    F: Fn(Head) -> R + Send + Sync + Copy,
    R: Send + Sync,
    <Tail as HJoinMappable<F>>::Output: Send + Sync,
{
    type Output = HCons<R, <Tail as HJoinMappable<F>>::Output>;

    fn map_join_lifo(self, f: F) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (tail_result, head_result) = join(|| tail.map_join_lifo(f), || f(head));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }

    fn map_join_fifo(self, f: F) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (head_result, tail_result) = join(|| f(head), || tail.map_join_fifo(f));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }
}

impl<F, R, MapperTail, Head, Tail> HJoinMappable<HCons<F, MapperTail>>
    for HCons<Head, Tail>
where
    Head: Send + Sync,
    Tail: HJoinMappable<MapperTail> + Send + Sync,
    <Tail as HJoinMappable<MapperTail>>::Output: Send + Sync,
    MapperTail: Send + Sync,
    F: Fn(Head) -> R + Send + Sync + Copy,
    R: Send + Sync,
{
    type Output = HCons<R, <Tail as HJoinMappable<MapperTail>>::Output>;

    fn map_join_lifo(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (mapper_head, mapper_tail) = (mapper.head, mapper.tail);
        let (tail_result, head_result) =
            join(|| tail.map_join_lifo(mapper_tail), || (mapper_head)(head));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }

    fn map_join_fifo(self, mapper: HCons<F, MapperTail>) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (mapper_head, mapper_tail) = (mapper.head, mapper.tail);
        let (head_result, tail_result) =
            join(|| (mapper_head)(head), || tail.map_join_fifo(mapper_tail));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }
}

impl<P, R, Head, Tail> HJoinMappable<Poly<P>> for HCons<Head, Tail>
where
    Head: Send + Sync,
    P: Func<Head, Output = R> + Send + Sync,
    R: Send + Sync,
    Tail: HJoinMappable<Poly<P>> + Send + Sync,
    <Tail as HJoinMappable<Poly<P>>>::Output: Send + Sync,
{
    type Output = HCons<R, <Tail as HJoinMappable<Poly<P>>>::Output>;

    fn map_join_lifo(self, f: Poly<P>) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (tail_result, head_result) = join(|| tail.map_join_lifo(f), || P::call(head));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }

    fn map_join_fifo(self, f: Poly<P>) -> Self::Output {
        let (head, tail) = (self.head, self.tail);
        let (head_result, tail_result) = join(|| P::call(head), || tail.map_join_fifo(f));

        HCons {
            head: head_result,
            tail: tail_result,
        }
    }
}

impl<Mapper> HJoinMappable<Mapper> for HNil {
    type Output = HNil;

    fn map_join_lifo(self, _: Mapper) -> Self::Output {
        HNil
    }

    fn map_join_fifo(self, _: Mapper) -> Self::Output {
        HNil
    }
}
