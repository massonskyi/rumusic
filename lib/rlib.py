import ctypes
import os

_lib = ctypes.CDLL(os.path.abspath("./dist/rlib.so"))


class PasswordOptions(ctypes.Structure):
    _fields_ = [
        ("length", ctypes.c_size_t),
        ("include_uppercase", ctypes.c_bool),
        ("include_numbers", ctypes.c_bool),
        ("include_special_chars", ctypes.c_bool),
    ]


class HashingAlgorithm(ctypes.Structure):
    _fields_ = [("tag", ctypes.c_int)]


class RustString(ctypes.Structure):
    _fields_ = [("data", ctypes.POINTER(ctypes.c_char)), ("len", ctypes.c_size_t)]


# Функция для генерации пароля
def generate_password(options):
    _lib.generate_password_c.restype = RustString
    _lib.generate_password_c.argtypes = [PasswordOptions]

    result = _lib.generate_password_c(options)
    password = ctypes.string_at(result.data, result.len).decode("utf-8")
    _lib.free_rust_string(result)
    return password


# Функция для хеширования пароля
def hash_password(method, password):
    _lib.hash_password.restype = RustString
    _lib.hash_password.argtypes = [HashingAlgorithm, ctypes.c_char_p]

    method_tag = HashingAlgorithm(method)
    result = _lib.hash_password(method_tag, password.encode("utf-8"))
    hashed_password = ctypes.string_at(result.data, result.len).decode("utf-8")
    _lib.free_rust_string(result)
    return hashed_password


# Функция для верификации пароля
def verify_password(method, password, hash):
    _lib.verify_password.restype = ctypes.c_bool
    _lib.verify_password.argtypes = [HashingAlgorithm, ctypes.c_char_p, ctypes.c_char_p]

    method_tag = HashingAlgorithm(method)
    return _lib.verify_password(method_tag, password.encode("utf-8"), hash.encode("utf-8"))


# Функция для валидации пароля
def validate_password(password, min_length, require_uppercase, require_numbers, require_special_chars):
    return _lib.validate_password(password.encode("utf-8"), min_length, require_uppercase, require_numbers,
                                  require_special_chars)


if __name__ == "__main__":
    options = PasswordOptions(length=12, include_uppercase=True, include_numbers=True, include_special_chars=True)
    generated_password = generate_password(options)
    print("Generated password:", generated_password)

    password = "password123"
    hashed_password = hash_password(HashingAlgorithm.Bcrypt, password)
    print("Hashed password:", hashed_password)

    is_valid = verify_password(HashingAlgorithm.Bcrypt, password, hashed_password)
    print("Password is valid:", is_valid)

    is_validated = validate_password(password, 8, True, True, True)
    print("Password validated:", is_validated)