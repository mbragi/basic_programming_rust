//	    Dereferencing
//      - Dereferencing of stack allocated types
//      - Dereferencing of heap allocated types

// Borrowing = sharing access
// Dereferencing = using value behind a reference
fn main() {
    // Dereferencing of stack allocated types
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("some_data is {some_data}, deref_copy is: {deref_copy}");

    // Owned types: No & sign at the start of the type
    // Borrowed types: starts with & sign at the start of the type

    // Dereferencing of heap allocated types
    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &mut heap_data;
    // let deref_copy = *ref_1;
    ref_1.push(8);
    (*ref_1).push(8);
}