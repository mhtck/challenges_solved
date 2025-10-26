pub fn run() {
    let s = String::from("AAAAAAAAAAAAAAAAAAAa");
    let result = greatest_letter(s);

    println!("Result : {}", result);
}

fn greatest_letter(s: String) -> String {
    let mut result = String::new();

    for (i, val) in s.char_indices() {
        let mut counter = 0;
        if s.find(val.to_uppercase().next().unwrap()) != None
            && s.find(val.to_lowercase().next().unwrap()) != None
        {
            counter += 2;
        }
        if counter >= 2 && (result.is_empty() || result <= val.to_uppercase().collect::<String>()) {
            println!("counter : {}", counter);
            result = val.to_uppercase().collect::<String>();
        }
    }
    result
}
