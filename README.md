# Rusty Merkle Tree
This repository contains a lib for Merkle Trees management that allows a user to:
- create a Merkle Tree given an array of data elements.
- generate a proof that that a certain element is contained in a tree.
- verify that an element is in the tree.
- add elements to an existing tree.

# Requirements
- Rust toolchain installed
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
In order to create a Merkle Tree, call the function:
`create_merkle_tree_from_data()` that takes a `Vec<Vec<u8>>` as an input. This means it takes a vector of buffers. 

# Adding an element to a Merkle Tree

# Generating a Merkle Proof

# Verifying an element is contained in the tree
