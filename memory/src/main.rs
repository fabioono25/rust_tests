// fn main() {
//     let mut s = String::from("Hello"); //heap

//     s.push_str(", world!");

//     println!("{}", s);

//     let s1 = String::from("hi");
//     let s2 = s1;

//     println!("{}", s1);
// }

fn main() {
    let mut s = String::from("hello");  // s comes into scope.

    takes_ownership(&mut s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    println!("{}", s);
    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: &mut String) { // some_string comes into scope.
    println!("{}", some_string);
    some_string.push_str(" World!");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

