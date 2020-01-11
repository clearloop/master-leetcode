# Round 1

## 1. `Box<T>` and `Rc`

`Box<T>`, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.

**What's the difference between `Rc<T>` and `Box<T>`?**

`Rc` provides shared ownership so by default its contents can't be mutated, while `Box` provides exclusive ownership and thus mutation is allowed:

```rust
use std::rc::Rc;

fn main() {
    let mut a = Box::new(1);
    let mut b = Rc::new(1);

    *a = 2; // works
    *b = 2; // doesn't
}
```

> Checkout this question on [stackoverflow][1]

[1]: https://stackoverflow.com/questions/49377231/when-to-use-rc-vs-box

## 2. `Cell` and `RefCell`

Values of the `Cell<T>` and `RefCell<T>` types may be mutated through shared references (i.e. the common &T type), whereas most Rust types can only be mutated through unique `(&mut T)` references. We say that `Cell<T>` and `RefCell<T>` provide **'interior mutability'**, in contrast with typical Rust types that exhibit **'inherited mutability'**.

Cell types come in two flavors: `Cell<T>` and `RefCell<T>`. `Cell<T>` implements interior mutability by moving values in and out of the `Cell<T>`. To use references instead of values, one must use the `RefCell<T>` type, acquiring a write lock before mutating. `Cell<T>` provides methods to retrieve and change the current interior value:

+ For types that implement `Copy`, the get method retrieves the current interior value.
+ For types that implement `Default`, the `take` method replaces the current interior value with `Default::default()` and returns the replaced value.
+ For all types, the `replace` method replaces the current interior value and returns the replaced value and the `into_inner` method consumes the `Cell<T>` and returns the interior value. Additionally, the `set` method replaces the interior value, dropping the replaced value.

`RefCell<T>` uses Rust's lifetimes to implement `'dynamic borrowing'`, a process whereby one can claim temporary, exclusive, mutable access to the inner value. Borrows for `RefCell<T>`s are tracked `'at runtime'`, unlike Rust's native reference types which are entirely tracked statically, at compile time. Because `RefCell<T>` borrows are dynamic it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.

### When to choose interior mutability?

The more common inherited mutability, where one must have unique access to mutate a value, is one of the key language elements that enables Rust to reason strongly about pointer aliasing, statically preventing crash bugs. Because of that, inherited mutability is preferred, and interior mutability is something of a last resort. Since cell types enable mutation where it would otherwise be disallowed though, there are occasions when interior mutability might be appropriate, or even must be used, e.g.

+ Introducing mutability 'inside' of something immutable
+ Implementation details of logically-immutable methods.
+ Mutating implementations of `Clone`.

## 3. `AsRef` and `Borrow`

`AsRef` is similar to `AsMut` which is used for converting between mutable references. If you need to do a costly conversion it is better to implement From with type &T or write a custom function.

**AsRef has the same signature as Borrow, but Borrow is different in few aspects:**

+ Unlike AsRef, Borrow has a blanket impl for any T, and can be used to accept either a reference or a value.

+ Borrow also requires that Hash, Eq and Ord for borrowed value are equivalent to those of the owned value. For this reason, if you want to borrow only a single field of a struct you can implement AsRef, but not Borrow.

Note: This trait must not fail. If the conversion can fail, use a dedicated method which returns an Option<T> or a Result<T, E>.

### Example

By creating a generic function that takes an `AsRef<str>` we express that we want to accept all references that can be converted to `&str` as an argument. Since both String and `&str` implement `AsRef<str>` we can accept both as input argument.

```rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
```

## 4. `Clone`、`Copy` and `ToOwned`

Differs from `Copy` in that `Copy` is implicit and extremely inexpensive, while Clone is always explicit and may or may not be expensive. In order to enforce these characteristics, Rust does not allow you to reimplement Copy, but you may reimplement Clone and run arbitrary code.

`Copy` are the types whose values can be duplicated simply by copying bits:

```rust
// We can derive a `Copy` implementation. `Clone` is also required, as it's
// a supertrait of `Copy`.
#[derive(Debug, Copy, Clone)]
struct Foo;

let x = Foo;

let y = x;

// `y` is a copy of `x`

println!("{:?}", x); // A-OK!
```

Both a copy and a move can result in bits being copied in memory, although this is sometimes optimized away.

`Clone` is a supertrait of `Copy`, so everything which is `Copy` must also implement `Clone`. If a type is `Copy` then its `Clone` implementation only needs to return *self (see the example above).

`ToOnwed` are some types make it possible to go from borrowed to owned, usually by implementing the Clone trait. But Clone works only for going from &T to T. The ToOwned trait generalizes Clone to construct owned data from any borrow of a given type.

## 5. Zero Copy

### `std::mem::swap`

Swaps the values at two mutable locations, without deinitializing either one.

```rust
use std::mem;

let mut x = 5;
let mut y = 42;

mem::swap(&mut x, &mut y);

assert_eq!(42, x);
assert_eq!(5, y);
```

### `std::mem::replace`

Moves src into the referenced dest, returning the previous dest value.

```rust
use std::mem;

let mut v: Vec<i32> = vec![1, 2];

let old_v = mem::replace(&mut v, vec![3, 4, 5]);
assert_eq!(vec![1, 2], old_v);
assert_eq!(vec![3, 4, 5], v);
```

### `std::option::Option::take`

Takes the value out of the option, leaving a None in its place.

```rust
let mut x = Some(2);
let y = x.take();
assert_eq!(x, None);
assert_eq!(y, Some(2));

let mut x: Option<u32> = None;
let y = x.take();
assert_eq!(x, None);
assert_eq!(y, None);
```
