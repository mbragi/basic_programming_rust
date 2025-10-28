fn main() {
  // Integers
    // Signed integers
    let a: i8 = 127; // i8, i16, i32, i64, i128
    println!("a is {a}"); 

    // Unsigned integers
    let b: u8 = 255; // u8, u16, u32, u64, u128
    println!("b is {b}");

    // Floating-point numbers
    let c: f32 = 10.5; // default floating-point number is f64
    println!("c is {c}");

     // Platform specific integers
     let arch_1: usize = 5;
     let arch_2: isize = 5;
 
     // Characters
     let char = 'a';
 
     // Boolean
     let b: bool = true;
 
     // Type Aliasing
     type Age = u8;
     let peter_age: Age = 42;
 
     // Type Conversion gives power to the programmer to convert one type to another
     let a: i32 = 10;
     let b = a as f64;
     println!("a is {a}");
     println!("b is {b}");
}