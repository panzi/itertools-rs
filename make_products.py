#!/usr/bin/env python3

def nest(var: str, n: int, prefix: str = '') -> str:
    if n <= 0:
        raise ValueError(n)
    elif n == 1:
        return f'{var}{n}'
    else:
        return f'{prefix}({nest(var, n - 1, prefix)}, {var}{n})'

print(
f"""\
pub trait Productable: Iterator + Sized
where Self::Item: Clone {{
    #[inline]
    fn product2<I2>(self, i2: I2) -> Product<Self, I2>
    where I2: Iterator + Clone {{
        Product::new(self, i2)
    }}
"""
)

for n in range(3, 17):
    print(
f"""\
    #[inline]
    fn product{n}<{', '.join(f'I{i}' for i in range(2,n+1))}>(self, {', '.join(f'i{i}: I{i}' for i in range(2,n+1))}) -> impl Iterator<Item = (Self::Item, {', '.join(f'I{i}::Item' for i in range(2,n+1))})>
    where {', '.join(f'I{i}: Iterator' for i in range(2,n+1))},
          {', '.join(f'I{i}: Clone' for i in range(2,n+1))},
          {', '.join(f'I{i}::Item: Clone' for i in range(2,n))} {{
        product{n}(self{''.join(f', i{i}' for i in range(2,n+1))})
    }}
""")

print("""\
}

impl<I> Productable for I where I: Iterator + Sized, I::Item: Clone {}
""")

for n in range(3, 17):
    print(
f"""\
#[inline]
pub fn product{n}<{', '.join(f'I{i}' for i in range(1,n+1))}>({', '.join(f'i{i}: I{i}' for i in range(1,n+1))}) -> impl Iterator<Item = ({', '.join(f'I{i}::Item' for i in range(1,n+1))})>
where {', '.join(f'I{i}: Iterator' for i in range(1,n+1))},
      {', '.join(f'I{i}: Clone' for i in range(2,n+1))},
      {', '.join(f'I{i}::Item: Clone' for i in range(1,n))} {{
    {nest('i', n, 'Product::new')}.map(|{nest('v', n)}| ({', '.join(f'v{i}' for i in range(1,n+1))}))
}}
""")
