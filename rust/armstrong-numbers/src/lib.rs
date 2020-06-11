pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let str_len = num_str.len() as u32;
    num == num_str
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(str_len))
        .sum()
}
