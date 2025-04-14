use actix_web::web::{self};

use actix_web::{HttpResponse, Responder};
use chrono::{self, Duration, Local};
use sqlx::{PgPool, Row};
use crate::models::user::{LoginData, NewUserData, User};
use crate::utils::jwt::{self, Claims, TokenResponse};
use uuid::{self, Uuid};


const SECRET: &str = "boi";

pub async fn login_handler(login_data: web::Json<LoginData>, pool: web::Data<PgPool>) -> impl Responder{

    let id = match validate_user(&login_data, &pool).await{
        Ok(None) =>{
            return HttpResponse::Unauthorized().body(format!("Invalid password or email"))
        },
        Ok(Some(id)) => id,
        Err(e) => 
            return HttpResponse::InternalServerError().body(format!("Error to validate user: {}", e)),
    };

    let iat = Local::now().timestamp() as usize;
    let exp = (Local::now() + Duration::days(1)).timestamp() as usize;
    let jti = uuid::Uuid::new_v4().to_string();

    let claims = Claims{
        sub: id.to_string(),
        iat: iat,
        exp: exp,
        jti: jti,
    };

    match jwt::create_jwt(&claims, SECRET).await{
        Ok(token) => HttpResponse::Ok().json(TokenResponse{token}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error generate token: {}", e))
    }

    

}


pub async fn register_handler(new_user_data: web::Json<NewUserData>, pool: web::Data<PgPool>) -> impl Responder{

    let user = User{
        id: uuid::Uuid::new_v4(),
        email: new_user_data.email.clone(),
        username: new_user_data.username.clone(),
        password: new_user_data.password.clone(),
        created_at: chrono::Local::now()
    };

    match create_user(&user, pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to register user")
        }
    }
    
}
async fn validate_user(user: &LoginData, pool: &sqlx::PgPool) -> Result<Option<Uuid>, sqlx::Error>{
    let query = 
        "SELECT id
        FROM users 
        WHERE email = $1
        AND password = $2";
    
    let row = sqlx::query(query)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|r| r.get("id")))
}
async fn create_user(user: &User, pool: &sqlx::PgPool) -> Result<(), sqlx::Error>{
    let query = 
    "INSERT INTO
    users
    (id, email, username, password, created_at)
    VALUES ($1, $2, $3, $4, $5)";
    
    sqlx::query(query)
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.password)
        .bind(user.created_at)
        .execute(pool)
        .await?;
    Ok(())

}