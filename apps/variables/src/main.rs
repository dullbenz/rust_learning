fn main() {
    let mut x;
    x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Testing usage of constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

    // Testing usage of checked_..., overflowing_..., wrapping_... and saturating_... methods
    println!("Testing saturating add: {} + 1 = {}", u8::MAX, u8::MAX.saturating_add(1));

    // Testing parse of string to u8
    // const A_337_STRING: &str = "337";

    // println!("Testing parsing into u8 of value 337 {}", A_337_STRING.parse::<u8>().expect("337 not to be parsable to an u8"));

    // assert_eq!(A_337_STRING.parse::<u8>().unwrap(), 255);

    // println!("Testing checked subtract: 0 - 1 = {}", u8::MIN.checked_sub(1).unwrap());

    println!("Testing wrapping add: {} + 1 = {}", u8::MAX, u8::MAX.wrapping_add(1));
    assert_eq!(u8::MAX.overflowing_add(1), (0, true) );

    // Testing tuples
    let tup = (5, 8.5, "test");

    let (x, y, z) = tup;

    println!("The values of the tuple are {x}, {y}, {z}");

    println!("Here I'm printing the values from the tuple directly with index accessors: {}, {}, {}", tup.0, tup.1, tup.2);

    // Testing the unit type
    let _x;
    assert_eq!((), ());
    assert_eq!(_x = 5, ());
    assert_eq!(no_return_value(), ());

    // A tuple is not modifiable in the sense that elements cannot be added or removed but individual elements can be modified
    let mut mut_tup = (4,  5);
    mut_tup.1 += 8;

    println!("mut_tup.1 is {}", mut_tup.1);

    // Testing array types
    let test_array: [u8; 3] = [1, 2, 3];

    println!("The first element of my array is {}", test_array[0]);

    // Testing array initializer of copies of the same number
    let t = ([3; 20], ['c'; 5], "test");
    // So apparently I can destructure a tuple but not an array in rust, or at least not in the way I would think
    // Also, apparently, when destructuring a tuple, I must specify all its elements else the following will occur
    // 1. if I specify a single variable in the destructured, it maps to the tuple itself
    // 2. if I specify 2 or more, it throws an error if all the elements were not destructured
    let (a, _b, _c) = t;
    println!("Testing tuples and arrays initialized to copies of the same number. The elements in the tuples arrays are {}, {}", a[0], t.1[0]);
}

fn no_return_value() {
    println!("My function which returns no value should actually return () i.e the unite");
}
