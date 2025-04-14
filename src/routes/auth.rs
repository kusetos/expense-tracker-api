use actix_web::{self, web};

use crate::handlers::auth::{login_handler, register_handler};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .route("/login", web::post().to(login_handler))
        .route("/register", web::post().to(register_handler));
        //.route("/info", web::get().to(check_token_handler));
}