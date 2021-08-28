use std::fmt;
use std::ops;

use crate::IntMatrix;

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
        } else {
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
            full_rep: String::from(""),
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
        p.full_rep = format!(
            "{:?}{:?}   {:?}\n{:?}{:?} = {:?}",
            a.top_row, b.top_row, p.top_row, a.bottom_row, b.bottom_row, p.bottom_row
        );

        p
    }

    pub fn inverse(&mut self) -> Permutation {
        let mut perm = self.top_row.clone();

        for i in 0..self.bottom_row.len() {
            let ib = self.bottom_row[i] - 1;
            let ia = self.top_row[i];
            perm[ib as usize] = ia;
        }

        let p = Permutation::new(self.top_row.clone(), perm);

        p
    }

    pub fn to_string(&self) -> String {
        format!("{:?}\n{:?}\n", self.top_row, self.bottom_row)
    }
    pub fn identity(&self) -> bool {
        self.top_row
            .iter()
            .zip(self.bottom_row.iter())
            .all(|(a, b)| a == b)
    }
}

impl ops::Mul<Permutation> for Permutation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::product(self, rhs)
    }
}

//////////////////////////////////////////////////Start of new stuff!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
impl Permutation {
    pub fn get_perm_matrix(&self) -> IntMatrix {
        let mut ret = IntMatrix::new(self.bottom_row.len() as i32);
        ret.zero();

        for i in 0..self.bottom_row.len() {
            let col_n = self.bottom_row[i] - 1;

            ret.set_value(col_n as usize, i as usize, 1i32); //perm value is row, i is column
        }

        ret
    }
}
