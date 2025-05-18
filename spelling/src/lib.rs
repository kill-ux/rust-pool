pub fn cases(n: u64) -> String {
    match n {
        0 => "".to_string(),
        _ => spell(n),
    }
}
pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let units = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let res = match n {
        0..10 => units[n as usize],
        10..20 => teens[(n % 10) as usize],
        20..100 => {
            let x = if n % 10 != 0 {
                &("-".to_string() + &cases(n % 10))
            } else {
                ""
            };
            &(tens[(n / 10) as usize].to_string() + x)
        }
        100..1_000 => &(units[(n / 100) as usize].to_string() + " hundred " + &cases(n % 100)),
        1_000..10_000 => {
            &(units[(n / 1_000) as usize].to_string() + " thousand " + &cases(n % 1_000))
        }
        10_000..1_000_000 => &(cases(n / 1_000) + " thousand " + &cases(n % 1_000)),
        1_000_000 => "one million",
        _ => "",
    };

    res.trim().to_string()
}
