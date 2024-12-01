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

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line in buf_reader.lines().map_while(Result::ok) {
        let line = line.split_whitespace().collect::<Vec<_>>();
        *left.entry(line[0].parse::<u32>().unwrap()).or_insert(0) += 1;
        *right.entry(line[1].parse::<u32>().unwrap()).or_insert(0) += 1;
    }

    let similarity_score = left
        .iter()
        .map(|(&number, &apperance_left)| {
            let apperance_right = right.get(&number).unwrap_or(&0);
            apperance_left * (number * apperance_right)
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
