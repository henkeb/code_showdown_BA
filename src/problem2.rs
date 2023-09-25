use chrono::Datelike;

use crate::Movie;
pub fn lös(input: &[Movie]) -> usize {
    input
        .iter()
        .map(|movie| &movie.date_added)
        .map(|date| chrono::NaiveDate::parse_from_str(date.trim(), "%B %d, %Y"))
        .filter_map(|date| date.ok())
        .filter(|date| date.weekday() == chrono::Weekday::Sat)
        .count()
}
