Let's slow down a little. I think your confusion comes from mixing three separate concepts:

1. `iter()` gives you references.
2. `map()` transforms values.
3. `filter_map()` transforms values and optionally removes them.

Once you understand these three, the difference becomes straightforward.

## 1. First understand what `iter()` gives you

You have:

```
let array: [i32; 18] = [34, 73, 223, 30, 46, 95, 39, 50, 41, 49, 28, 43, 992, 483, 702, 390, 193, 495];
```

When you write:

```
array.iter()
```

You are NOT getting `i32` values directly.

You are getting references:

```
&i32
```

For example:

```
array.iter().for_each(|x| {
    println!("{x}");
});
```

Here:

- `x` is `&i32`.
- `*x` gives you the actual `i32`.

This explains your first error.

You wrote:

```
if x > 2
```

But `x` is a reference, not an integer.

You need:

```
if *x > 2
```

## 2. Understand `map()`

Think of `map()` as a transformation machine.

You give it one value, and it produces another value.

Example: Multiply every element by 3

Input

`2, 4, 6`

map(|x| x \* 3)

`6, 12, 18`

Code:

```
let vec: Vec<i32> = array
    .iter()
    .map(|x| *x * 3)
    .collect();
```

Notice:

- `x` is `&i32`.
- `*x` dereferences it to `i32`.
- `*x * 3` produces `i32`.
- `collect()` gathers those integers into a `Vec<i32>`.

Important: `map()` doesn't remove elements. Every input produces exactly one output.

## 3. Understand `filter_map()`

Now imagine you want to:

- Multiply numbers greater than 100 by 3.
- Ignore everything else.

This is where `filter_map()` becomes useful.

Example: Transform only matching elements

INPUT

10

120

50

200

30

filter_map

If x > 100, return Some(x \* 3); otherwise None

OUTPUT

360

600

Code:

```
let vec: Vec<i32> = array
    .iter()
    .filter_map(|x| {
        if *x > 100 {
            Some(*x * 3)
        } else {
            None
        }
    })
    .collect();
```

Here is the important part:

```
Some(*x * 3)
```

This means: Keep this value.

```
None
```

This means: Discard this element.

`filter_map()` expects your closure to return an `Option`.

That's why your original code generated errors.

You wrote:

```
if x > 2 {
    x * 3
} else {
    None
}
```

There are two problems:

1. `x` is `&i32`, so you need `*x` for integer operations.
2. The `if` branch returns `i32`, but the `else` branch returns `Option<_>`. Both branches must return the same type.

Correct version:

```
if *x > 2 {
    Some(*x * 3)
} else {
    None
}
```

## 4. Compare their actual implementations

This is the conceptual difference:

| Feature             | `map()`              | `filter_map()`                |
| ------------------- | -------------------- | ----------------------------- |
| Transforms elements | Yes                  | Yes                           |
| Removes elements    | No                   | Yes                           |
| Closure returns     | Any type             | `Option<U>`                   |
| Output count        | Same as input        | Zero to input count           |
| Typical use         | Transform every item | Transform only selected items |

Their simplified signatures look like this:

map:

```
fn map<B, F>(self, f: F) -> Map<Self, F>
where
    F: FnMut(Self::Item) -> B;
```

filter_map:

```
fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
where
    F: FnMut(Self::Item) -> Option<B>;
```

Notice the difference in the closure:

- `map`: `-> B`
- `filter_map`: `-> Option<B>`

That's the key.

## 5. Let's test your understanding

I want you to solve these three small exercises. Don't run them immediately; first predict their outputs.

Rust iterator practice

0/3 answered

1\. What will this produce?

[1, 2, 3, 4].iter().map(|x| \*x \* 2).collect::\<Vec<\_>>()&#x20;

[2, 4, 6, 8]

[2, 4]

[Some(2), Some(4), Some(6), Some(8)]

2\. What will this produce?

[1, 2, 3, 4].iter().filter_map(|x| if \*x > 2 { Some(\*x \* 2) } else { None }).collect::\<Vec<\_>>()&#x20;

[2, 4, 6, 8]

[6, 8]

[None, None, Some(6), Some(8)]

3\. What will this produce?

[1, 2, 3].iter().map(|x| if \*x > 2 { Some(\*x) } else { None }).collect::\<Option\<Vec<\_>>>()

Some([3])

[None, None, Some(3)]

None

My advice: Don't memorize the methods. Remember what their closures must return. Once you understand that, choosing between `map()` and `filter_map()` becomes natural.
