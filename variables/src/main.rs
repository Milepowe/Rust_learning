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
    let spaces = "   ";
    let spaces = spaces.len();
}
