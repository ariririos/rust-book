fn main() {
    println!("cargo::rerun-if-changed=c/lib.c");
    cc::Build::new().file("c/lib.c").compile("clib");
}