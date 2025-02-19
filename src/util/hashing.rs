use argon2::{password_hash, Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;

/// Hashes password with OsRng salt and default Argon2
pub fn hash_password(password: &String) -> password_hash::errors::Result<PasswordHash> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hashed = argon2.hash_password(password.as_bytes(), &salt);
    hashed
}