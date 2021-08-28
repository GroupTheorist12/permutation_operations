mod int_matrix;
use int_matrix::IntMatrix;

mod permutation;
use permutation::Permutation;

mod permutation_list;
use permutation_list::PermutationList;


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
    println!("The product of a permutation matrix and it's inverse is the identity\n{} {}", idn, idn.clone().identity());

}

pub fn test_permutation() {
    let v = vec![1, 2, 3, 4, 5];

    let v1 = vec![3, 4, 5, 1, 2];
    let v2 = vec![3, 5, 1, 2, 4];

    let  p1 = Permutation::new(v.clone(), v1);
    let  p2 = Permutation::new(v.clone(), v2);

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

    for perm in &the_perms.perms {
        print!("{}", perm.get_perm_matrix());
        println!();
    }

}

fn main() {
    test_permutation_list();
}
