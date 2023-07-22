use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike, NaiveDate};

use crate::models;
use crate::services;

pub fn run(file_path: &str) {
    println!("家計簿の集計を行います");
    let data = services::io::read_data_or_panic(file_path);

    let target_dates = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();

    for date in target_dates {
        let filtered_data = get_filtered_data(&data, date);
        let sum = summarize_data(&filtered_data);
        result_table.insert(date, sum);
    }
    print_table(result_table);
}

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| item.get_first_day()).collect();
    target_dates
}

fn get_filtered_data(data: &Vec<models::Item>, filtered_date: NaiveDate) -> Vec<&models::Item> {
    let filtered_data: Vec<_> = data
        .iter()
        .filter(|item| {
            item.get_year() == filtered_date.year() && item.get_month() == filtered_date.month()
        })
        .collect();
    filtered_data
}

fn summarize_data(data: &Vec<&models::Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary()
    }
    sum
}

fn format_date(data: NaiveDate) -> String {
    format!("{}年{}月", data.year(), data.month())
}

fn format_price(price: i32) -> String {
    if price >= 0 {
        format!("+{}", price)
    } else {
        price.to_string()
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date = format_date(result.0);
        let price = format_price(result.1);
        println!("{}の収支は{}円でした", date, price);
    }
}

#[cfg(test)]
mod summarize_test {
    use super::*;

    fn get_test_data() -> Vec<models::Item> {
        vec![
            super::models::Item::new(
                "給与".to_string(),
                super::models::Cartegory::Income(super::models::IncomeCategory::Salary),
                300000,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            ),
            super::models::Item::new(
                "食費".to_string(),
                super::models::Cartegory::Expence(super::models::ExpenceCategory::Food),
                3000,
                NaiveDate::from_ymd_opt(2020, 2, 1).unwrap(),
            ),
            super::models::Item::new(
                "食費".to_string(),
                super::models::Cartegory::Expence(super::models::ExpenceCategory::Food),
                3000,
                NaiveDate::from_ymd_opt(2020, 3, 1).unwrap(),
            ),
        ]
    }

    #[test]
    fn test_get_target_dates() {
        let data = get_test_data();
        let mut expected = BTreeSet::new();
        expected.insert(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap());
        expected.insert(NaiveDate::from_ymd_opt(2020, 2, 1).unwrap());
        expected.insert(NaiveDate::from_ymd_opt(2020, 3, 1).unwrap());
        
        let actual = get_target_dates(&data);
        assert_eq!(actual, expected);
    }
}
