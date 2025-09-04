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

    // Testing loops
    let mut counter = 0;

    println!("dgfhg");

    let result = 'outer: loop {
        // just another loop
        loop {
            // println!("Loop until stopped with a signal");
            counter += 2;
            println!("Incrementing counter by 2");

            if counter >= 234 {
                break 'outer counter;
            }
        }
    };

    println!("The value of result is {result}");

    println!("Testing the while loop");

    while counter > 0 {
        println!("Counter {counter}");
        counter -= 1;
    }

    // Try with inclusive range 1..=40 and see the difference
    for index in (1..=40).rev() {
        println!("Current index {index}");
    }

    for element in [4, 5, 9, 10] as [u8; 4] {
        println!("Array elements {element}");
    }
}