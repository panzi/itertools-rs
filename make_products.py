#!/usr/bin/env python3

def nest(var: str, n: int, prefix: str = '') -> str:
    if n <= 0:
        raise ValueError(n)
    elif n == 1:
        return f'{var}{n}'
    else:
        return f'{prefix}({nest(var, n - 1, prefix)}, {var}{n})'

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
