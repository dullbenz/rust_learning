fn main() {
    println!("Hello, world!");

    another_function(45);

    // Testing what a code block evaluates to
    let y = {
        let x = 3;
        // With and without the semicolon here do two completely different things
        // With semicolon, it turns to a statement and the block return a unit
        // Without semicolon, it remains an expression and the block resolves to the value of the expression
        x + 1
    };

    println!("The value of y is {y}");
    
    println!("The function ten() always evaluates to {}", ten())
}

fn another_function(x: u8) {
    println!("The value of x is {x}");
}

// Exploring functions that return a value
fn ten() -> u8 {
    10
}