# Rust Borrow Checker and Vector Modification Bug
This repository demonstrates a common error in Rust involving borrowing and mutable vectors.  The code in `bug.rs` attempts to borrow a value from a vector and then modify the vector, leading to a potential memory safety violation.

The `bugSolution.rs` file provides a corrected version that avoids this issue by using techniques such as cloning or creating a copy of the vector data.