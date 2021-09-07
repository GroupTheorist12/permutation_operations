use std::ffi::CString;
use std::os::raw::c_char;

/* automatically generated by rust-bindgen 0.59.1 */

extern "C" {
    pub fn createpdf(fil: *mut ::std::os::raw::c_char, latex: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn createpdffromtex(fil: *mut ::std::os::raw::c_char, tex: *mut ::std::os::raw::c_char);
}

pub fn createpdf_r(fil: String, latex: String) {
    /*
    let fil1 = fil.clone();
    let bytes1: Vec<u8> = fil.into_bytes();
    let bytes2: Vec<u8> = latex.into_bytes();


    let mut c_chars1: Vec<i8> = bytes1.iter().map(| c | *c as i8).collect::<Vec<i8>>();
    
    c_chars1.push(0); // null terminator

    let mut c_chars2: Vec<i8> = bytes2.iter().map(| c | *c as i8).collect::<Vec<i8>>();
    
    c_chars2.push(0); // null terminator

    let ptr1: *mut c_char = c_chars1.as_mut_ptr();
    let ptr2: *mut c_char = c_chars2.as_mut_ptr();

    let c_str = CString::new(fil1).expect("Got inner null byte!");
    let ptr3: *mut c_char = c_str.as_ptr() as *mut c_char;
    unsafe {
        createpdf(ptr1, ptr2);    

    }
    */

    let c_str1 = CString::new(fil).expect("Got inner null byte!");

    let ptr1: *mut c_char = c_str1.as_ptr() as *mut c_char;

    let c_str2 = CString::new(latex).expect("Got inner null byte!");

    let ptr2: *mut c_char = c_str2.as_ptr() as *mut c_char;


    unsafe {
        createpdf(ptr1, ptr2);    

    }

}

pub fn write_latex_and_launch(fil: String, latex: String) {
    let s = r#"
\documentclass[]{article}
\usepackage{amssymb}
\usepackage{amsmath}

\setcounter{MaxMatrixCols}{20}

\begin{document}
\begin{equation}
REPLACE_ME_WITHH_LATEX
\end{equation}
\end{document}
"#;
let  s2 = &s.replace("REPLACE_ME_WITHH_LATEX", &latex);
let s3 = s2.to_string();

launch_pdf(fil, s3);

}

pub fn launch_pdf(fil: String, tex: String) {
    let c_str1 = CString::new(fil).expect("Got inner null byte!");

    let ptr1: *mut c_char = c_str1.as_ptr() as *mut c_char;

    let c_str2 = CString::new(tex).expect("Got inner null byte!");

    let ptr2: *mut c_char = c_str2.as_ptr() as *mut c_char;

    unsafe {
        createpdffromtex(ptr1, ptr2);
    }
}
