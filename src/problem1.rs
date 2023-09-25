use crate::Movie;

pub fn lös(input: &[Movie]) -> usize {
    input
        .iter()
        .map(|movie| count_director_longer_than_12(&movie.director))
        .sum()
}

fn count_director_longer_than_12(director: &str) -> usize {
    director
        .split(", ")
        .map(|names: &str| names.split_whitespace().take(1).collect::<String>())
        .filter(|name| name.chars().count() >= 12)
        .count()
}
