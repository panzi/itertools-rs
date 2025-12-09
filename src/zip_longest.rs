pub struct ZipLongest<F, S> {
    first: F,
    second: S,
}

impl<F, S> ZipLongest<F, S> {
    #[inline]
    pub fn new(first: F, second: S) -> Self {
        Self { first, second }
    }
}

impl<F, S> Clone for ZipLongest<F, S>
where F: Clone, S: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            first:  self.first.clone(),
            second: self.second.clone(),
        }
    }
}

impl<F, S> Iterator for ZipLongest<F, S>
where F: Iterator, S: Iterator,
      F::Item: Default, S::Item: Default {
    type Item = (F::Item, S::Item);

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (first_lower, first_upper) = self.first.size_hint();
        let (second_lower, second_upper) = self.second.size_hint();
        let lower = first_lower.max(second_lower);
        let upper = match (first_upper, second_upper) {
            (Some(first_upper), Some(second_upper)) => Some(first_upper.max(second_upper)),
            _ => None,
        };

        (lower, upper)
    }

    fn count(self) -> usize {
        self.first.count().max(self.second.count())
    }

    fn next(&mut self) -> Option<Self::Item> {
        match (self.first.next(), self.second.next()) {
            (Some(first), Some(second)) => Some((first, second)),
            (Some(first), None) => Some((first, S::Item::default())),
            (None, Some(second)) => Some((F::Item::default(), second)),
            (None, None) => None,
        }
    }
}

#[inline]
pub fn zip_longest<F, S>(first: F, second: S) -> ZipLongest<F, S> {
    ZipLongest::new(first, second)
}

pub trait ZippableLongest: Iterator + Sized {
    #[inline]
    fn zip_longest<Iter: Iterator>(self, iter: Iter) -> ZipLongest<Self, Iter> {
        ZipLongest::new(self, iter)
    }
}

impl<I> ZippableLongest for I where I: Iterator {}
