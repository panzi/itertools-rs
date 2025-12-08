use crate::Combinations;

pub struct Powerset<I>
where I: Iterator {
    iter: I,
    combinations: Combinations<I>,
    r: usize,
    finished: bool,
}

impl<I> Powerset<I>
where I: Iterator, I: Clone {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            combinations: Combinations::new(iter.clone(), 0),
            iter,
            r: 0,
            finished: false,
        }
    }
}

impl<I> Clone for Powerset<I>
where I: Iterator, I: Clone, I::Item: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            combinations: self.combinations.clone(),
            r: self.r,
            finished: self.finished,
        }
    }
}

impl<I> Iterator for Powerset<I>
where I: Iterator, I: Clone, I::Item: Clone {
    type Item = Vec<I::Item>;

    // TODO: Is there a way to calculate size_hint()?

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        if let Some(value) = self.combinations.next() {
            return Some(value);
        }

        self.r += 1;
        self.combinations = Combinations::new(self.iter.clone(), self.r);

        let value = self.combinations.next();
        self.finished = value.is_none();

        value
    }
}

#[inline]
pub fn powerset<I>(iter: I) -> Powerset<I>
where I: Iterator, I: Clone {
    Powerset::new(iter)
}

pub trait Powersetable: Iterator
where Self: Sized + Clone {
    #[inline]
    fn powerset(self) -> Powerset<Self> {
        Powerset::new(self)
    }
}

impl<I> Powersetable for I where I: Iterator + Clone {}
