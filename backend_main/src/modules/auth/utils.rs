use std::io::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::{thread_rng, Rng};

pub struct PasswordManager;

impl PasswordManager{
    pub fn generate_password(length: u8, use_special_chars: bool, use_numbers: bool) -> Result<String, Error> {
        let mut rng = thread_rng();
        let mut password = String::with_capacity(length as usize);
        let mut charset = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_vec();

        if use_special_chars {
            charset.extend(b"!@#$%^&*()-_=+[]{}|;:'\",.<>?/`~");
        }

        if use_numbers {
            charset.extend(b"0123456789");
        }

        for _ in 0..length {
            let idx = rng.gen_range(0..charset.len());
            password.push(charset[idx] as char);
        }

        Ok(password)
    }
    pub fn hash_password(pwd: &str) -> Result<String, bcrypt::BcryptError>{
        hash(pwd, DEFAULT_COST)
    }
    pub fn verify_password(pwd: &str, hashed_pwd: &str) -> Result<bool, bcrypt::BcryptError>{
        verify(pwd, hashed_pwd)
    }
}

