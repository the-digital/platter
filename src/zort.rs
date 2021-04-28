use std::iter::Peekable;

type Ranker<T> = fn(&T) -> usize;
type Decider<T> = fn(T, T) -> T;

pub struct Zort<T: Iterator> {
    a: Peekable<T>,
    b: Peekable<T>,
    ranker: Ranker<T::Item>,
    decider: Decider<T::Item>,
    trail: usize,
}

impl<T: Iterator> Zort<T> {
    pub fn new(
        a: T, b: T,
        ranker: Ranker<T::Item>,
        decider: Decider<T::Item>,
        trail: usize,
    ) -> Self {
        Self { a: a.peekable(), b: b.peekable(), ranker, decider, trail }
    }
}

impl<T: Iterator> Iterator for Zort<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.a.peek();
        let b = self.b.peek();
        if a.is_none() && b.is_none() { None }
        else if a.is_none() && self.trail == 2 { self.b.next() }
        else if b.is_none() && self.trail == 1 { self.a.next() }
        else if a.is_none() || b.is_none() {
            if self.trail == 3 { None }
            else if a.is_none() { self.b.next() }
            else { self.a.next() }
        }
        else {
            let a = a.unwrap();
            let b = b.unwrap();
            let rank_a = (self.ranker)(a);
            let rank_b = (self.ranker)(b);
            if rank_a == rank_b {
                Some((self.decider)(
                    self.a.next().unwrap(),
                    self.b.next().unwrap())
                )
            } else if rank_a < rank_b { self.a.next() }
            else { self.b.next() }
        }
    }
}

#[cfg(test)]
mod disjoint {
    use super::Zort;

    fn ranker(value: &&usize) -> usize { **value }
    fn decider<'a>(left: &'a usize, _: &usize) -> &'a usize { left }

    #[test]
    fn basic() {
        let a: Vec<usize> = vec![1, 3, 5, 7, 9];
        let b: Vec<usize> = vec![0, 2, 4, 6, 8];
        let mut c = Zort::new(a.iter(), b.iter(), ranker, decider, 0);
        assert_eq!(c.next(), Some(&0));
        assert_eq!(c.next(), Some(&1));
        assert_eq!(c.next(), Some(&2));
        assert_eq!(c.next(), Some(&3));
        assert_eq!(c.next(), Some(&4));
        assert_eq!(c.next(), Some(&5));
        assert_eq!(c.next(), Some(&6));
        assert_eq!(c.next(), Some(&7));
        assert_eq!(c.next(), Some(&8));
        assert_eq!(c.next(), Some(&9));
    }
}
