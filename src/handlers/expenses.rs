use std::vec;

use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{middleware::auth::AuthUser, models::expense::{Expense, NewExpense, UpdateExpense}};


pub async fn get_expenses(user: AuthUser, pool: Data<PgPool>) -> impl Responder {

   let expense_ids = match get_expenses_id_query(user.id, &pool).await{
      Ok(vec) => {
         //let vec_str: Vec<String> = vec.iter().map(Uuid::to_string).collect();
         vec
      },
      Err(e) => 
         return HttpResponse::InternalServerError().body(format!("Error getting expense ids in query: {}", e)),
   };
   let mut expenses: Vec<Expense> = vec![];
   for v in expense_ids{
      match get_expenses_query(v, &pool).await{
         Ok(exp) => expenses.push(exp),
         Err(e) => 
            return HttpResponse::InternalServerError().body(format!("Error getting expense in query: {}", e)),
      }
   }
   HttpResponse::Ok().json(expenses)
}
//let json = serde_json::to_string(&expense_ids).unwrap();
pub async fn get_expenses_query(expense_id: Uuid, pool: &PgPool) -> Result<Expense, sqlx::Error>{
   let query = 
      "SELECT 
         id,
         user_id,
         price::FLOAT8 as price,
         description,
         category,
         created_at
      FROM expenses
      WHERE id = $1";
   let res = sqlx::query_as::<_,Expense>(query)
      .bind(expense_id)
      .fetch_one(pool)
      .await?;
   Ok(res)
}
pub async fn get_expenses_id_query(user_id: Uuid, pool: &PgPool) -> Result<Vec<Uuid>, sqlx::Error>{
   
   let query = 
      "SELECT id 
      FROM expenses 
      WHERE user_id = $1";

   let res: Vec<Uuid>= sqlx::query_scalar(query)
      .bind(user_id)
      .fetch_all(pool)
      .await?;
   Ok(res)
}
pub async fn add_expense(user: AuthUser, new_expense: Json<NewExpense>, pool: Data<PgPool>) -> impl Responder{
   
   let expense = Expense{
      id: Uuid::new_v4(),
      user_id: user.id,
      price: new_expense.price,
      description: new_expense.description.clone(),
      category: new_expense.category.clone(),
      created_at: chrono::Local::now().naive_local()
   };
   match create_expence_query(&expense, &pool).await{
      Ok(_) => HttpResponse::Ok().body("Ok"),
      Err(e) => HttpResponse::InternalServerError().body(format!("Error creating expense in query: {}", e)),
   }
}

pub async fn create_expence_query(expense: &Expense, pool: &PgPool) -> Result<(), sqlx::Error>{

   let query = 
   "INSERT INTO
   expenses
   (id, user_id, price, description, category, created_at)
   VALUES ($1, $2, $3, $4, $5, $6)";

   sqlx::query(query)
      .bind(expense.id)
      .bind(expense.user_id)
      .bind(expense.price)
      .bind(&expense.description)
      .bind(&expense.category)
      .bind(expense.created_at)
      .execute(pool)
      .await?;
   Ok(())
}

pub async fn update_expense(update: Json<UpdateExpense>, pool: Data<PgPool>) -> impl Responder{

   match update_expense_query(&update, &pool).await{
    Ok(_) => HttpResponse::Ok().body("query updated"),
    Err(e) => HttpResponse::InternalServerError().body(format!("Error update query: {}", e)),
   }
}

pub async fn update_expense_query(
    update_expense: &UpdateExpense,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let query = r#"
        UPDATE expenses
        SET 
            price = $1,
            description = $2,
            category = $3
        WHERE id = $4
    "#;
    
    sqlx::query(query)
        .bind(update_expense.price)
        .bind(&update_expense.description)
        .bind(&update_expense.category)
        .bind(update_expense.id)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn delete_expense(payload: Json<Uuid>, pool: Data<PgPool>)-> impl Responder{
   match delete_expense_query(&payload, &pool).await{
    Ok(()) => HttpResponse::Ok().body("Deleted well"),
    Err(e) => HttpResponse::InternalServerError().body(format!("Error to delete expense: {}", e)),
   }
}
pub async fn delete_expense_query(
   id: &Uuid,
   pool: &PgPool
)-> Result<(), sqlx::Error>{
   
   let query = r#"
      DELETE
      FROM
         expenses
      WHERE
          id = $1
      "#;
   sqlx::query(query)
      .bind(id)
      .execute(pool)
      .await?;
   Ok(())
}