pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!" ;
    }

    let mut is_upper = false;
    let mut enter = false;
    for ch in text.chars() {
        if ch.is_alphabetic() && ch.is_uppercase() {
            is_upper = true;
            enter = true ;
        } else if ch.is_alphabetic() && ch.is_lowercase() {
            is_upper = false ;
            break;
        }
    }
    
    println!("{is_upper}");

    if &text[text.len()-1..] == "?" && !is_upper  {
        return "Sure.";
    }
    
    if &text[text.len()-1..] == "?" && is_upper {
        return "Quiet, I am thinking!";
    }
    
    if is_upper {
        return "There is no need to yell, calm down!" ;
    }
    
    "Interesting"
}