#![allow(unused_variables)]
/// Returns the square of an integer.
///
/// # Examples
///
/// ```
/// let r = square(4);
/// assert_eq!(r, 16);
/// ```
fn square(x: i32) -> i32 {
    let temp = 42;
    x * x
}
/// Program entry point that initializes example local values.
///
/// This function creates a sample integer and string and then exits; it has no observable side effects.
///
/// # Examples
///
/// ```
/// fn main_wrapper() {
///     // calling `main` runs the program's entry point
///     main();
/// }
/// main_wrapper();
/// ```
fn main() {
    let i = 10;
    let s = String::from("Hi there");
}