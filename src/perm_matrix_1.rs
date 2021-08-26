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
//********* start of IntMatrix
#[derive(Clone, PartialEq)]
pub struct IntMatrix {
    pub rows: i32,
    pub cols: i32,
    pub matris: Vec<Vec<i32>>,
}
//********* end of IntMatrix

//********* start of IntMatrix impl's
impl IntMatrix {
    pub fn new(order: i32) -> IntMatrix {
        let matrix = vec![vec![0i32; order as usize]; order as usize];

        IntMatrix {
            rows: order,
            cols: order,
            matris: matrix,
        }
    }
}

impl IntMatrix {
    pub fn zero(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.matris[i as usize][j as usize] = 0i32;
            }
        }
    }
}

impl IntMatrix {
    pub fn set_value(&mut self, row: usize, col: usize, value: i32) {
        self.matris[row][col] = value;
    }
}

impl IntMatrix {
    pub fn get_value(&mut self, row: usize, col: usize) -> i32 {
        self.matris[row][col]
    }
}

impl IntMatrix {
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let vr = format!("{} ", self.matris[i as usize][j as usize]);
                s.push_str(&vr);
            }
            s.push_str("\n");
        }
        s
    }
}

//********* end of IntMatrix impl's

impl fmt::Display for IntMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

//////////////////////////////////////////////////End of new stuff!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

fn main(){
    let v = vec![1, 2, 3];
    let v1 = vec![2, 1, 3];

    let perm = Permutation::new(v.clone(), v1.clone());

    let perm_matrix = perm.get_perm_matrix();

    println!("{}", perm_matrix);

}
