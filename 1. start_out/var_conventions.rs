/// Demonstrates variable naming conventions, marking intentionally unused bindings, and reusing static and constant values.
///
/// Shows declaring intentionally unused bindings with a leading underscore (`_number_one`, `_x`), a static string slice `WELCOME`, a constant `PI`, and creating local bindings that reuse those static/const values.
///
/// # Examples
///
/// ```
/// // This example compiles and illustrates the declarations; it does not produce output.
/// fn main() {
///     let _number_one = 45;
///     let _x = 40_000;
///     static WELCOME: &str = "Welcome to Rust";
///     const PI: f32 = 3.14;
///     let a = PI;
///     let b = PI;
///     let y = WELCOME;
///     let z = WELCOME;
///     // prevent unused variable warnings in doctest
///     let _ = (a, b, y, z, _number_one, _x);
/// }
/// ```
fn main() {
  // Variable Convention and Unused Variables
  let _number_one = 45;
  let _x = 40_000;

  // Statics
  static WELCOME: &str = "Welcome to Rust";
  const PI: f32 = 3.14;

  let a = PI;
  let b = PI;

  let y = WELCOME;
  let z = WELCOME;
}