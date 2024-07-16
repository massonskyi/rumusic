
pub mod pwd{
    use rand::{thread_rng, Rng};
    use rand::distributions::{Alphanumeric, Distribution, Standard};
    use argon2::{self, Argon2};
    use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
    use argon2::password_hash::rand_core::OsRng;
    use bcrypt::{DEFAULT_COST, hash, verify};

    #[derive(Debug)]
    pub enum HashingAlgorithm {
        Argon2,
        Bcrypt,
    }

    #[derive(Debug)]
    pub struct PasswordOptions {
        pub length: usize,
        pub include_uppercase: bool,
        pub include_numbers: bool,
        pub include_special_chars: bool,
    }

    pub trait PasswordGenerator {
        fn generate_password(&self, options: PasswordOptions) -> String;
    }

    pub trait PwdHasher {
        fn hash_password(&self, password: &str) -> Result<String, String>;
        fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String>;
    }

    pub struct SimplePasswordGenerator;

    impl PasswordGenerator for SimplePasswordGenerator {
        fn generate_password(&self, options: PasswordOptions) -> String {
            let mut rng = thread_rng();
            let mut password = String::new();
            let mut chars: Vec<char> = Alphanumeric
                .sample_iter(&mut rng)
                .map(char::from)
                .collect();

            if options.include_uppercase {
                chars.extend('A'..='Z');
            }
            if options.include_numbers {
                chars.extend('0'..='9');
            }
            if options.include_special_chars {
                chars.extend("!@#$%^&*()_+-=[]{}|;:',.<>?/~`".chars());
            }

            for _ in 0..options.length {
                let idx = rng.gen_range(0..chars.len());
                password.push(chars[idx]);
            }

            password
        }
    }

    // Структура для реализации хеширования пароля
    pub struct PasswordHasherImpl {
        pub method: HashingAlgorithm,
    }

    impl PwdHasher for PasswordHasherImpl {
        fn hash_password(&self, password: &str) -> Result<String, String> {
            match self.method {
                HashingAlgorithm::Argon2 => {
                    let salt = SaltString::generate(&mut OsRng);
                    let argon2 = Argon2::default();
                    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
                        .map_err(|e| e.to_string())?;
                    Ok(password_hash.to_string())
                },
                HashingAlgorithm::Bcrypt => {
                    hash(password, DEFAULT_COST).map_err(|e| e.to_string())
                }
            }
        }

        fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
            match self.method {
                HashingAlgorithm::Argon2 => {
                    let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
                    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
                },
                HashingAlgorithm::Bcrypt => {
                    verify(password, hash).map_err(|e| e.to_string())
                }
            }
        }
    }

    // Валидация пароля
    pub fn validate_password(password: &str, min_length: usize, require_uppercase: bool, require_numbers: bool, require_special_chars: bool) -> bool {
        if password.len() < min_length {
            return false;
        }
        if require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return false;
        }
        if require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return false;
        }
        if require_special_chars && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:',.<>?/~`".contains(c)) {
            return false;
        }
        true
    }
}