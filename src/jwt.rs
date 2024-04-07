use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY")
            .unwrap()
            .as_str()
            .unwrap();
        return key_str.to_owned()
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        
        let token = encode(&Header::default(), &self, &key).unwrap();

        return token;
    }

    pub fn new(user_id: i32) -> Self {
        let config = Config::new();

        let minutes_to_expire = config.map.get("EXPIRE_TOKEN_IN_MINUTES")
            .unwrap()
            .as_i64()
            .unwrap();

        let expiration_time = Utc::now()
            .checked_add_signed(Duration::minutes(minutes_to_expire))
            .expect("valid timestamp for expiration datetime")
            .timestamp();


        return JwToken {
            user_id, exp: expiration_time as usize
        };
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());

        let token_result = decode::<JwToken>(&token, &key, &Validation::new(Algorithm::HS256));

        match token_result {
            Ok(data) => Ok(data.claims),
            Err(error) => {
                let message = format!("not decoding token {:?}", error);
                return Err(message);
            }
        }
    }
}


impl FromRequest for JwToken {
    type Error = Error;

    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();

                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Ok(token) => ok(token),
                    Err(error) => {
                        if error == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("token expired"))
                        }
                        return err(ErrorUnauthorized("token cannot be decoded"))
                    }

                }
            },
            None => {
                let error = ErrorUnauthorized("token not in the header under key 'token'");
                return err(error)
            }
        }
    }
}
