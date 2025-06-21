pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }
    let mut is_upper = false;
    for ele in text.chars() {
        if ele.is_uppercase() && ele.is_alphabetic() {
            is_upper = true;
        } else if ele.is_alphabetic() {
            is_upper = false;
            break;
        }
    }

    if is_upper && text.chars().last().unwrap() == '?' {
        return "Quiet, I am thinking!";
    }

    if !is_upper && text.chars().last().unwrap() == '?' {
        return "Sure.";
    }

    if is_upper {
        return "There is no need to yell, calm down!";
    }
    "Interesting"
}
