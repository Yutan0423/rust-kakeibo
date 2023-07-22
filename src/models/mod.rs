use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomeCategory {
    Salary,
    Bonus,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenceCategory {
    Food,
    Hobby,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Cartegory {
    Income(IncomeCategory),
    Expence(ExpenceCategory),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    name: String,
    category: Cartegory,
    price: u32,
    date: NaiveDate,
}

impl Item {
    pub fn new(name: String, category: Cartegory, price: u32, date: NaiveDate) -> Self {
        Self {
            name,
            category,
            price,
            date,
        }
    }

    pub fn get_category(register_type: u8, category_type: u8) -> Cartegory {
        if register_type == 0 {
            match category_type {
                0 => Cartegory::Income(IncomeCategory::Salary),
                1 => Cartegory::Income(IncomeCategory::Bonus),
                2 => Cartegory::Income(IncomeCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        } else {
            match category_type {
                0 => Cartegory::Expence(ExpenceCategory::Food),
                1 => Cartegory::Expence(ExpenceCategory::Hobby),
                2 => Cartegory::Expence(ExpenceCategory::Other),
                _ => panic!("不正なカテゴリ種別です"),
            }
        }
    }

    pub fn get_year(&self) -> i32 {
        self.date.year()
    }

    pub fn get_month(&self) -> u32 {
        self.date.month()
    }

    pub fn get_first_day(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.get_year(), self.get_month(), 1).unwrap()
    }

    pub fn get_price_for_summary(&self) -> i32 {
        match self.category {
            Cartegory::Income(_) => self.price as i32,
            Cartegory::Expence(_) => -(self.price as i32),
        }
    }
}
