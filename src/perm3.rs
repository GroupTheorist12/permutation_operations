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

impl Permutation {
    // start of new method ***************
    pub fn new(tr: Vec<i32>, br: Vec<i32>) -> Permutation {
        Permutation {
            order: tr.len() as i32,
            top_row: tr.clone(),
            bottom_row: br.clone()
        }
    }
    // end of new method ***************

}
fn main() {
    let v = vec![1, 2];
    let v2 = vec![2, 1];

    let p1 = Permutation::new(v.clone(), v.clone());

    let p2 = Permutation::new(v.clone(), v2.clone());


    println!("{}", p1);

    println!("{}", p2);
}
