use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let sum_of_multiplications = get_sum_of_multiplications(file);

    println!("sum of multiplications is {}", sum_of_multiplications);
}

fn get_sum_of_multiplications<R: Read>(reader: R) -> u32 {
    let mut string = String::new();
    BufReader::new(reader)
        .read_to_string(&mut string)
        .expect("unable to read to string");

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let sum_of_multiplications = re
        .captures_iter(string.as_str())
        .map(|m| {
            let num1 = m.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let num2 = m.get(2).unwrap().as_str().parse::<u32>().unwrap();
            num1 * num2
        })
        .sum();

    sum_of_multiplications
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::get_sum_of_multiplications;

    #[test]
    fn test_get_sum_of_multiplications() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = get_sum_of_multiplications(Cursor::new(input));
        assert_eq!(result, 161);
    }
}
