use std::collections::HashSet;
use crate::IntMatrix;

use crate::Permutation;

use crate::PermutationList;

pub fn get_cyclic_groups(order: i32) -> HashSet<IntMatrix>{

    let mut perms = PermutationList::new(order);

    perms.permute(perms.set_imut.clone());

    let vc1 = perms.get_cyclic_groups();

    let ins_matrix_perms = |order: i32, perms: Vec<Permutation>| -> IntMatrix {
        let mut mx = IntMatrix::new(order);
        for i in 0..perms.len() {
            let index = perms[i].bottom_row.clone().iter().position(|&r| r == 1).unwrap();
            IntMatrix::set_row_from_vec(&mut mx, perms[i].bottom_row.clone(), index);

        }
        mx
    };

    let mut ind = 1;

    let mut mv:Vec<IntMatrix> = Vec::new();


    for perm in &vc1 {

        let mut prod1 = perm.clone();    
        let mut vp:Vec<Permutation> = Vec::new();

        vp.push(perm.clone());

        for _ in 0..order - 1 {
            prod1 = prod1 * perm.clone();

            vp.push(prod1.clone());
        }
        
        let m_tmp = ins_matrix_perms(order, vp);

        if !m_tmp.has_zeros() {
            mv.push(m_tmp);
        }
        ind = ind + 1;

    }

    let hi:HashSet<IntMatrix> = mv.iter().cloned().collect();

    hi

}
