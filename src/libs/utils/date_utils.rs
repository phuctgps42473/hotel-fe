use chrono::{DateTime, NaiveDate, Utc};

pub fn get_check_in_out(date_range: String) -> Option<(String, String)> {
    date_range
        .split_once(" to ")
        .and_then(|(ckin, ckout)| Some((ckin.to_string(), ckout.to_string())))
}

pub fn calculate_days_between_range(range: String) -> i64 {
    if let Some((from, to)) = get_check_in_out(range) {
        let format = "%Y-%m-%d";

        let start = NaiveDate::parse_from_str(&from, format).unwrap();
        let end = NaiveDate::parse_from_str(&to, format).unwrap();

        let dur = end - start;
        dur.num_days()
    } else {
        0
    }
}

pub fn timestamp_to_html_date(input: String) -> String {
    let datetime: DateTime<Utc> = input.parse().unwrap_or("2025-06-30T17:00:00.000+00:00".parse().unwrap());
    let formatted_date = datetime.format("%Y-%m-%d").to_string();
    formatted_date
}