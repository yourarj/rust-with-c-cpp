// function defined in C++
extern "C" {
    fn add(first: i32, second: i32) -> i32;
}

fn main() {
    println!("Hello, world!");

    // Following UNSAFE API calls should be refactored to separate module
    // for safety
    // SAFETY: we know add exist and conforms to specifed
    // planning to adding multiple different approaches for the same
    // example bindgen, cxx, cc
    // PS: If time permits
    let sum_from_cpp = unsafe { add(23, 56) };
    println!(
        "Addition of the numbers is performed in CPP {}",
        sum_from_cpp
    );
}
