use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::pwd;
use crate::pwd::pwd::{HashingAlgorithm, PasswordGenerator, PasswordHasherImpl, PasswordOptions, PwdHasher, SimplePasswordGenerator, validate_password};

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe { CString::from_raw(s) };
}

    // Функция для генерации пароля и возврата его в виде строки C
    #[no_mangle]
    pub extern "C" fn generate_password_c(length: usize, include_uppercase: bool, include_numbers: bool, include_special_chars: bool) -> *mut c_char {
        let generator = SimplePasswordGenerator;
        let options = PasswordOptions {
            length,
            include_uppercase,
            include_numbers,
            include_special_chars,
        };
        let password = generator.generate_password(options);
        let c_str = CString::new(password).unwrap();
        c_str.into_raw()
    }

    // Функция для хеширования пароля и возврата его в виде строки C
    #[no_mangle]
    pub extern "C" fn hash_password_c(password: *const c_char, method: HashingAlgorithm) -> *mut c_char {
        if password.is_null() { return std::ptr::null_mut(); }
        let password_str = unsafe { CStr::from_ptr(password).to_string_lossy().into_owned() };

        let hasher = PasswordHasherImpl { method };
        let hash_result = match hasher.hash_password(&password_str) {
            Ok(hash) => hash,
            Err(_) => return std::ptr::null_mut(),
        };

        let c_str = CString::new(hash_result).unwrap();
        c_str.into_raw()
    }

    // Функция для верификации пароля по хешу и возврата результата в виде булевого значения
    #[no_mangle]
    pub extern "C" fn verify_password_c(password: *const c_char, hash: *const c_char, method: HashingAlgorithm) -> bool {
        if password.is_null() || hash.is_null() { return false; }
        let password_str = unsafe { CStr::from_ptr(password).to_string_lossy().into_owned() };
        let hash_str = unsafe { CStr::from_ptr(hash).to_string_lossy().into_owned() };

        let hasher = PasswordHasherImpl { method };
        hasher.verify_password(&password_str, &hash_str).unwrap_or_else(|_| false)
    }

    // Функция для валидации пароля и возврата результата в виде булевого значения
    #[no_mangle]
    pub extern "C" fn validate_password_c(password: *const c_char, min_length: usize, require_uppercase: bool, require_numbers: bool, require_special_chars: bool) -> bool {
        if password.is_null() { return false; }
        let password_str = unsafe { CStr::from_ptr(password).to_string_lossy().into_owned() };

        validate_password(&password_str, min_length, require_uppercase, require_numbers, require_special_chars)
    }