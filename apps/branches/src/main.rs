fn main() {
    // Testing conditionals
    let x = 10;

    if x < 5 {
        println!("x is less than 5")
    } else {
        println!("x is not less than 5")
    }

    // Using if as an expression
    println!("Here I'm using if as an expression to print something");
    println!("If x is less than 2, you would see 'x is less than 2' else you will see 'x is not less than 2'");
    println!("{}", if x < 2 {"x is less than 2"} else {"x is not less than 2"});

    let y = if x == 10 {
        let a = 2;
        let b = a * 10;
        a * b
    } else {0};

    println!("y is {y}");
}
