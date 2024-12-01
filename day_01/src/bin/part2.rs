use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();

    let similarity_score = get_similarity_score(file);

    println!("similarity score is {}", similarity_score);
}

fn get_similarity_score<R: Read>(reader: R) -> u32 {
    let buf_reader = BufReader::new(reader);

    let mut first_column = HashMap::new();
    let mut second_column = HashMap::new();

    for line in buf_reader.lines().map_while(Result::ok) {
        let line = line
            .split_whitespace()
            .flat_map(&str::parse::<u32>)
            .collect::<Vec<_>>();

        *first_column.entry(line[0]).or_insert(0) += 1;
        *second_column.entry(line[1]).or_insert(0) += 1;
    }

    let similarity_score = first_column
        .iter()
        .map(|(&number, &apperance_in_first_column)| {
            let apperance_in_second_column = second_column.get(&number).unwrap_or(&0);
            apperance_in_first_column * (number * apperance_in_second_column)
        })
        .sum::<u32>();

    similarity_score
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::get_similarity_score;

    #[test]
    fn test_get_similarity_score() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = get_similarity_score(Cursor::new(input));
        assert_eq!(result, 31);
    }
}
