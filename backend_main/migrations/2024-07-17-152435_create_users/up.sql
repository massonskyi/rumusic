-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    age INTEGER NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    hash_password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP, -- Опциональное поле
    role VARCHAR(50) NOT NULL,
    avatar VARCHAR(255),
    status VARCHAR(50),
    token VARCHAR(255),

    -- Поля, представляющие собой списки
    followers INTEGER[], -- Список идентификаторов пользователей
    followings INTEGER[], -- Список идентификаторов пользователей

    bio TEXT,
    favorite_genres TEXT[], -- Список жанров
    last_active TIMESTAMP NOT NULL,
    recently_played TEXT[], -- Список идентификаторов или ссылок на песни
    liked_songs TEXT[], -- Список идентификаторов или ссылок на песни
    social_links TEXT[] -- Список ссылок на социальные сети
);