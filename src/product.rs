use std::iter::Iterator;

#[derive(Debug)]
pub struct Product<F, S>
where F: Iterator {
    first: F,
    first_current: Option<F::Item>,
    second: S,
    second_clone: S,
}

impl<F, S> Product<F, S>
where F: Iterator, S: Clone {
    #[inline]
    pub fn new(first: F, second: S) -> Self {
        Self {
            first,
            first_current: None,
            second_clone: second.clone(),
            second,
        }
    }
}

impl<F, S> Clone for Product<F, S>
where F: Iterator, F: Clone, F::Item: Clone, S: Clone {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            first: self.first.clone(),
            first_current: self.first_current.clone(),
            second: self.second.clone(),
            second_clone: self.second_clone.clone(),
        }
    }
}

impl<F, S> Iterator for Product<F, S>
where F: Iterator, F::Item: Clone, S: Iterator, S: Clone {
    type Item = (F::Item, S::Item);

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (first_lower, first_upper) = self.first.size_hint();
        let (second_lower, second_upper) = self.second.size_hint();

        if self.first_current.is_some() {
            let (second_clone_lower, second_clone_upper) = self.second_clone.size_hint();
            let lower = second_lower.saturating_add(first_lower.saturating_mul(second_clone_lower));
            let upper = match (first_upper, second_upper, second_clone_upper) {
                (Some(first_upper), Some(second_upper), Some(second_clone_upper)) => {
                    first_upper
                        .checked_mul(second_clone_upper)
                        .and_then(|val| val.checked_add(second_upper))
                },
                _ => None,
            };
            (lower, upper)
        } else {
            let lower = first_lower.saturating_mul(second_lower);
            let upper = match (first_upper, second_upper) {
                (Some(first_upper), Some(second_upper)) =>
                    first_upper.checked_mul(second_upper),
                _ => None,
            };
            (lower, upper)
        }
    }

    fn count(self) -> usize {
        let first_count = self.first.count();
        let second_count = self.second.count();

        if self.first_current.is_some() {
            let second_clone_count = self.second_clone.count();

            first_count
                .checked_mul(second_clone_count)
                .and_then(|val| val.checked_add(second_count))
        } else {
            first_count.checked_mul(second_count)
        }.expect("usize overflow")
    }

    fn next(&mut self) -> Option<Self::Item> {
        let second_value = if let Some(second_value) = self.second.next() {
            second_value
        } else {
            self.second = self.second_clone.clone();
            if let Some(second_value) = self.second.next() {
                self.first_current = None;
                second_value
            } else {
                return None;
            }
        };

        if self.first_current.is_none() {
            self.first_current = self.first.next();
        }

        let Some(first_value) = self.first_current.clone() else {
            return None;
        };

        Some((first_value, second_value))
    }
}

#[inline]
pub fn product<F, S>(first: F, second: S) -> Product<F, S>
where F: Iterator, F::Item: Clone, S: Clone {
    Product::new(first, second)
}

// The following are generated with make_products.py

pub trait Productable: Iterator + Sized
where Self::Item: Clone {
    #[inline]
    fn product2<I2>(self, i2: I2) -> Product<Self, I2>
    where I2: Iterator + Clone {
        Product::new(self, i2)
    }

    #[inline]
    fn product3<I2, I3>(self, i2: I2, i3: I3) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item)>
    where I2: Iterator, I3: Iterator,
          I2: Clone, I3: Clone,
          I2::Item: Clone {
        product3(self, i2, i3)
    }

    #[inline]
    fn product4<I2, I3, I4>(self, i2: I2, i3: I3, i4: I4) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator,
          I2: Clone, I3: Clone, I4: Clone,
          I2::Item: Clone, I3::Item: Clone {
        product4(self, i2, i3, i4)
    }

    #[inline]
    fn product5<I2, I3, I4, I5>(self, i2: I2, i3: I3, i4: I4, i5: I5) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone {
        product5(self, i2, i3, i4, i5)
    }

    #[inline]
    fn product6<I2, I3, I4, I5, I6>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone {
        product6(self, i2, i3, i4, i5, i6)
    }

    #[inline]
    fn product7<I2, I3, I4, I5, I6, I7>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone {
        product7(self, i2, i3, i4, i5, i6, i7)
    }

    #[inline]
    fn product8<I2, I3, I4, I5, I6, I7, I8>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone {
        product8(self, i2, i3, i4, i5, i6, i7, i8)
    }

    #[inline]
    fn product9<I2, I3, I4, I5, I6, I7, I8, I9>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone {
        product9(self, i2, i3, i4, i5, i6, i7, i8, i9)
    }

    #[inline]
    fn product10<I2, I3, I4, I5, I6, I7, I8, I9, I10>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone {
        product10(self, i2, i3, i4, i5, i6, i7, i8, i9, i10)
    }

    #[inline]
    fn product11<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone {
        product11(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11)
    }

    #[inline]
    fn product12<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone {
        product12(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12)
    }

    #[inline]
    fn product13<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone {
        product13(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13)
    }

    #[inline]
    fn product14<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone {
        product14(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14)
    }

    #[inline]
    fn product15<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14, I15>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14, i15: I15) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item, I15::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator, I15: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone, I15: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone, I14::Item: Clone {
        product15(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15)
    }

    #[inline]
    fn product16<I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14, I15, I16>(self, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14, i15: I15, i16: I16) -> impl Iterator<Item = (Self::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item, I15::Item, I16::Item)>
    where I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator, I15: Iterator, I16: Iterator,
          I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone, I15: Clone, I16: Clone,
          I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone, I14::Item: Clone, I15::Item: Clone {
        product16(self, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15, i16)
    }
}

impl<I> Productable for I where I: Iterator + Sized, I::Item: Clone {}

#[inline]
pub fn product3<I1, I2, I3>(i1: I1, i2: I2, i3: I3) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator,
      I2: Clone, I3: Clone,
      I1::Item: Clone, I2::Item: Clone {
    Product::new(Product::new(i1, i2), i3).map(|((v1, v2), v3)| (v1, v2, v3))
}

#[inline]
pub fn product4<I1, I2, I3, I4>(i1: I1, i2: I2, i3: I3, i4: I4) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator,
      I2: Clone, I3: Clone, I4: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone {
    Product::new(Product::new(Product::new(i1, i2), i3), i4).map(|(((v1, v2), v3), v4)| (v1, v2, v3, v4))
}

#[inline]
pub fn product5<I1, I2, I3, I4, I5>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5).map(|((((v1, v2), v3), v4), v5)| (v1, v2, v3, v4, v5))
}

#[inline]
pub fn product6<I1, I2, I3, I4, I5, I6>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6).map(|(((((v1, v2), v3), v4), v5), v6)| (v1, v2, v3, v4, v5, v6))
}

#[inline]
pub fn product7<I1, I2, I3, I4, I5, I6, I7>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7).map(|((((((v1, v2), v3), v4), v5), v6), v7)| (v1, v2, v3, v4, v5, v6, v7))
}

#[inline]
pub fn product8<I1, I2, I3, I4, I5, I6, I7, I8>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8).map(|(((((((v1, v2), v3), v4), v5), v6), v7), v8)| (v1, v2, v3, v4, v5, v6, v7, v8))
}

#[inline]
pub fn product9<I1, I2, I3, I4, I5, I6, I7, I8, I9>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9).map(|((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9)| (v1, v2, v3, v4, v5, v6, v7, v8, v9))
}

#[inline]
pub fn product10<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10).map(|(((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
}

#[inline]
pub fn product11<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11).map(|((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11))
}

#[inline]
pub fn product12<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11), i12).map(|(((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11), v12)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12))
}

#[inline]
pub fn product13<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11), i12), i13).map(|((((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11), v12), v13)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13))
}

#[inline]
pub fn product14<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11), i12), i13), i14).map(|(((((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11), v12), v13), v14)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14))
}

#[inline]
pub fn product15<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14, I15>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14, i15: I15) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item, I15::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator, I15: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone, I15: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone, I14::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11), i12), i13), i14), i15).map(|((((((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11), v12), v13), v14), v15)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15))
}

#[inline]
pub fn product16<I1, I2, I3, I4, I5, I6, I7, I8, I9, I10, I11, I12, I13, I14, I15, I16>(i1: I1, i2: I2, i3: I3, i4: I4, i5: I5, i6: I6, i7: I7, i8: I8, i9: I9, i10: I10, i11: I11, i12: I12, i13: I13, i14: I14, i15: I15, i16: I16) -> impl Iterator<Item = (I1::Item, I2::Item, I3::Item, I4::Item, I5::Item, I6::Item, I7::Item, I8::Item, I9::Item, I10::Item, I11::Item, I12::Item, I13::Item, I14::Item, I15::Item, I16::Item)>
where I1: Iterator, I2: Iterator, I3: Iterator, I4: Iterator, I5: Iterator, I6: Iterator, I7: Iterator, I8: Iterator, I9: Iterator, I10: Iterator, I11: Iterator, I12: Iterator, I13: Iterator, I14: Iterator, I15: Iterator, I16: Iterator,
      I2: Clone, I3: Clone, I4: Clone, I5: Clone, I6: Clone, I7: Clone, I8: Clone, I9: Clone, I10: Clone, I11: Clone, I12: Clone, I13: Clone, I14: Clone, I15: Clone, I16: Clone,
      I1::Item: Clone, I2::Item: Clone, I3::Item: Clone, I4::Item: Clone, I5::Item: Clone, I6::Item: Clone, I7::Item: Clone, I8::Item: Clone, I9::Item: Clone, I10::Item: Clone, I11::Item: Clone, I12::Item: Clone, I13::Item: Clone, I14::Item: Clone, I15::Item: Clone {
    Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(Product::new(i1, i2), i3), i4), i5), i6), i7), i8), i9), i10), i11), i12), i13), i14), i15), i16).map(|(((((((((((((((v1, v2), v3), v4), v5), v6), v7), v8), v9), v10), v11), v12), v13), v14), v15), v16)| (v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16))
}
