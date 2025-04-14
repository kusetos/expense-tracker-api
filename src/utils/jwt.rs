use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub jti: String,
}



#[derive(Serialize)]
pub struct TokenResponse{
    pub token: String
}
pub async fn create_jwt(claims: &Claims, key: &str) -> Result<String, jsonwebtoken::errors::Error>{

    let header = jsonwebtoken::Header::default();
    let key = EncodingKey::from_secret(key.as_bytes());
    
    jsonwebtoken::encode(&header, &claims, &key)
}


pub fn verify_jwt(jwt: &str, key: &str) -> Result<Claims, jsonwebtoken::errors::Error>{
    // let header = req.headers();
    // let token = match header.get("Authorization"){
    //     Some(header_value) => header_value.to_str().unwrap(),
    //     None => return HttpResponse::ExpectationFailed().body("Error to get value in Authorization Header"),
    // };
    
    let key = DecodingKey::from_secret(key.as_ref());
    let token_data = 
        jsonwebtoken::decode::<Claims>(
            jwt,
            &key,
            &Validation::new(jsonwebtoken::Algorithm::HS256)
        )?;
    
    println!("{}", token_data.claims.sub);
    Ok(token_data.claims)

}