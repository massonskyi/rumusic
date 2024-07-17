from .config import load_config

cfg = load_config()


DB_HOST = cfg.getattr('DB_HOST')
DB_PORT = cfg.getattr('DB_PORT')
DB_USER = cfg.getattr('DB_USER')
DB_PASS = cfg.getattr('DB_PASS')
DB_NAME = cfg.getattr('DB_NAME')

BACKEND_SECRET_COOKIE_KEY = cfg.getattr('BACKEND_SECRET_COOKIE_KEY')