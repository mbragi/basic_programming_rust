//	  Borrowing in Functions
//      - Functions that immutably borrow values
//      - Functions that mutably borrow values
//      - Functions that not uses borrow but returns it
//      - Functions that uses mixed types of borrows

/*
- Borrowing Rules
    - At any time, you can have either one mutable reference or any number of immutable references.
    - References must always be valid.
*/

fn borrows_vec(vec: &Vec<i32>) {
    println!("vec is: {:?}", vec);
}

fn mutably_borrows_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}

fn gives_ownership() -> Vec<i32> {
    vec![4, 5, 6]
}

fn mixed_borrows(subject: &String, scores: &mut Vec<i32>) {
    println!("{} before update: {:?}", subject, scores);
    scores.push(99);
    println!("{} after update: {:?}", subject, scores);
}
fn main() {
    // Functions that immutably borrow values
    // let mut vec_1 = vec![1, 2, 3];
    // let ref_1 = &vec_1;
    // borrows_vec(ref_1);
    // let ref_2 = &mut vec_1;
    // println!("vec_1 is: {:?}", vec_1);

    // Functions that mutably borrow values
    let mut vec_1 = vec![1, 2, 3];
    let ref_1 = &mut vec_1;
    mutably_borrows_vec(ref_1);
    println!("vec_1 is: {:?}", vec_1);

    // Functions that do not uses borrow but returns it
    // Will be covered later under the topic of lifetimes

    // Functions that uses mixed types of borrows
    let mut scores = vec![85, 90, 85];
    let subject = String::from("Math");
    mixed_borrows(&subject, &mut scores);
}