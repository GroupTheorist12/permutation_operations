mod int_matrix;
use int_matrix::IntMatrix;

mod permutation;
use permutation::Permutation;

mod permutation_list;
use permutation_list::PermutationList;

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

pub fn test_permutation_list_filter() {
    let v = vec![1, 2, 3, 4];

    let mut perms = PermutationList::new(4);

    perms.permute(v.clone());

    let the_perms = perms.clone();
    let v = the_perms
        .perms
        .into_iter()
        .filter(|i| i.is_off_diagonal())
        .collect::<Vec<_>>();
    println!("{}", v.len());
    for perm in &v {
        let p = perm.clone();

        if Permutation::has_row_column(p, 1i32, 2i32) {
            print!("{}", perm.to_latex());
            println!();
        }
    }
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

fn main() {
    test_permutation_list_filter_off_diagonal();
}
