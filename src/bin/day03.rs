fn main() {
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    let nums = get_nums(input);
    println!("the part numbers are: {:?}", nums);

    let sum: u32 = nums.iter().sum();
    println!("their sum is : {}", sum);
}

fn get_nums(input: &str) -> Vec<u32> {
    let nums = Vec::new();

    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut s = Vec::new();

    for j in 0..height {

        for i in 0..width {
            if lines[j][i].is_digit(10) {
                let mut value = String::new();

                for k in i..width {
                    if lines[j][k].is_digit(10) {
                        value.push(lines[j][k]);
                    } else {
                        s.push(value);
                        break;
                    }
                }
            }
        }
    }

    dbg!(s);

    nums
}
