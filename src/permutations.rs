#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Active,
    Finished,
}

#[derive(Debug, Clone)]
pub struct Permutations<'a, T> {
    data: &'a [T],
    permutation: Vec<&'a T>,
    indices: Vec<usize>,
    index: usize,
    state: State,
}

impl<'a, T> Permutations<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        Self {
            data,
            permutation: Vec::with_capacity(data.len()),
            indices: vec![0; data.len()],
            index: 1,
            state: State::Start,
        }
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;

    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     // this is all, but what is just remaining?
    //     let size = (1..=self.data.len()).product();
    // }

    fn next(&mut self) -> Option<Self::Item> {
        // Heap's algorithm
        match self.state {
            State::Start => {
                self.permutation.extend(self.data.iter());
                self.state = State::Active;
                Some(self.permutation.clone())
            }
            State::Active => {
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

                        return Some(self.permutation.clone());
                    } else {
                        self.indices[self.index] = 0;
                        self.index += 1;
                    }
                }

                self.permutation.clear();
                self.state = State::Finished;

                None
            }
            State::Finished => {
                None
            }
        }
    }
}

#[inline]
pub fn permutations<'a, T>(data: &'a [T]) -> Permutations<'a, T> {
    Permutations::new(data)
}
