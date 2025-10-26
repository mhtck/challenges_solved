use std::collections::HashMap;

pub fn run() {
    let case_1 = String::from("()");
    let case_2 = String::from("([)])");
    let case_3 = String::from("(]");

    println!("result: {}", is_valid(case_3));
}

fn is_valid(s: String) -> bool {
    let parentheses: HashMap<char, char> = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);

    let mut tmp = String::new();

    println!("paranthese : {:?}", parentheses);

    for i in s.chars() {
        if tmp.is_empty() || parentheses.contains_key(&i) {
            println!("result : {}", i);
            tmp.push(i);
        } else {
            if let Some(last_char) = tmp.chars().last() {
                if let Some(&closing) = parentheses.get(&last_char) {
                    if closing == i {
                        tmp.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
    }

    if tmp.is_empty() {
        return true;
    } else {
        return false;
    }
}
