use serde::Serialize;
use tauri::State;
use sqlx::SqlitePool;

#[derive(Serialize)]
pub struct Movie {
    id: i64,
    title: String,
    year: Option<i32>,
    poster_path: Option<String>,
}

#[tauri::command]
pub async fn get_movies(db: State<'_, SqlitePool>) -> Result<Vec<Movie>, String> {
    let movies = sqlx::query_as!(
        Movie,
        r#"
        SELECT id, title, year, poster_path
        FROM movies
        ORDER BY title
        "#
    )
    .fetch_all(&*db)
    .await
    .map_err(|e| e.to_string())?;

    Ok(movies)
}