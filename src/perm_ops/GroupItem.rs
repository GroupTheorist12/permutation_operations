use std::fmt;
use std::ops;

use crate::Permutation;

//********* start of GroupItem
#[derive(Clone, PartialEq)]
pub struct GroupItem {
    pub is_group_generator: bool,
    pub is_identity: bool,
    pub permutation: Permutation,
    pub name: String,
}

impl GroupItem {
    pub fn new(gg: bool, id:bool, p:Permutation, n:String ) -> GroupItem {
        GroupItem {
            is_group_generator: gg,
            is_group_generator: id,
            permutation: p,
            name:n
        }

    }
}
//********* end of GroupItem
