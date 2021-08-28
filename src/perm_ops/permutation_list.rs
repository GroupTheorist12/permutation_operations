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
}

