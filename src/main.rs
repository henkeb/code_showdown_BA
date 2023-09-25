mod problem1;
mod problem2;
mod problem3;

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
    show_id: String,
    r#type: String,
    title: String,
    director: String,
    cast: String,
    country: String,
    date_added: String,
    release_year: usize,
    rating: String,
    duration: String,
    listed_in: String,
    description: String,
}

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut reader =
        csv::Reader::from_path("/home/henkeb/Private/code_showdown/src/netflix_titles.csv")?;
    let mut all: Vec<Movie> = vec![];

    for result in reader.deserialize() {
        let movie: Movie = result?;
        all.push(movie);
    }

    println!("{}", problem1::lös(&all));
    println!("{}", problem2::lös(&all));
    println!("{}", problem3::lös(&all));
    Ok(())
}
