pub fn spell(n: u64) -> String {
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
        0 => "zero",
        0..10 => units[n as usize],
        10..20 => teens[(n % 10) as usize],
        20..100 => &(tens[(n / 10) as usize].to_string() + "-" + &spell(n % 10)),
        100..1_000 => &(units[(n / 100) as usize].to_string() + " hundred " + &spell(n % 100)),
        1_000..10_000 => {
            &(units[(n / 1_000) as usize].to_string() + " thousand " + &spell(n % 1_000))
        }
        10_000..1_000_000 => &(spell(n / 1_000) + " thousand " + &spell(n % 1_000)),
        1_000_000 => "one million",
        _ => "",
    };

    res.to_string()
}
