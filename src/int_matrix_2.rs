use std::fmt;
use std::ops;

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
impl IntMatrix {
    pub fn product(a_in: IntMatrix, b_in: IntMatrix) -> IntMatrix {
        let mut a = a_in.clone();
        let mut b = b_in.clone();

        let mut perm_matrix = IntMatrix::new(a.rows);

        perm_matrix.zero();

        for i in 0..perm_matrix.rows as usize {
            for j in 0..perm_matrix.cols as usize {
                for k in 0..perm_matrix.cols as usize {
                    let mut val = perm_matrix.get_value(i, j);
                    val +=
                        a.get_value(i, k) * b.get_value(k, j);
                    perm_matrix.set_value(i, j, val);
                }
            }
        }
        perm_matrix
    }
}

//********* end of IntMatrix impl's

impl fmt::Display for IntMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl ops::Mul<IntMatrix> for IntMatrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::product(self, rhs)
    }
}


pub fn get_perm_matrix(br: Vec<i32>) -> IntMatrix {
    let mut ret = IntMatrix::new(br.len() as i32);
    ret.zero();

    for i in 0..br.len() {
        let col_n = br[i] - 1;

        ret.set_value(col_n as usize, i as usize, 1i32); //perm value is row, i is column
    }

    ret
}

fn main() {
    let v1 = vec![2, 1, 3];

    let perm_matrix1 = get_perm_matrix(v1);

    let v2 = vec![1, 3, 2];

    let perm_matrix2 = get_perm_matrix(v2);

    let perm_matrix = perm_matrix1.clone() * perm_matrix2.clone();

    println!("{}", perm_matrix1);
    println!("{}", perm_matrix2);

    println!("{}", perm_matrix);
}
