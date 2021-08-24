# Permutation Operations in Rust

#### Permutation Examples

A permutation is the order left to right that a set of objects can be arranged. Thus the set {1,2} has the permutations:   {1,2},  {2,1}. The number of permutations for a given set of objects is **n!** where n is the number of objects (order) and ! is n factorial.

A permutation can be written as a two rows of numbers. The permutations of the order 2 set \{1,2\} can be written as:

![](CodeCogsEqn.gif)

We can express this in rust as:

######     perm1.rs

```rust
fn main() {
    let v = vec![1, 2];
    let v2 = vec![2, 1];

    println!("{:?}", v);
    println!("{:?}", v);
    println!();

    println!("{:?}", v);
    println!("{:?}", v2);
}

```

Which produces the output:

```bash
$ cargo run --bin perm1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/perm1`
[1, 2]
[1, 2]

[1, 2]
[2, 1]
```

While this works, it would be nice to aggerate permutation information in a Rust struct. The following source code creates a **Permutation** struct and a implements the **Display** trait for it.

 ######     perm2.rs

```rust
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Permutation {
    pub order: i32,
    pub top_row: Vec<i32>,
    pub bottom_row: Vec<i32>,
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}\n{:?}\n", self.top_row, self.bottom_row)
    }
}

fn main() {
    let v = vec![1, 2];
    let v2 = vec![2, 1];

    let p1 = Permutation {
        order: 2,
        top_row: v.clone(),
        bottom_row: v.clone(),
    };

    let p2 = Permutation {
        order: 2,
        top_row: v.clone(),
        bottom_row: v2.clone(),
    };

    println!("{}", p1);

    println!("{}", p2);
}


```

