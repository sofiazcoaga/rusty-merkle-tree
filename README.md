# Rusty Merkle Tree
This repository contains a lib for Merkle Trees management that allows a user to:
- create a Merkle Tree given an array of data elements.
- generate a proof that a certain element is contained in a tree.
- verify that an element is in the tree.
- add elements to an existing tree.

# Requirements
- [Rust toolchain installed](https://doc.rust-lang.org/beta/book/ch01-01-installation.html)
- Make installed

# How to interact with the project
The project contains a Makefile to allow easy interaction. The available commands are:
- `make build`: compiles the project.
- `make test`: executes the tests.
- `make example`: executes an example of the use of the library.
- `make doc`: generates the documentation of the lib code and opens it in the browser.

# What is a Merkle Tree?
A Merkle Tree (also called Hash Tree) is a binary tree where every leaf is the hash of a data element and every parent node is a hash of its two children (unified by some operation as it could be concatenation).

# Creating a Merkle Tree
In order to create a Merkle Tree, call the static method:
`MerkleTree::new_from_data()` that takes a `Vec<Vec<u8>>` as an input. This means it takes a vector of buffers that can be anything in its content.

Let's say we want to store the words "Hello" and "World" in a Merkle Tree, we could do something like:
```rust
let hello_as_bytes = b"Hello".to_vec();
let world_as_bytes = b"World".to_vec();
// Consider: `unwrap()` is not safe, only use if certain that the result of the function is Ok().
// This is only for example purposes. Also take into account that it is important for `merkle_tree`
// to be mutable.
let mut merkle_tree = MerkleTree::new_from_data(&vec![hello_as_bytes, world_as_bytes]).unwrap();
```

# Adding an element to a Merkle Tree
Let's say that now that we have our Merkle Tree we want to add a new element to it. We can do so by calling:
```rust
let exclamation_mark_as_bytes = b"!".to_vec();
merkle_tree.add_element(exclamation_mark_as_bytes);
```

# Generating a Merkle Proof
A Merkle Tree can provide us with a so called _merkle proof_ which is basically an array of hashes that another function can use to verify that a certain element is a part of the tree without having the whole tree.
In order to ask the tree for this proof we need to know the index where the data has been stored. With that we can call the following method:
```rust
let hello_index = 0;
let hello_proof = merkle_tree.get_merkle_proof(hello_index);
```
After that, let's see how we can use the merkle proof to verify the element is a part of the tree.

# Verifying an element is contained in the tree
The lib provides a function to verify that an element is contained in a tree in a specific position. Let's say we got the proof as showed in the previous step, then we can pass it into the verifying function an examine its result.

```rust
let hello_as_bytes = b"Hello".to_vec();
let hello_index = 0; // We must know the index of the content we will be asking for.
let hello_proof = merkle_tree.get_merkle_proof(hello_index);

// Verified will be a boolean value. If it is true, then the element was verified for that position in that tree. If false, the element was not verified.
let verified = verify_element(hello_as_bytes, &hello_proof, merkle_tree.get_root(), hello_index);
```
