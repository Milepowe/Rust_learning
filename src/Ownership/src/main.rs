fn main() {
    // s is not valid there, it's not yet declared
    let s = "hello"; // s is valid from this point forward

    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1); will produce an error

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function
                        // and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is copy so it's okay, to still use x

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back
                                       // which moves it's return value to s3

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("the length of '{}' is {}.", s2, len);
} // this scope is over so s is no longer valid/ x goes out of scope
  // s3 goes out of scope and is dropped, s2 is moved so nothing happens. s1 goes out of scope

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // some_string goes out of scope and 'drop' is called, the memory is freed

fn makes_copy(some_integer: i32) {
    // some some_integer comes into scope
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens

fn gives_ownership() -> String {
    // gives_ownership will move the value into the function that
    // calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out of the calling function
}

// this function takes a String and returns one
fn takes_and_gives_back(aString: String) -> String {
    // aString comes into scope
    aString // aString is called moves out of the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
