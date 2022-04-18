// ! Unwinding the Stack or Aborting in Response to a Panic

// By default, when a panic occurs, the program starts unwinding, which means
// Rust walks back up the stack and cleans up the data from each function it
// encounters. However, this walking back and cleanup is a lot of work. Rust,
// therefore, allows you to choose the alternative of immediately aborting,
// which ends the program without cleaning up. Memory that the program was using
// will then need to be cleaned up by the operating system. If in your project
// you need to make the resulting binary as small as possible, you can switch
// from unwinding to aborting upon a panic by adding panic = 'abort' to the
// appropriate [profile] sections in your Cargo.toml file. For example, if you
// want to abort on panic in release mode, add this:

// [profile.release]
// panic = 'abort'

fn main() {
    // panic!("Hello, world!");

    let v = vec![1, 2, 3];

    v[99];
    // Here, we’re attempting to access the 100th element of our vector (which
    // is at index 99 because indexing starts at zero), but the vector has only
    // 3 elements. In this situation, Rust will panic. Using [] is supposed to
    // return an element, but if you pass an invalid index, there’s no element
    // that Rust could return here that would be correct.

    // run `RUST_BACKTRACE=1 cargo run` to see backtrace
}
