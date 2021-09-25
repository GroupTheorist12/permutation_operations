//use std::ops;
//use std::fmt;

use crate::Permutation;
//use crate::IntMatrix;

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

    pub fn new(o: i32) -> PermutationList {
        PermutationList {
            order: o,
            max: factorial(o) / o,
            set_imut: PermutationList::get_set(o),
            perms: Vec::new(),
        }
    }

    pub fn get_cyclic_series_biv(pms:&mut PermutationList , ind:usize, val:i32) -> Vec<Permutation> {
        pms.perms.clone()
        .into_iter()
        .filter(|i| i.is_off_diagonal() && i.bottom_row[ind] == val)
        .collect::<Vec<_>>()
    
    }

    pub fn get_cyclic_series_by_index_val(&self, ind:usize, val:i32) -> Vec<Permutation> {
        self.perms.clone()
        .into_iter()
        .filter(|i| i.is_off_diagonal() && i.bottom_row[ind] == val)
        .collect::<Vec<_>>()
    
    }

    pub fn get_cyclic_series(&self, row:i32, col:i32) -> Vec<Permutation> {
        self.perms.clone()
        .into_iter()
        .filter(|i| i.is_off_diagonal() && i.contains_row_column(row, col))
        .collect::<Vec<_>>()

    }

    pub fn get_cyclic_groups(&self) -> Vec<Permutation> {
        self.perms.clone()
        .into_iter()
        .filter(|i| i.get_perm_matrix().is_off_diagonal())
        .collect::<Vec<_>>()

    }

}

