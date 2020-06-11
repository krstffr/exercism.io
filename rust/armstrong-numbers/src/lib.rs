pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = format!("{}", num);
    let len = num_str.len().to_string().parse().unwrap();
    let mut sum = 0;
    for x in num_str.chars() {
        let n = x.to_string().parse::<u32>().unwrap();
        sum += u32::pow(n, len);
    }
    sum == num
}
