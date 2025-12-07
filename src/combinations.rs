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
                    finished: true,
                };
            };
            items.push((iter.clone(), value));
        }

        Self {
            items,
            finished: r == 0,
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

        'outer: for index in (0..self.items.len()).rev() {
            // TODO: optimize clones!
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

        Some(res)
    }
}

#[inline]
pub fn combinations<I>(iter: I, r: usize) -> Combinations<I>
where I: Iterator, I: Clone {
    Combinations::new(iter, r)
}
