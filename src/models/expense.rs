use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use std::fmt;


// #[derive(Debug, Serialize, sqlx::FromRow)]
// pub struct ExpenseId {
//     pub id: Uuid,
// }


#[derive(Serialize, Deserialize, Clone, sqlx::Type, Debug)]
#[sqlx(type_name = "expense_category")]
pub enum ExpenseCategory {
    Groceries,
    Leisure,
    Electronics,
    Utilities,
    Clothing,
    Health,
    Others,
}


impl fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category_str = match self {
            ExpenseCategory::Groceries => "Groceries",
            ExpenseCategory::Leisure => "Leisure",
            ExpenseCategory::Electronics => "Electronics",
            ExpenseCategory::Utilities => "Utilities",
            ExpenseCategory::Clothing => "Clothing",
            ExpenseCategory::Health => "Health",
            ExpenseCategory::Others => "Others",
        };
        write!(f, "{}", category_str)
    }
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Expense{
    pub id: Uuid,
    pub user_id: Uuid,
    pub price: f64,
    pub description: String,
    pub category: ExpenseCategory,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct NewExpense {
    pub price: f64,
    pub description: String,
    pub category: ExpenseCategory,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpense {
    pub id: Uuid,
    pub price: f64,
    pub description: String,
    pub category: ExpenseCategory,
}



