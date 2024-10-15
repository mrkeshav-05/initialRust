fn main() {
    let greeting = String::from("Hello, world!");
    println!("{}", greeting);
    let char1 = greeting.chars().nth(2).unwrap();

    // match char1 {
    //     Some(c) => println!("First character: {}", c),
    //     None => println!("No first character."),
    // }

    print!("Characters: {}", char1);

    println!();
}
