// update build.rs file as:
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/perm_ops/cfiles/createpdf.c")
        .compile("libcreatepdf.a");
}