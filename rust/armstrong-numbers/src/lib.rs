pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = format!("{}", num);
    let str_len = num_str.len() as u32;
    let sum: u32 = num_str
        .chars()
        .map(|x| u32::pow(x.to_string().parse().unwrap(), str_len))
        .sum();
    sum == num
}
