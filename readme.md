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

While this works, it would be nice to aggerate permutation information in a Rust struct. The following source code creates a **Permutation** struct and implements the **Display** trait for it.

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

Run the following to produce the output:

```bash
s$ cargo run --bin perm2
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/perm2`
[1, 2]
[1, 2]

[1, 2]
[2, 1]
```

While I love filling out struct declarations manually you probably don't so let's create a **new** method that takes two vectors:

######     perm3.rs

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


```

#### Permutation Product

The permutation product is the composition of two permutations. The product is calculated from right to left. An example order 5 permutation product would be:

![perm_prod_1](perm_prod_1.gif)

1. In the right array 1 maps to 3, then 3 maps to 5 in the left array. Thus the first value in the bottom row of the resultant permutation is 5.
2. In the right array 2 maps to 5, then 5 maps to 2 in the left array. Thus the second value in the bottom row of the resultant permutation is 2.
3. In the right array 3 maps to 1, then 1 maps to 3 in the left array. Thus the third value in the bottom row of the resultant permutation is 3.
4. In the right array 4 maps to 2, then 2 maps to 4 in the left array. Thus the fourth value in the bottom row of the resultant permutation is 4.
5. In the right array 5 maps to 4, then 4 maps to 1 in the left array. Thus the fifth value in the bottom row of the resultant permutation is 1.

And the resultant permutation is:

![perm_prod_2](perm_prod_2.gif)

We will create a **product** method and implement the **Mul** trait (*) the source looks as follows:

######     perm_prod.rs

```rust
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

    //Start of product method ********************
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
    //End of product method ********************

    pub fn to_string(&self) -> String{

        format!("{:?}\n{:?}\n", self.top_row, self.bottom_row)
    }    
}

//Start of Mul trait impl ********************

impl ops::Mul<Permutation> for Permutation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::product(self, rhs)
    }
}
//End of Mul trait impl ********************


fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let v1 = vec![3, 4, 5, 1, 2];
    let v2 = vec![3, 5, 1, 2, 4];

    let  p1 = Permutation::new(v.clone(), v1);
    let  p2 = Permutation::new(v.clone(), v2);

    let p3 = p1.clone() * p2.clone();

    println!("{}", p3);
}


```

Notice we added a new **full_rep** field to the **Permutation** struct and added a **to_string** method. 



Running the above will produce the following output:

```bash
$ cargo run --bin perm_prod
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/perm_prod`
[1, 2, 3, 4, 5][1, 2, 3, 4, 5]   [1, 2, 3, 4, 5]
[3, 4, 5, 1, 2][3, 5, 1, 2, 4] = [5, 2, 3, 4, 1]
```

#### Permutation Identity

The permutation identity is any permutation that has the same top row as bottom row. (The top row is ordered). So for an order 4 permutation the identity is:

![perm_identity](perm_identity.png)

The **identity** method can be found in the **perm_inverse.rs** file below.

#### Permutation Inverse

A permutation inverse is a permutation that when the product is taken with another permutation returns the identity permutation.  This is shown below for n = 3:

![perm_inverse_1](perm_inverse_1.gif)

We will calculate the inverse permutation of order 4. 

![perm_inverse_2](perm_inverse_2.gif)

**Exchange top row with bottom row**

![perm_inverse_4](perm_inverse_4.png)

**1 goes to 2 in new permutation** 

![perm_inverse_3](perm_inverse_3.png)

**2 goes to 3 in new permutation**

![perm_inverse_5](perm_inverse_5.png)

**3 goes to 1 in new permutation**

![perm_inverse_6](perm_inverse_6.png)

**4 goes to 4 in new permutation**

![perm_inverse_7](perm_inverse_7.png)

**We are done.**


We will create an **inverse** method. The source is below.

######     perm_inverse.rs

```rust
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

    pub fn inverse(&mut self) -> Permutation {
        let mut perm = self.top_row.clone();

        for i in 0..self.bottom_row.len() {
            let ib = self.bottom_row[i] - 1;
            let ia = self.top_row[i];
            perm[ib as usize] = ia;
        }

        let  p = Permutation::new(self.top_row.clone(), perm);

        p
    }

    pub fn to_string(&self) -> String{

        format!("{:?}\n{:?}\n", self.top_row, self.bottom_row)
    }
    
    pub fn identity(&self) -> bool {
        self.top_row.iter().zip(self.bottom_row.iter()).all(|(a,b)| a == b)
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

    let  mut p1 = Permutation::new(v.clone(), v1);

    let p2 = p1.inverse();


    println!("{}", p2);

    let p3 = p1 * p2.clone();

    println!("{}", p3.identity());

    println!("{}", p3);

}

```



Running the above produces the following output:

```bash
$ cargo run --bin perm_inverse
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/perm_inverse`
[1, 2, 3, 4]
[1, 4, 2, 3]

true
[1, 2, 3, 4][1, 2, 3, 4]   [1, 2, 3, 4]
[1, 3, 4, 2][1, 4, 2, 3] = [1, 2, 3, 4]
```

Notice how taking the product of the two permutations produces the identity permutation.





#### Permutation Matrix

Permutation matrices are square matrices with a single 1 in each column and a single 1 in each row with zeros elsewhere. An example of a 3 x 3 permutation matrix:

![perm_matrix_1](perm_matrix_1.png)

We can convert from a permutation to a permutation matrix as follows:

![perm_matrix_2](perm_matrix_2.png)

Starting with zeroed matrix, Row 2, column 1 gets a 1

![perm_matrix_3](perm_matrix_3.png)

Row 1, column 2 gets a 1

![perm_matrix_4](perm_matrix_4.png)

Row 3, column 3 gets a 1

![perm_matrix_5](perm_matrix_5.png)

We are done



We will create a **IntMatrix** struct that will handle the duty as a permutation matrix (and other duties as well).

######     IntMatrix

```rust
//********* start of IntMatrix
#[derive(Clone, PartialEq)]
pub struct IntMatrix {
    pub rows: i32,
    pub cols: i32,
    pub matris: Vec<Vec<i32>>,
}
//********* end of IntMatrix

```

We need a **new** method impl.

######     new method impl

```rust
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

```

Notice that the matrix is square since we are using the order for both rows and columns.

We will create a **zero** method so that we can zero out a matrix.

######     zero method

```rust
impl IntMatrix {
    pub fn zero(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.matris[i as usize][j as usize] = 0i32;
            }
        }
    }
}

```

We will create a **getter** and **setter** methods.

######     getter, setter methods

```rust
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

```
We will create a **to_string**  method that will produce a nicely formatted matrix.

######     to_string method

```rust
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

```

We implement the **Display** trait for **IntMatrix**

######     Implement Display Trait

```rust
impl fmt::Display for IntMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

```

Notice it calls the **to_string** method.

And finally we create a **get_perm_matrix** method.

######     get_perm_matrix Method

```rust
pub fn get_perm_matrix(br: Vec<i32>) -> IntMatrix {
    let mut ret = IntMatrix::new(br.len() as i32);
    ret.zero();

    for i in 0..br.len() {
        let col_n = br[i] - 1;

        ret.set_value(col_n as usize, i as usize, 1i32); //perm value is row, i is column
    }

    ret
}

```

This method creates a permutation matrix from a permutation. 

The full source is found below:

######     int_matrix_1.rs

```rust
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

//********* end of IntMatrix impl's

impl fmt::Display for IntMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
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

fn main(){
    let v1 = vec![2, 1, 3];

    let perm_matrix = get_perm_matrix(v1);

    println!("{}", perm_matrix);

}

```
Running the above produces the following output:

```bash
$ cargo run --bin int_matrix_1
   Compiling permutation_operations v0.1.0 (/home/brad/RustProjects/permutation_operations)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/int_matrix_1`
0 1 0 
1 0 0 
0 0 1 
```

#### Permutation Matrix Transpose
The transpose of a matrix is swapping the rows for columns in the matrix.  An example for n = 3 is:

![int_matrix_1](int_matrix_1.png)

We implement the **transpose** method for **IntMatrix**

######      transpose method

```rust
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

```
#### Permutation Matrix Inverse
The transpose of a permutation matrix is also it's inverse. 

The product of the a permutation matrix and it's inverse is the identity:

![int_matrix_2](int_matrix_2.png)

We implement the **inverse** method for **IntMatrix** by taking it's transpose.

######      inverse method

```rust
impl IntMatrix {
    pub fn inverse(&mut self) -> IntMatrix{
        self.transpose()
    }
}

```

