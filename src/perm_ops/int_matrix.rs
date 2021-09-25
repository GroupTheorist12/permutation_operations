use std::fmt;
use std::ops;

//********* start of IntMatrix
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
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
    pub fn is_unital(&self) -> bool {
        let mut ret = true;
        'outer: for i in 0..self.rows {
            for j in 0..self.cols {
                if self.matris[i as usize][j as usize] != 1i32 {
                    ret = false;
                    break 'outer;
                }
            }
        }
        ret
    }
}

impl IntMatrix {
    pub fn has_zeros(&self) -> bool {
        let mut ret = false;
        'outer: for i in 0..self.rows {
            for j in 0..self.cols {
                if self.matris[i as usize][j as usize] == 0i32 {
                    ret = true;
                    break 'outer;
                }
            }
        }
        ret
    }
}

impl IntMatrix {
    pub fn is_off_diagonal(&self) -> bool {
        let mut ret = true;
        'outer: for i in 0..self.rows {
            for j in 0..self.cols {
                if i == j && self.matris[i as usize][j as usize] != 0i32 {
                    ret = false;
                    break 'outer;
                }
            }
        }
        ret
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
    pub fn swap_row(&mut self, row_a: usize, row_b: usize) -> IntMatrix {

        let mut swap_matrix = self.clone();

        let ra = row_a - 1; // indexer
        let rb = row_b - 1; // indexer

        for i in 0..self.cols as usize {
                swap_matrix.set_value(ra, i, self.get_value(rb, i));
                swap_matrix.set_value(rb, i, self.get_value(ra, i));
            
          }
  
        swap_matrix
    }
}

impl IntMatrix {
    pub fn set_row_from_vec(matrix: &mut IntMatrix, vector: Vec<i32>, row: usize) {

        for i in 0..matrix.cols as usize {
            matrix.set_value(row, i, vector[i]);
        }
    }
}

impl IntMatrix {
    pub fn set_col_from_vec(matrix: &mut IntMatrix, vector: Vec<i32>, col: usize) {

        for i in 0..matrix.rows as usize {
            matrix.set_value(i, col, vector[i]);
        }
    }
}

impl IntMatrix {
    pub fn transpose(&mut self) -> IntMatrix {
        let mut ret = IntMatrix::new(self.rows);
        
        for i in 0..self.cols as usize {
          for  j in 0.. self.rows as usize{
            ret.set_value(j, i, self.get_value(i, j));
          }
        }
        ret
    }
}

impl IntMatrix {
    pub fn inverse(&mut self) -> IntMatrix{
        self.transpose()
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

impl IntMatrix {
    pub fn mul_scalar(&self, scalar_value: i32) -> IntMatrix {
        let mut ret = self.clone();

        for i in 0..self.rows as usize {
            for j in 0..self.cols as usize {
            
                let val = ret.get_value(i, j) * scalar_value;
                ret.set_value(i, j, val);
            }     
        }    
        ret
    }
}
impl IntMatrix {
    pub fn addition(a_in: IntMatrix, b_in: IntMatrix) -> IntMatrix {
        let mut a = a_in.clone();
        let mut b = b_in.clone();

        let mut perm_matrix = IntMatrix::new(a.rows);

        perm_matrix.zero();

        for i in 0..perm_matrix.rows as usize {
            for j in 0..perm_matrix.cols as usize {
                perm_matrix
                .set_value(i, j, a.get_value(i, j) + b.get_value(i, j));
            }
        }
        perm_matrix

    }
}

impl IntMatrix {
    pub fn identity(&mut self) -> bool {
        let mut ret = false;
        for i in 0..self.rows as usize {
            if self.get_value(i, i) == 1 {
                ret = true;
            }
            else {
                ret = false;
            }

        }
        ret
    }

}

impl IntMatrix {
    pub fn get_perm_matrix(br: Vec<i32>) -> IntMatrix {
        let mut ret = IntMatrix::new(br.len() as i32);
        ret.zero();
    
        for i in 0..br.len() {
            let col_n = br[i] - 1;
    
            ret.set_value(col_n as usize, i as usize, 1i32); //perm value is row, i is column
        }
    
        ret
    }
    
}
impl IntMatrix {
    pub fn to_latex(&self) -> String {
        let mut s = String::new();
        s.push_str("\\begin{bmatrix}\n");
        for i in 0..self.rows {
            for j in 0..self.cols {
                if j < self.cols - 1 {
                    let vr = format!("{} &", self.matris[i as usize][j as usize]);
                    s.push_str(&vr);
                } else {
                    let vrn = format!("{}", self.matris[i as usize][j as usize]);
                    s.push_str(&vrn);
                }
            }
            s.push_str("\\\\\n");
        }
        s.push_str(" \\end{bmatrix}");
        s
    }
}

impl IntMatrix {
    pub fn get_row(&self, row: usize) ->Vec<i32> {
        let mut v:Vec<i32> = Vec::new();

        for i in 0..self.cols as usize {
            v.push(self.matris[row][i]);
        }
        v
    }
}

impl IntMatrix {
    pub fn get_col(&self, col: usize) ->Vec<i32> {
        let mut v:Vec<i32> = Vec::new();

        for i in 0..self.cols as usize {
            v.push(self.matris[i][col]);
        }
        v
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

impl ops::Add<IntMatrix> for IntMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::addition(self, rhs)
    }
}


