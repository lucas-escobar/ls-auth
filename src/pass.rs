use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).expect("Could not hash password")
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).expect("Invalid password")
}
