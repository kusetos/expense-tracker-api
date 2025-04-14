use actix_web::web;

use crate::handlers::expenses::{add_expense, delete_expense, get_expenses, update_expense};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .route("/add_expense", web::post().to(add_expense))
        .route("/get_expenses", web::get().to(get_expenses))
        .route("/update_expense", web::put().to(update_expense))
        .route("/delete_expense", web::delete().to(delete_expense));
}