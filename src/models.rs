use sqlx;


#[derive(Debug, sqlx::FromRow, Default)]
pub struct Book {
    pub id: i32,
    pub title: String,
}