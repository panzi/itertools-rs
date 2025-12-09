use std::ops::Add;

#[derive(Debug)]
pub struct Accumulate<I>
where I: Iterator {
    iter: I,
    value: I::Item,
}

impl<I> Clone for Accumulate<I>
where I: Iterator, I: Clone, I::Item: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            value: self.value.clone(),
        }
    }
}

impl<I> Accumulate<I>
where I: Iterator, I::Item: Default {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            value: I::Item::default(),
        }
    }
}

impl<I> Iterator for Accumulate<I>
where I: Iterator, I::Item: Add<Output = I::Item>, I::Item: Clone {
    type Item = I::Item;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let Some(value) = self.iter.next() else {
            return None;
        };

        self.value = self.value.clone() + value;

        Some(self.value.clone())
    }
}

#[inline]
pub fn accumulate<I>(iter: I) -> Accumulate<I>
where I: Iterator, I::Item: Default {
    Accumulate::new(iter)
}

pub trait Accumulatable: Iterator + Sized {
    #[inline]
    fn accumulate(self) -> Accumulate<Self>
    where Self::Item: Default {
        Accumulate::new(self)
    }
}

impl<I> Accumulatable for I where I: Iterator + Sized {}
