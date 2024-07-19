use chrono:: prelude::*;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;
use super::{model::UserModel, utils::PasswordManager};
use tokio_postgres::{Client, Error, NoTls};
use tokio::{self, sync::Mutex};
use serde_json::Value; 
use utoipa::ToSchema;

#[derive(utoipa::ToSchema)]
pub struct UserManager {
    pool: Arc<Mutex<Client>>,
}

impl Clone for UserManager {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }

}
impl UserManager {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(database_url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(UserManager {
            pool: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn create_user(
        &self,
        name: String,
        surname: String,
        age: i32,
        username: String,
        email: String,
        password: String,
        role: String,
        avatar: String,
    ) -> Result<UserModel, String> {
        let hash_password = PasswordManager::hash_password(&password).unwrap();
        let now = Utc::now().to_string();
        let user = UserModel {
            id: 0,
            name: name.clone(),
            surname: surname.clone(),
            age,
            username: username.clone(),
            email: email.clone(),
            hash_password: hash_password.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            deleted_at: None,
            role: role.clone(),
            avatar: avatar.clone(),
            status: String::from("active"),
            token: Uuid::new_v4().to_string(),
            followers: Vec::new(),
            followings: Vec::new(),
            bio: String::new(),
            favorite_genres: Vec::new(),
            last_active: now.clone(),
            recently_played: Vec::new(),
            liked_songs: Vec::new(),
            social_links: Vec::new(),
        };

        let query = "
            INSERT INTO users (name, surname, age, username, email, hash_password, created_at, updated_at, role, avatar, status, token, bio, favorite_genres, last_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id
        ";

        let client = self.pool.lock().await;
        let stmt = client.prepare(&query).await.unwrap();
        
        let row = client
        .query_one(
            &stmt,
            &[
                &name, &surname, &age, &username, &email, &hash_password, &now, &now, &role,
                &avatar, &user.status, &user.token, &user.bio, &user.favorite_genres.join(","),
                &now,
            ],
        )
        .await.map_err(|e| format!("Failed to execute query: {}", e))?;

        let id: i32 = row.get(0);
        Ok(UserModel { id, ..user })
    }
    pub async fn update_user(
        &self,
        id: i32,
        update_data: HashMap<String, String>,
    ) -> Result<UserModel, String> {
        let mut user = self.get_user(id).await.ok_or("User not found")?;

        for (key, value) in update_data.iter() {
            match key.as_str() {
                "name" => user.name = value.clone(),
                "surname" => user.surname = value.clone(),
                "age" => user.age = value.parse().unwrap_or(user.age),
                "username" => user.username = value.clone(),
                "email" => user.email = value.clone(),
                "hash_password" => {
                    user.hash_password = PasswordManager::hash_password(value).unwrap()
                }
                "role" => user.role = value.clone(),
                "avatar" => user.avatar = value.clone(),
                "status" => user.status = value.clone(),
                "bio" => user.bio = value.clone(),
                "favorite_genres" => {
                    user.favorite_genres = value.split(',').map(String::from).collect()
                }
                "recently_played" => {
                    user.recently_played = value.split(',').map(String::from).collect()
                }
                "liked_songs" => {
                    user.liked_songs = value.split(',').map(String::from).collect()
                }
                "social_links" => {
                    user.social_links = value.split(',').map(String::from).collect()
                }
                _ => {}
            }
        }

        user.updated_at = Utc::now().to_string();
        let query = "
            UPDATE users
            SET name = $1, surname = $2, age = $3, username = $4, email = $5, hash_password = $6, updated_at = $7, role = $8, avatar = $9, status = $10, bio = $11, favorite_genres = $12, last_active = $13
            WHERE id = $14
        ";

        let client = self.pool.lock().await;
        let stmt = client.prepare(&query).await.unwrap();

        client
            .execute(
                &stmt,
                &[
                    &user.name, &user.surname, &user.age, &user.username, &user.email,
                    &user.hash_password, &user.updated_at, &user.role, &user.avatar,
                    &user.status, &user.bio, &user.favorite_genres.join(","), &user.last_active,
                    &id,
                ],
            )
            .await
            .unwrap();

        Ok(user)
    }

    pub async fn delete_user(&self, id: i32) -> Result<(), String> {
        let _user = self.get_user(id).await.ok_or("User not found")?;

        let query = "UPDATE users SET deleted_at = $1 WHERE id = $2";
        let client = self.pool.lock().await;
        let stmt = client.prepare(query).await.unwrap();

        client
            .execute(&stmt, &[&Utc::now().to_string(), &id])
            .await
            .unwrap();

        Ok(())
    }


    pub async fn get_user(&self, id: i32) -> Option<UserModel> {
        let query = "
            SELECT id, name, surname, age, username, email, hash_password, created_at, updated_at, deleted_at, role, avatar, status, token, bio, favorite_genres, last_active
            FROM users
            WHERE id = $1
        ";

        let client = self.pool.lock().await;
        let stmt = client.prepare(query).await.unwrap();

        if let Ok(row) = client.query_one(&stmt, &[&id]).await {
            Some(UserModel {
                id: row.get(0),
                name: row.get(1),
                surname: row.get(2),
                age: row.get(3),
                username: row.get(4),
                email: row.get(5),
                hash_password: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                deleted_at: row.get(9),
                role: row.get(10),
                avatar: row.get(11),
                status: row.get(12),
                token: row.get(13),
                bio: row.get(14),
                favorite_genres: row.get::<_, String>(15).split(',').map(String::from).collect(),
                last_active: row.get(16),
                followers: Vec::new(),     // Placeholder
                followings: Vec::new(),    // Placeholder
                recently_played: Vec::new(),
                liked_songs: Vec::new(),
                social_links: Vec::new(),
            })
        } else {
            None
        }
    }
    pub async fn list_users(&self) -> Vec<UserModel> {
        let query = "
            SELECT id, name, surname, age, username, email, hash_password, created_at, updated_at, deleted_at, role, avatar, status, token, bio, favorite_genres, last_active
            FROM users
        ";

        let client = self.pool.lock().await;
        let stmt = client.prepare(query).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();

        rows.iter()
            .map(|row| UserModel {
                id: row.get(0),
                name: row.get(1),
                surname: row.get(2),
                age: row.get(3),
                username: row.get(4),
                email: row.get(5),
                hash_password: row.get(6),
                created_at: row.get(7),
                updated_at: row.get(8),
                deleted_at: row.get(9),
                role: row.get(10),
                avatar: row.get(11),
                status: row.get(12),
                token: row.get(13),
                bio: row.get(14),
                favorite_genres: row.get::<_, String>(15).split(',').map(String::from).collect(),
                last_active: row.get(16),
                followers: Vec::new(),     // Placeholder
                followings: Vec::new(),    // Placeholder
                recently_played: Vec::new(),
                liked_songs: Vec::new(),
                social_links: Vec::new(),
            }).collect()
        }
        pub async fn get_user_by_username(&self, username: &str) -> Result<Option<UserModel>, Error> {
            let query = "
                SELECT id, name, surname, age, username, email, hash_password, created_at, updated_at, deleted_at, role, avatar, status, token, bio, favorite_genres, last_active
                FROM users
                WHERE username = $1
            ";
            let client = self.pool.lock().await;
            let stmt = client.prepare(query).await?;
            let row = client.query_opt(&stmt, &[&username]).await?;
            
            match row {
                Some(row) => {
                    let id: i32 = row.try_get(0)?;
                    let name: String = row.try_get(1)?;
                    let surname: String = row.try_get(2)?;
                    let age: i32 = row.try_get(3)?;
                    let username: String = row.try_get(4)?;
                    let email: String = row.try_get(5)?;
                    let hash_password: String = row.try_get(6)?;
                    let created_at: String = row.try_get(7)?;
                    let updated_at: String = row.try_get(8)?;
                    let deleted_at: Option<String> = row.try_get(9)?;
                    let role: String = row.try_get(10)?;
                    let avatar: String = row.try_get(11)?;
                    let status: String = row.try_get(12)?;
                    let token: String = row.try_get(13)?;
                    let bio: String = row.try_get(14)?;
                    let favorite_genres_raw: String = row.try_get(15)?;
                    let favorite_genres: Vec<String> = favorite_genres_raw.split(',').map(String::from).collect();
                    let last_active: String = row.try_get(16)?;
                    
                    // Deserialize JSON fields
                    let followers: Vec<UserModel> = Vec::new(); // Placeholder
                    let followings: Vec<UserModel> = Vec::new(); // Placeholder
                    let recently_played: Vec<String> = Vec::new();
                    let liked_songs: Vec<String> = Vec::new();
                    let social_links: Vec<String> = Vec::new();
    
                    // Convert JSON fields
                    let user = UserModel {
                        id,
                        name,
                        surname,
                        age,
                        username,
                        email,
                        hash_password,
                        created_at,
                        updated_at,
                        deleted_at,
                        role,
                        avatar,
                        status,
                        token,
                        bio,
                        favorite_genres,
                        last_active,
                        followers,
                        followings,
                        recently_played,
                        liked_songs,
                        social_links,
                    };
    
                    Ok(Some(user))
                },
                None => Ok(None),
            }
        }
}