#[derive(Debug)]
pub struct Pairwise<I>
where I: Iterator {
    iter: I,
    item: Option<I::Item>,
}

impl<I> Pairwise<I>
where I: Iterator {
    #[inline]
    pub fn new(mut iter: I) -> Self {
        Self {
            item: iter.next(),
            iter,
        }
    }
}

impl<I> Clone for Pairwise<I>
where I: Iterator, I: Clone, I::Item: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            item: self.item.clone(),
            iter: self.iter.clone(),
        }
    }
}

impl<I> Iterator for Pairwise<I>
where I: Iterator, I::Item: Clone {
    type Item = (I::Item, I::Item);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.item.is_none() {
            return (0, Some(0));
        }

        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        if self.item.is_none() {
            return 0;
        }

        self.iter.count()
    }

    fn next(&mut self) -> Option<Self::Item> {
        let Some(first) = &self.item else {
            return None;
        };
        let first = first.clone();
        self.item = self.iter.next();

        let Some(second) = &self.item else {
            self.item = None;
            return None;
        };
        let second = second.clone();

        Some((first, second))
    }
}

#[inline]
pub fn pairwise<I>(iter: I) -> Pairwise<I>
where I: Iterator {
    Pairwise::new(iter)
}

pub trait Pairwiseable: Iterator
where Self: Sized {
    #[inline]
    fn pairwise(self) -> Pairwise<Self> {
        Pairwise::new(self)
    }
}

impl<I> Pairwiseable for I where I: Iterator {}
