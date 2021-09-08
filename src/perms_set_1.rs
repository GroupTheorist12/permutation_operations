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
    //Start of product method ********************

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
//Start of Mul trait impl ********************

//Start of PermutationList impl
#[derive(Clone, PartialEq)]
pub struct PermutationList {
    pub order: i32,
    pub max: i32,
    pub set_imut: Vec<i32>,
    pub perms: Vec<Permutation>,
}

fn factorial(num: i32) -> i32 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

pub fn swap(set_in: Vec<i32>, index1: usize, index2: usize) -> Vec<i32> {
    let mut set = set_in.clone();

    let tmp = set[index1];

    set[index1] = set[index2];

    set[index2] = tmp;

    set
}

impl PermutationList {
    pub fn get_set(order: i32) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();

        for i in 1..=order {
            v.push(i);
        }
        v
    }
}
impl PermutationList {

    pub fn get_permutations(&mut self, mut set: Vec<i32>) {
        let mut ind1 = 1;
        let mut ind2 = 2;

        for _ in 0..self.max {
            set = swap(set, ind1, ind2);
            let p = Permutation {
                order: self.order,
                top_row: self.set_imut.clone(),
                bottom_row: set.clone(),
                full_rep: "".to_string()
            };
            self.perms.push(p);

            ind1 = ind1 + 1;
            ind2 = ind2 + 1;

            if ind2 > set.len() - 1 {
                ind1 = 1;
                ind2 = 2;
            }
        }
    }
}

impl PermutationList {

    pub fn permute(&mut self, mut set: Vec<i32>) {
        
        if set.len() == 2 {
            let p = Permutation {
                order: self.order,
                top_row: self.set_imut.clone(),
                bottom_row: self.set_imut.clone(),
                full_rep: "".to_string()
            };    
            let p2 = Permutation {
                order: self.order,
                top_row: self.set_imut.clone(),
                bottom_row: vec![2, 1],
                full_rep: "".to_string()
            };    
            self.perms.push(p);
            self.perms.push(p2);

            return;    
        }

        for i in 0..self.set_imut.len() {
            self.get_permutations(set.clone());

            if i < self.set_imut.len() - 1 {
                set = swap(set, 0, i + 1);
            }
        }
    }
}

impl PermutationList {
    pub fn new(o: i32) -> PermutationList {
        PermutationList {
            order: o,
            max: factorial(o) / o,
            set_imut: PermutationList::get_set(o),
            perms: Vec::new(),
        }
    }
}

fn main() {
    let mut perms = PermutationList::new(3);

    perms.permute(perms.set_imut.clone());

    for perm in perms.perms {
        println!("{}", perm);
    }

}
