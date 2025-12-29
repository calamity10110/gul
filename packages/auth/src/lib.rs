use serde::{Deserialize, Serialize};

pub mod jwt {
    use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: String,
        pub exp: usize,
    }

    pub fn create_token(sub: &str, secret: &[u8]) -> String {
        let claims = Claims {
            sub: sub.to_owned(),
            exp: 10000000000,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap()
    }
}

pub mod rbac {
    pub struct Role {
        pub name: String,
        pub permissions: Vec<String>,
    }
}

pub mod oauth2 {
    pub struct OAuthClient;
}
