fn main() {
    print!("--- Conditional Loops ---");
    let is_even = true;
    println!();
    if is_even {
        println!("Even");
    } else {
        println!("Odd");
    }
    for i in 0..10 {
        print!("{} ", i);
    }
    // i is from 0 to 9 basically
    println!();
}
