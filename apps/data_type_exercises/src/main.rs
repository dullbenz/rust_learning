fn main() {
    println!("Hello, world!");

    println!("0 degrees celsius is {} fahrenheit", celsius_to_fahrenheit(0f32));
    println!("{} degrees fahrenheit is {} celsius", celsius_to_fahrenheit(4f32), fahrenheit_to_celsius(celsius_to_fahrenheit(4f32)));

    print!("The first 10 fibonacci numbers are: ");
    for i in 0..10 {
        print!("{} ", fibonacci_number(i));
    }

    println!("\n\n\n");
    print_twelve_days_of_christmas();
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    temp * 9f32 / 5f32 + 32f32
}

fn fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32f32) * 5f32 / 9f32
}

fn fibonacci_number(n: u8) -> u32 {
    let mut prev: u32 = 0;
    let mut next: u32 = 1;

    if n == 0 { return prev };

    for _ in 2..=n {
        next += prev;
        prev = next - prev;
    }

    next
}

fn print_twelve_days_of_christmas() {
    let lyrics_array = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "four calling birds,",
        "five golden rings,",
        "six geese a-laying,",
        "seven swans a-swimming,",
        "eight maids a-milking,",
        "nine ladies dancing,",
        "ten lords a-leaping,",
        "eleven pipers piping,",
        "twelve drummers drumming,"
    ];
    let day_text = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];
    
    for state in 0..12 {
        println!("On the {} day of Christmas my true love sent to me\n", day_text[state]);

        for lyric_text in lyrics_array[..=state].iter().rev() {
            println!("{}", lyric_text);
        }

        println!("\n\n");
    }
}