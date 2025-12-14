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

#[cfg(target_pointer_width = "16")]
#[inline]
fn factorial_usize(n: usize) -> Option<usize> {
    match n {
        0 => Some(1),
        1 => Some(1),
        2 => Some(2),
        3 => Some(6),
        4 => Some(24),
        5 => Some(120),
        6 => Some(720),
        7 => Some(5040),
        8 => Some(40320),
        _ => None
    }
}

#[cfg(target_pointer_width = "32")]
#[inline]
fn factorial_usize(n: usize) -> Option<usize> {
    match n {
        0 => Some(1),
        1 => Some(1),
        2 => Some(2),
        3 => Some(6),
        4 => Some(24),
        5 => Some(120),
        6 => Some(720),
        7 => Some(5040),
        8 => Some(40320),
        9 => Some(362880),
        10 => Some(3628800),
        11 => Some(39916800),
        12 => Some(479001600),
        _ => None
    }
}

#[cfg(target_pointer_width = "64")]
#[inline]
fn factorial_usize(n: usize) -> Option<usize> {
    match n {
        0 => Some(1),
        1 => Some(1),
        2 => Some(2),
        3 => Some(6),
        4 => Some(24),
        5 => Some(120),
        6 => Some(720),
        7 => Some(5040),
        8 => Some(40320),
        9 => Some(362880),
        10 => Some(3628800),
        11 => Some(39916800),
        12 => Some(479001600),
        13 => Some(6227020800),
        14 => Some(87178291200),
        15 => Some(1307674368000),
        16 => Some(20922789888000),
        17 => Some(355687428096000),
        18 => Some(6402373705728000),
        19 => Some(121645100408832000),
        20 => Some(2432902008176640000),
        _ => None
    }
}

#[cfg(not(any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64")))]
#[inline]
fn factorial_usize(n: usize) -> Option<usize> {
    let mut res = 1usize;
    for i in 1..=n {
        res = res.checked_mul(i)?;
    }

    Some(res)
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = factorial_usize(self.data.len());
        match size {
            Some(size) => {
                let remaining = size - self.count;
                (remaining, Some(remaining))
            }
            None => (usize::MAX, None)
        }
    }

    #[inline]
    fn count(self) -> usize {
        let size = factorial_usize(self.data.len()).expect("usize overflow");
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
