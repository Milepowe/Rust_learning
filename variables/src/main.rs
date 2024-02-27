fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

    }

    println!("The value of x is: {x}");
    let guess: u32 = "42".parse().expect("Not a number!");   
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // we get a compile time error because we can't mutate a variable type
    //let spaces = "   ";
    //let spaces = spaces.len();
    //if :u32 is missing we get an error
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
     let truncated = -5 / 3; // Results in -1
 
     // remainder
     let remainder = 43 % 5;

     let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
