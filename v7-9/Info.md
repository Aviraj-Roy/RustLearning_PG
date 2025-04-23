Ownership - set of rules to govern how a Rust program manages memory, checked by compiler, which enforces borrowing and ownership principles and ensures safety without needing a garbage collector.

These principles help prevent data races and ensure that references do not outlive the data they point to, leading to more predictable and efficient memory management.

Size known -> place in stack
Size unknown -> place in heap & store the pointer to the allocated memory, in the stack
Accessing a data in the heap is slower than accessing data on the stack because a pointer has to be followed to get there.

When code calls a func, valus passed to the func & func's local variables are stored on the stack, while any data allocated on the heap must be managed through references.

Rules:-

1. Each value in Rust has a single owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped.
4. Values can be borrowed temporarily, but the rules of ownership still apply.

Here are some of the types that implement Copy:

1. All the integer types, such as u32.
2. The Boolean type, bool, with values true and false.
3. All the floating-point types, such as f64.
4. The character type, char.
5. Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
