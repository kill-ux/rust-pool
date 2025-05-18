pub fn number_logic(num: u32) -> bool {
    let n = num.to_string();
    let le = n.len();
    let mut sum = 0;
    for nb in n.chars() {
       sum += nb.to_string().parse::<u32>().unwrap().pow(le as u32);
    }
    println!("{sum}");
    sum == num 
}