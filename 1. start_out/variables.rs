fn main() {
    // Defining variables
    let x: f64 = 300.23;
    println!("x is {x}");

    //mutable variables
    let mut y: f64 = 300.23;
    println!("y is {y}");
    y = 400.23;
    println!("y is {y}");

    // Scope of variables

    {
        let z: f64 = 300.23;
        println!("z is {z}");
    }
    // println!("z is {z}"); // This will cause an error because z is not in scope

    // Shadowing variables
    let a: f64 = 300.23;
    println!("a is {a}");
    let a: f64 = a + 400.23;
    println!("a is {a}");
     
    let v: i32 = 10;
    {
      let v: i32 = 20;
      println!("v is {v}");
    }
    println!("v is {v}");

    //Constants
    const MAX_POINTS: i32 = 100_000;
    println!("MAX_POINTS is {MAX_POINTS}");
}
