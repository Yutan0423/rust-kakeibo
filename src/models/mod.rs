use chrono::NaiveDate;
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
}
