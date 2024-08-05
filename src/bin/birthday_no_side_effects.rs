use chrono::{Datelike, NaiveDate, Utc};

fn main() -> () {
    let today = today();
    let date_of_birth = date_of_birth();
    let birthday = birthday(today, date_of_birth);

    if birthday == today {
        println!(
            "Happy birthday! Congratulations on becoming {}!",
            age(today, date_of_birth)
        );
    } else {
        if birthday < today {
            println!("You've already had your birthday. I hope you had fun!")
        } else {
            println!(
                "It's not your birthday yet. Wait for {} more days.",
                days_until_birthday(today, birthday)
            );
        }
    }
}

fn days_until_birthday(today: NaiveDate, birthday: NaiveDate) -> i64 {
    birthday.signed_duration_since(today).num_days()
}
fn age(today: NaiveDate, date_of_birth: NaiveDate) -> u32 {
    today.years_since(date_of_birth).unwrap()
}
fn date_of_birth() -> NaiveDate {
    let date_string = std::env::args().nth(1).unwrap();
    NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").unwrap()
}
fn birthday(today: NaiveDate, date_of_birth: NaiveDate) -> NaiveDate {
    date_of_birth.with_year(today.year()).unwrap()
}

fn today() -> NaiveDate {
    Utc::now().date_naive()
}