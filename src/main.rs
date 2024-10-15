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
        if i % 2 == 0 {
            print!("{} ", i);
        }
    }
    println!();
}
