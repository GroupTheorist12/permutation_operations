use std::fmt;
use std::ops;

#[derive(Clone, PartialEq)]
pub struct Permutation {
    pub order: i32,
    pub top_row: Vec<i32>,
    pub bottom_row: Vec<i32>,
    pub full_rep: String,
}

impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.full_rep != String::from("") {
            write!(f, "{}", self.full_rep)

        }
        else {
        
            write!(f, "{}", self.to_string())

        }
    }
}

impl Permutation {
    pub fn new(tr: Vec<i32>, br: Vec<i32>) -> Permutation {
        Permutation {
            order: tr.len() as i32,
            top_row: tr.clone(),
            bottom_row: br.clone(),
            full_rep: String::from("")
        }
    }

    pub fn product(a: Permutation, b: Permutation) -> Permutation {
        let mut perm = Vec::new();

        for i in 0..a.top_row.len() {
            let ib = b.bottom_row[i] - 1;
            let ia = a.bottom_row[ib as usize];
            perm.push(ia);
        }

        let mut p = Permutation::new(a.top_row.clone(), perm);
        p.full_rep = format!("{:?}{:?}   {:?}\n{:?}{:?} = {:?}", 
        a.top_row,
        b.top_row, 
        p.top_row,
        a.bottom_row,
        b.bottom_row, 
        p.bottom_row);

        p
    }

    pub fn to_string(&self) -> String{

        format!("{:?}\n{:?}\n", self.top_row, self.bottom_row)
    }    
}

impl ops::Mul<Permutation> for Permutation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::product(self, rhs)
    }
}


fn main() {
    let v = vec![1, 2, 3, 4];

    let v1 = vec![1, 3, 4, 2];
    let v2 = vec![3, 4, 1, 2];

    let  p1 = Permutation::new(v.clone(), v1);
    let  p2 = Permutation::new(v.clone(), v2);

    let p3 = p1.clone() * p2.clone();

    println!("{}", p3);
}
