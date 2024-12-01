use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let sum_of_differences = get_sum_of_differences(file);

    println!("sum of differences is {}", sum_of_differences);
}

fn get_sum_of_differences<R: Read>(reader: R) -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        let line = line.split_whitespace().collect::<Vec<_>>();
        left.push(line[0].parse::<u32>().unwrap());
        right.push(line[1].parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    let sum_of_differences = left
        .iter()
        .zip(right.iter())
        .map(|(&left_num, &right_num)| left_num.abs_diff(right_num))
        .sum::<u32>();

    sum_of_differences
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::get_sum_of_differences;

    #[test]
    fn test_get_sum_of_differences() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = get_sum_of_differences(Cursor::new(input));
        assert_eq!(result, 11);
    }
}
