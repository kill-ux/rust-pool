use pangram::*;

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_not_pangram() {
        assert!(is_pangram(
            "Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich."
        ));
    }
}
