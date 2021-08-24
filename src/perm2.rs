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
