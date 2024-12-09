fn main() {
    // Basic Configuration
    println!("cargo:rustc-link-arg=/MERGE:.rdata=.text");
    println!("cargo:rustc-link-arg=/STACK:8388608");

    // Security Features
    println!("cargo:rustc-link-arg=/DYNAMICBASE");
    println!("cargo:rustc-link-arg=/CETCOMPAT");
    println!("cargo:rustc-link-arg=/NXCOMPAT");
    println!("cargo:rustc-link-arg=/GUARD:CF");
    println!("cargo:rustc-link-arg=/HIGHENTROPYVA");

    // Optimization Settings
    println!("cargo:rustc-link-arg=/OPT:REF,ICF,LBR");
    println!("cargo:rustc-link-arg=/LTCG");
    println!("cargo:rustc-link-arg=/INCREMENTAL:NO");

    // Disable debug information
    println!("cargo:rustc-link-arg=/DEBUG:NONE");
    println!("cargo:rustc-link-arg=/NOCOFFGRPINFO");
}
