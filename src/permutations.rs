#[derive(Debug, Clone)]
pub struct Permutations<'a, T> {
    data: &'a [T],
    permutation: Vec<&'a T>,
    indices: Vec<usize>,
    index: usize,
    count: usize,
}

impl<'a, T> Permutations<'a, T> {
    #[inline]
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            permutation: Vec::with_capacity(data.len()),
            indices: vec![0; data.len()],
            index: 1,
            count: 0,
        }
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size: usize = (1..=self.data.len()).product();
        let remaining = size - self.count;

        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        let size: usize = (1..=self.data.len()).product();
        size - self.count
    }

    fn next(&mut self) -> Option<Self::Item> {
        // Heap's algorithm
        if self.count == 0 {
            self.permutation.extend(self.data.iter());
            self.count = 1;
            Some(self.permutation.clone())
        } else {
            let n = self.data.len();

            while self.index < n {
                if self.indices[self.index] < self.index {
                    if (self.index & 1) == 0 {
                        self.permutation.swap(0, self.index);
                    } else {
                        self.permutation.swap(self.indices[self.index], self.index);
                    }

                    self.indices[self.index] += 1;
                    self.index = 1;
                    self.count += 1;

                    return Some(self.permutation.clone());
                } else {
                    self.indices[self.index] = 0;
                    self.index += 1;
                }
            }

            self.permutation.clear();

            None
        }
    }
}

#[inline]
pub fn permutations<'a, T>(data: &'a [T]) -> Permutations<'a, T> {
    Permutations::new(data)
}

pub trait Permutable<'a> {
    type Item;
    fn permutations(&'a self) -> Permutations<'a, Self::Item>;
}

impl<'a, T> Permutable<'a> for [T] {
    type Item = T;

    #[inline]
    fn permutations(&'a self) -> Permutations<'a, Self::Item> {
        Permutations::new(self)
    }
}

impl<'a, T, const N: usize> Permutable<'a> for [T; N] {
    type Item = T;

    #[inline]
    fn permutations(&'a self) -> Permutations<'a, Self::Item> {
        Permutations::new(self.as_slice())
    }
}
