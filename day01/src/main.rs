fn main() {
    let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    let result: i32 = input
        .lines()
        .map(get_values_from_line)
        .map(get_first_and_last)
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("{:?}", result);
}

fn get_values_from_line(line: &str) -> String {
    line
        .chars()
        .filter(|c| c.is_digit(10))
        .collect()
}

fn get_first_and_last(s: String) -> i32 {
    let v = s.chars().collect::<Vec<char>>();
    let mut first = v.first().unwrap().to_string();
    let last = v.last().unwrap().to_string();
    first.push_str(&last);
    first.parse().unwrap()
}
