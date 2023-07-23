use cmake::Config;
fn main() {
    println!("cargo:rerun-if-changed=../add_num_cpp_cmake/add_num.cpp");
    let dst = Config::new("../add_num_cpp_cmake")
        .build_target("all")
        .build();
    println!("path: {}", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=add_num");
}
