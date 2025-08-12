// src/tests/unit_tests.rs
#[cfg(test)]
mod tests {
    use bcrypt::{hash, verify, DEFAULT_COST};
    use uuid::Uuid;
    use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
    use serde::{Serialize, Deserialize};

    #[test]
    fn test_password_hash_and_verify() {
        let password = "my_secret";
        let hashed = hash(password, DEFAULT_COST).unwrap();
        assert!(verify(password, &hashed).unwrap());
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    #[test]
    fn test_jwt_encode_decode() {
        let key = "secret";
        let claims = Claims { sub: "user123".into(), exp: 2000000000 };
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(key.as_bytes())).unwrap();
        let decoded = decode::<Claims>(&token, &DecodingKey::from_secret(key.as_bytes()), &Validation::default()).unwrap();
        assert_eq!(decoded.claims.sub, "user123");
    }

    #[test]
    fn test_uuid_validity() {
        let id = Uuid::new_v4();
        assert_eq!(id.to_string().len(), 36);
    }
}
