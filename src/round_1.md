# Round 1

## 1. Why `Box<T>`?

`Box<T>`, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. Boxes provide ownership for this allocation, and drop their contents when they go out of scope.

**What's the difference between `Rc<T>` and `Box<T>`?**

`Rc` provides shared ownership so by default its contents can't be mutated, while `Box` provides exclusive ownership and thus mutation is allowed:

## 2. `AsRef` and `Borrow`

`AsRef` is similar to `AsMut` which is used for converting between mutable references. If you need to do a costly conversion it is better to implement From with type &T or write a custom function.

**AsRef has the same signature as Borrow, but Borrow is different in few aspects:**

+ Unlike AsRef, Borrow has a blanket impl for any T, and can be used to accept either a reference or a value.

+ Borrow also requires that Hash, Eq and Ord for borrowed value are equivalent to those of the owned value. For this reason, if you want to borrow only a single field of a struct you can implement AsRef, but not Borrow.

Note: This trait must not fail. If the conversion can fail, use a dedicated method which returns an Option<T> or a Result<T, E>.
