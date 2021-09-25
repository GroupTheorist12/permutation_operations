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

    pub fn has_row_column(perm: Permutation, r: i32, c: i32) -> bool{
        perm.top_row
            .iter()
            .zip(perm.bottom_row.iter())
            .any(|(a, b)| *a == c && *b == r)

    }

    pub fn contains_row_column(&self, r: i32, c: i32) -> bool{
        self.top_row
            .iter()
            .zip(self.bottom_row.iter())
            .any(|(a, b)| *a == c && *b == r)

    }

    pub fn is_off_diagonal(&self) -> bool {
        
        /*
        self.top_row
            .iter()
            .zip(self.bottom_row.iter())
            .all(|(a, b)| a != b)
        */

        self.get_perm_matrix().is_off_diagonal()
    }

    pub fn repeat_character(c: char, times: usize) -> String {
        let mut sbuilder = String::new();

        for _ in 0..times {
            sbuilder.push(c);
        }
        sbuilder
    }

    pub fn to_tuple_vector(&self) -> Vec<(i32, i32)> {

        let mut v:Vec<(i32, i32)>  = Vec::new();

        for i in 0..self.top_row.len() {
            v.push((self.top_row[i], self.bottom_row[i]));
        }
        v

    }
    pub fn to_latex(&self) -> String {
        let s = r#"
        \left(
        \begin{array}{REPLACE_THECC}
        REPLACE_MY_TOP_ROW\\
        REPLACE_MY_BOTTOM_ROW\\
        \end{array}
        \right)
        "#;

        let s3 = &s.replace("REPLACE_THECC", &Permutation::repeat_character('c', self.top_row.len()));
        let mut j1 = String::new();
        let mut j2 = String::new();

        for i in 0..self.top_row.len() {

            if i < self.top_row.len() - 1 {
                j1.push_str(&format!("{}&", self.top_row[i]));
                j2.push_str(&format!("{}&", self.bottom_row[i]));
            }
            else {
                j1.push_str(&format!("{}", self.top_row[i]));
                j2.push_str(&format!("{}", self.bottom_row[i]));

            }

        }
        let s4 = &s3.replace("REPLACE_MY_TOP_ROW", &j1);
        let s5 = &s4.replace("REPLACE_MY_BOTTOM_ROW", &j2);
    
        s5.to_string()
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
