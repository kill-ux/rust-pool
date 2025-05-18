

pub fn pig_latin(text: &str) -> String {
    let vowels = "aeiou";
    if !vowels.contains(text.chars().next().unwrap()) && &text[1..3] == "qu" {
        return  text[3..].to_string() + &text[..3] + "ay" ;
    }

    for (i,ch) in text.char_indices() {
        if vowels.contains(ch) {
            return  text[i..].to_string() + &text[..i] + "ay" ;
        }
    }

    text.to_string()
}