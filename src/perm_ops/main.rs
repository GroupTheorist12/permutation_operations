use std::collections::HashSet;
use std::hash::{Hash};
mod int_matrix;
use int_matrix::IntMatrix;

mod permutation;
use permutation::Permutation;

mod permutation_list;
use permutation_list::PermutationList;

mod finte_group;

mod pdf_methods;

pub fn test_int_matrix() {
    let v1 = vec![2, 1, 3];

    let perm_matrix1 = IntMatrix::get_perm_matrix(v1);

    let v2 = vec![1, 3, 2];

    let perm_matrix2 = IntMatrix::get_perm_matrix(v2);

    let perm_matrix = perm_matrix1.clone() * perm_matrix2.clone();

    println!("{}", perm_matrix1);
    println!("{}", perm_matrix2);

    println!("permutaion matrix product:\n{}", perm_matrix);

    let tp = perm_matrix.clone().transpose();
    println!("permutation matrix transpose:\n{}", tp);

    let tp2 = perm_matrix.clone().inverse();
    println!("permutation matrix identity same as transpose:\n{}", tp2);

    let idn = perm_matrix.clone() * tp2.clone();
    println!(
        "The product of a permutation matrix and it's inverse is the identity\n{} {}",
        idn,
        idn.clone().identity()
    );
}

pub fn test_permutation() {
    let v = vec![1, 2, 3, 4, 5];

    let v1 = vec![3, 4, 5, 1, 2];
    let v2 = vec![3, 5, 1, 2, 4];

    let p1 = Permutation::new(v.clone(), v1);
    let p2 = Permutation::new(v.clone(), v2);

    let p3 = p1.clone() * p2.clone();

    println!("{}", p3);
    println!();

    let p_inv = p3.clone().inverse();
    println!("{}", p_inv);
    println!();

    let pm = p_inv.clone().get_perm_matrix();
    println!("{}", pm);
    println!();

    let is_ident = p3.clone() * p_inv;
    println!("{}", is_ident.identity());
}

pub fn test_permutation_list() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let the_perms = &perms;

    println!("{}", the_perms.perms.len());
    for perm in &the_perms.perms {
        print!("{}", perm);
        println!();
    }
}

pub fn test_get_cyclic_groups() {
    let v = vec![1, 2, 3, 4];

    let idn = Permutation::new(v.clone(), v.clone());

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());
}

pub fn test_get_cyclic_groups_bunk() {
    let v = vec![1, 2, 3, 4];

    let idn = Permutation::new(v.clone(), v.clone());

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let vc1 = perms.get_cyclic_series(1i32, 2i32);
    let vc2 = perms.get_cyclic_series(1i32, 3i32);
    let vc3 = perms.get_cyclic_series(1i32, 4i32);

    let mut matrices: Vec<IntMatrix> = Vec::new();

    /*
    matrices.push(vc1[0].get_perm_matrix().clone() + vc2[0].get_perm_matrix().clone() + vc3[0].get_perm_matrix().clone());
    matrices.push(vc1[0].get_perm_matrix().clone() + vc2[1].get_perm_matrix().clone() + vc3[1].get_perm_matrix().clone());
    matrices.push(vc1[0].get_perm_matrix().clone() + vc2[2].get_perm_matrix().clone() + vc3[2].get_perm_matrix().clone());

    matrices.push(vc1[1].get_perm_matrix().clone() + vc2[0].get_perm_matrix().clone() + vc3[0].get_perm_matrix().clone());
    matrices.push(vc1[1].get_perm_matrix().clone() + vc2[1].get_perm_matrix().clone() + vc3[1].get_perm_matrix().clone());
    matrices.push(vc1[1].get_perm_matrix().clone() + vc2[2].get_perm_matrix().clone() + vc3[2].get_perm_matrix().clone());

    matrices.push(vc1[2].get_perm_matrix().clone() + vc2[0].get_perm_matrix().clone() + vc3[0].get_perm_matrix().clone());
    matrices.push(vc1[2].get_perm_matrix().clone() + vc2[1].get_perm_matrix().clone() + vc3[1].get_perm_matrix().clone());
    matrices.push(vc1[2].get_perm_matrix().clone() + vc2[2].get_perm_matrix().clone() + vc3[2].get_perm_matrix().clone());
    */

    /*
    for perm1 in &vc1 {
        matrices.push(perm1.get_perm_matrix());
    }

    for perm2 in &vc2 {
        matrices.push(perm2.get_perm_matrix());
    }
    for perm3 in &vc3 {
        matrices.push(perm3.get_perm_matrix());
    }

    */

    println!("{}", vc1.len());

    for i in 0..vc1.len() {
        for j in 0..vc2.len() {
            for k in 0..vc3.len() {
                let mat_check = idn.get_perm_matrix()
                    + vc1[i].get_perm_matrix()
                    + vc2[j].get_perm_matrix()
                    + vc3[k].get_perm_matrix();

                let mut_cayle = idn.get_perm_matrix()
                    + vc1[i].get_perm_matrix().mul_scalar(2i32)
                    + vc2[j].get_perm_matrix().mul_scalar(3i32)
                    + vc3[k].get_perm_matrix().mul_scalar(4i32);
                if mat_check.is_unital() {
                    matrices.push(mut_cayle.clone());
                    //println!("{:?} {:?} {:?}", vc1[i].top_row, vc2[j].top_row, vc3[k].top_row);
                    //println!("{:?} {:?} {:?}", vc1[i].bottom_row, vc2[j].bottom_row, vc3[k].bottom_row);
                    //println!();
                }
            }
        }
    }
    for mat in matrices {
        println!("{}", mat);
    }
}
pub fn test_permutation_list_filter() {
    let v = vec![1, 2, 3, 4, 5];

    let mut perms = PermutationList::new(5);

    perms.permute(v.clone());

    //let v = perms.get_cyclic_groups();
    let v = perms.get_cyclic_series_by_index_val(4, 1);

    println!("{}", perms.perms.len());
    println!("{}", v.len());

    for perm in v {
        println!("{}", perm);
    }
}   


pub fn test_permutation_list_filter_matrix() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let v = perms.get_cyclic_groups();

    for perm in &v {
        println!("{}", perm.get_perm_matrix());
    }

    println!();
    println!();
}

pub fn test_permutation_list_filter_off_diagonal() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let the_perms = perms.clone();
    let v = the_perms
        .perms
        .into_iter()
        .filter(|i| i.is_off_diagonal())
        .collect::<Vec<_>>();
    //println!("{}", v.len());
    let mut s = String::new();
    s.push_str("\\begin{aligned}");
    s.push_str("\\\\");

    let mut ind = 1;

    for perm in &v {
        s.push_str(perm.to_latex().as_str());

        s.push_str("\\;\\;");
        if ind % 3 == 0 {
            s.push_str("\\\\");
        }
        ind = ind + 1;
    }
    s.push_str("\\end{aligned}");

    let fil = String::from("test_permutation_list_filter_off_diagonal");

    pdf_methods::write_latex_and_launch(fil, s);
}

pub fn test_permutation_list_filter_latex_out() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let the_perms = perms.clone();
    let v = the_perms
        .perms
        .into_iter()
        .filter(|i| i.is_off_diagonal())
        .collect::<Vec<_>>();
    //println!("{}", v.len());
    let mut s = String::new();
    s.push_str("\\\\");

    for perm in &v {
        let p = perm.clone();

        if Permutation::has_row_column(p, 1i32, 2i32) {
            s.push_str(perm.to_latex().as_str());
            s.push_str("\\;\\;");
        }
    }

    let fil = String::from("permutation_list_filter_latex_out");

    pdf_methods::createpdf_r(fil, s);
}

pub fn test_latex() {
    let v = vec![1, 2, 3, 4, 5];

    let v1 = vec![3, 4, 5, 1, 2];

    let p1 = Permutation::new(v.clone(), v1);

    println!("{}", p1.get_perm_matrix().to_latex());
}

pub fn test_latex_out() {
    let v = vec![1, 2, 3, 4, 5];

    let v1 = vec![3, 4, 5, 1, 2];

    let p1 = Permutation::new(v.clone(), v1);

    let fil = String::from("mungout");

    pdf_methods::createpdf_r(fil, p1.get_perm_matrix().to_latex());
}

pub fn test_permutation_list_latex() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let the_perms = &perms;

    let mut ind = 1;

    let mut s = String::new();
    s.push_str("\\begin{aligned}");
    for perm in &the_perms.perms {
        s.push_str(perm.get_perm_matrix().to_latex().as_str());
        s.push_str("\\;\\;");

        if ind % 4 == 0 {
            s.push_str("\\\\");
        }

        ind = ind + 1;
    }
    s.push_str("\\end{aligned}");

    let fil = String::from("permutation_list_latex");

    pdf_methods::write_latex_and_launch(fil, s);
}

pub fn test_permutation_list_latex_order(order: i32) {
    let mut perms = PermutationList::new(order);

    perms.permute(perms.set_imut.clone());

    let the_perms = &perms;

    let mut ind = 1;

    let mut s = String::new();
    s.push_str("\\begin{aligned}");
    for perm in &the_perms.perms {
        s.push_str(perm.to_latex().as_str());
        s.push_str("\\;\\;");

        if ind % 4 == 0 {
            s.push_str("\\\\");
        }

        ind = ind + 1;
    }
    s.push_str("\\end{aligned}");

    println!("{}", s);
}
/* automatically generated by rust-bindgen 0.59.1 */

pub fn test_swap_row() {
    let v = vec![1, 2, 3, 4];

    let v1 = vec![1, 2, 3, 4];

    let p1 = Permutation::new(v.clone(), v1);

    let mut m2 = p1.get_perm_matrix();

    for i in 1..4 {
        m2 = m2.swap_row(i, i + 1);
        println!();
        println!("Swap {},{}\n{}", i, i + 1, m2);
    }
}

pub fn test_symetric_matrix() {
    let v = vec![1, 2, 3, 4];

    let v1 = vec![1, 2, 3, 4];

    let p1 = Permutation::new(v.clone(), v1);

    let mut m2 = p1.get_perm_matrix();

    IntMatrix::set_row_from_vec(&mut m2, v.clone(), 0);

    let mut vr = v.clone();
    vr.reverse();

    let r = (m2.rows - 1) as usize;
    IntMatrix::set_row_from_vec(&mut m2, vr.clone(), r);

    IntMatrix::set_col_from_vec(&mut m2, v.clone(), 0);

    let c = (m2.cols - 1) as usize;

    IntMatrix::set_col_from_vec(&mut m2, vr.clone(), c);

    for i in 0..m2.rows as usize {
        for j in 0..m2.cols as usize {
            if m2.get_value(i, j) == 0 {
                m2.set_value(i, j, m2.cols);
            }
        }
    }
    println!("{}", m2);
}

pub fn test_prod_cyclic_series() {
    let order = 5;

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

    for matrix in hi.iter() {
        println!("{}", matrix);
        println!();
    }

}
pub fn test_contains() {
    let v = vec![1, 2, 3, 4];

    let v1 = vec![1, 2, 3, 4];

    let v2 = vec![2, 1, 4, 3];

    let p1 = Permutation::new(v.clone(), v1);
    let p2 = Permutation::new(v.clone(), v2);

    let mut vv: Vec<Permutation> = Vec::new();

    vv.push(p1.clone());
    vv.push(p2.clone());

    let p2c = p2.clone();

    if vv.contains(&p2) {
        println!("contains\n{}", p2);
    } else {
        println!("doesn't contain {}", p2);
    }
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub fn test_cyclic_matrix() {
    let order = 4;

    let mut perms = PermutationList::new(order);

    perms.permute(perms.set_imut.clone());

    let mut less_perms = PermutationList::new(order - 1);

    less_perms.permute(less_perms.set_imut.clone());

    let mut vm: Vec<IntMatrix> = Vec::new();

    let is_unique_col = |matrix:IntMatrix| -> bool {
        let mut ret = true;

        for i in 0..matrix.cols as usize {
            if !has_unique_elements(matrix.get_col(i)) {
                ret = false;
                break;
            }
        }
        ret
    };

    let gcsb_br = |ind1: usize, val: i32, ind2: usize| -> Vec<i32> {
        PermutationList::get_cyclic_series_biv(&mut perms.clone(), ind1, val).clone()[ind2]
            .bottom_row
            .clone()
    };

    let ins_matrix_perms = |order: i32, less_perm: Permutation| -> IntMatrix {
        let mut mx = IntMatrix::new(order);
        IntMatrix::set_row_from_vec(&mut mx, perms.set_imut.clone(), 0);
        for i in 0..less_perm.order as usize {
            IntMatrix::set_row_from_vec(&mut mx, gcsb_br(i + 1, 1, (less_perm.bottom_row[i] as usize) - 1), i + 1);

        }
        mx
    };

    for less_perm in less_perms.perms {
        let matrix = ins_matrix_perms(order, less_perm);

        if is_unique_col(matrix.clone()) {
            vm.push(matrix.clone());
        }
    }

    for m in &vm {
        println!("{}", m);
    }
}

fn main() {
    //test_cyclic_matrix();
    //test_permutation_list_filter();
    //test_prod_cyclic_series();

    let hi: HashSet<IntMatrix> = finte_group::get_cyclic_groups(4);

    for matrix in hi.iter() {
        println!("{}", matrix);
    }
}
