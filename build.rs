fn main() {
    println!("cargo:rustc-link-arg=/MERGE:.rdata=.text"); // Merge sections
    println!("cargo:rustc-link-arg=/STACK:8388608"); // Set stack size to 8MB
    println!("cargo:rustc-link-arg=/DYNAMICBASE"); // Enable ASLR
    println!("cargo:rustc-link-arg=/CETCOMPAT"); // Enable CET Shadow Stack
    println!("cargo:rustc-link-arg=/NXCOMPAT"); // Enable DEP
    println!("cargo:rustc-link-arg=/GUARD:CF"); // Enable Control Flow Guard
    println!("cargo:rustc-link-arg=/OPT:REF"); // Eliminate unreferenced functions/data
    println!("cargo:rustc-link-arg=/OPT:ICF"); // Perform identical COMDAT folding
    println!("cargo:rustc-link-arg=/RELEASE"); // Optimize for release
    println!("cargo:rustc-link-arg=/LTCG"); // Enable Link-time Code Generation
}
