// TODO: How to do this returning &[I::Item] instead of Vec<I::Item>?

pub struct Combinations<I>
where I: Iterator {
    items: Vec<(I, I::Item)>,
    finished: bool,
}

impl<I> Combinations<I>
where I: Iterator, I: Clone {
    pub fn new(mut iter: I, r: usize) -> Self {
        let mut items = Vec::with_capacity(r);

        for _ in 0..r {
            let Some(value) = iter.next() else {
                return Self {
                    items: vec![],
                    finished: false,
                };
            };
            items.push((iter.clone(), value));
        }

        Self {
            items,
            finished: false,
        }
    }
}

impl<I> Iterator for Combinations<I>
where I: Iterator, I: Clone, I::Item: Clone {
    type Item = Vec<I::Item>;

    // TODO: Is there a way to calculate size_hint()?

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let res = self.items.iter().map(|(_, v)| v.clone()).collect();

        if self.items.is_empty() {
            self.finished = true;
        } else {
            /*
             * The following could be just this, but that has some unnecessary
             * clones of the iterator and checks the index == 0 potentially
             * multiple times.

            'outer: for index in (0..self.items.len()).rev() {
                let mut iter = self.items[index].0.clone();

                for tail_index in index..self.items.len() {
                    let Some(value) = iter.next() else {
                        if index == 0 {
                            self.finished = true;
                            break;
                        }
                        continue 'outer;
                    };
                    self.items[tail_index] = (iter.clone(), value);
                }

                break;
            }
             */

            'outer: for index in (1..self.items.len()).rev() {
                let mut iter = self.items[index].0.clone();

                for tail_index in index..(self.items.len() - 1) {
                    let Some(value) = iter.next() else {
                        continue 'outer;
                    };
                    self.items[tail_index] = (iter.clone(), value);
                }

                if let Some(value) = iter.next() {
                    // eliminate one unnecessary clone
                    *self.items.last_mut().unwrap() = (iter, value);
                } else {
                    continue 'outer;
                }

                return Some(res);
            }

            let mut iter = self.items[0].0.clone();

            for tail_index in 0..(self.items.len() - 1) {
                let Some(value) = iter.next() else {
                    self.finished = true;
                    break;
                };
                self.items[tail_index] = (iter.clone(), value);
            }

            if let Some(value) = iter.next() {
                // eliminate one unnecessary clone
                *self.items.last_mut().unwrap() = (iter, value);
            } else {
                self.finished = true;
            }
        }

        Some(res)
    }
}

#[inline]
pub fn combinations<I>(iter: I, r: usize) -> Combinations<I>
where I: Iterator, I: Clone {
    Combinations::new(iter, r)
}
