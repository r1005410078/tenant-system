use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub struct Argon;

impl Argon {
    pub fn password_hash(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2: Argon2 = Argon2::default();
        let password = password.as_bytes();
        argon2.hash_password(password, &salt).unwrap().to_string()
    }

    pub fn verify_password(password: &str, password_hash: &str) -> bool {
        if let Ok(parsed_hash) = PasswordHash::new(password_hash) {
            Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok()
        } else {
            false
        }
    }
}
