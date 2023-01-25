### Doubly linked list implementation in **safe** Rust

For educational use only.

Aside from bidirectional search this implementation keeps it as simple as it gets - with no generics and in a sub 200 lines (including tests) single header.

The list contains only four methods: new, push_back, get and len. It implements no iterators or any other trait.

Testing:

```$ cargo test```
