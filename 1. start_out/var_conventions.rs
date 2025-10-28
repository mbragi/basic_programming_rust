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