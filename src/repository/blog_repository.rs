use std::sync::Arc;

use chrono::Local;
use leptos::*;

#[cfg(feature = "ssr")]
use actix_web::web::Data;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};

#[cfg(feature = "ssr")]
use leptos_actix::extract;
use crate::model::blog_post::Post;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    let pool: Arc<Pool<Sqlite>> =
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    // sql update / insert query here later
    use uuid::Uuid;
    let id = id.unwrap_or(Uuid::new_v4().to_string());
    sqlx::query("INSERT INTO post VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET dt=excluded.dt,
            image_url=excluded.image_url,
            title=excluded.title,
            text=excluded.text")
        .bind(&id)
        .bind(&dt)
        .bind(&image_url)
        .bind(&title)
        .bind(&text)
        .execute(&*pool)
        .await?;
    Ok(id)
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    let pool: Arc<Pool<Sqlite>> = 
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;

    let res: Post = sqlx::query_as("SELECT * FROM post WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|_| ServerFnError::ServerError("error getting post".to_owned()))?;
    Ok(res)
}

#[server(GetPreviews, "/api")]
pub async fn get_previews(
    oldest: Option<String>,
    newest: Option<String>,
    preview_length: u8,
    page_size: u8,
) -> Result<Vec<Post>, ServerFnError> {
    let pool: Arc<Pool<Sqlite>> = 
        extract(|conn: Data<Pool<Sqlite>>| async move { conn.into_inner() }).await?;
    let res: Vec<Post> = sqlx::query_as(
        "SELECT 
            id, dt, image_url, title,
            CASE
                WHEN LENGTH(text) > $1 THEN SUBSTR(text, $1, ?) || '...'
                ELSE text
            END AS text
        FROM post
        ORDER BY dt DESC
        LIMIT $2"
    )
    .bind(preview_length)
    .bind(page_size)
    .fetch_all(&*pool)
    .await?;
    
    Ok(res)
}