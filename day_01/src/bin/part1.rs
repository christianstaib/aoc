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
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        let line = line
            .split_whitespace()
            .flat_map(&str::parse::<u32>)
            .collect::<Vec<_>>();

        first_column.push(line[0]);
        second_column.push(line[1]);
    }

    first_column.sort_unstable();
    second_column.sort_unstable();

    let sum_of_differences = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(&first_number, &second_number)| first_number.abs_diff(second_number))
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
