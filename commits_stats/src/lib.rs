
use chrono::*;
use json::JsonValue;
use std::collections::HashMap;

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week = HashMap::new();
    for commit in data.members() {
        let date = commit["commit"]["author"]["date"].as_str().unwrap();
        let date = date.split('T').collect::<Vec<&str>>()[0];
        let date = date.split('-').collect::<Vec<&str>>();
        let year = date[0];
        let month = date[1];
        let day = date[2];
        let week = chrono::NaiveDate::from_ymd_opt(
            year.parse().unwrap(),
            month.parse().unwrap(),
            day.parse().unwrap(),
        )
        .unwrap()
        .iso_week()
        .week();
        let week = format!("{}-W{}", year, week);
        let count = commits_per_week.entry(week).or_insert(0);
        *count += 1;
    }
    commits_per_week
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author = HashMap::new();
    for commit in data.members() {
        let author = commit["author"]["login"].as_str().unwrap();
        let count = commits_per_author.entry(author.to_string()).or_insert(0);
        *count += 1;
    }
    commits_per_author
}