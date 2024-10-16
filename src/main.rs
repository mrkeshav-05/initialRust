fn main() {
    // Iteration over String
    let sentance = String::from("My name is Keshav Thakur");
    let first_word = get_first_word(sentance);
    println!(); 
}

fn get_first_word(sentance: String) -> String {
    let mut first_word = String::new();
    for char in sentance.chars() {
        if char == ' ' {
            break;
        }
        first_word.push(char);
    }
    first_word // this is for returing the first character
}
