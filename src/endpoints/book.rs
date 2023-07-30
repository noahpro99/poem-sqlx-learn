use crate::models;
use poem::error::InternalServerError;
use poem::Result;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, Tags,
};

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
struct Book {
    #[oai(read_only)]
    pub id: i64,
    #[oai(validator(max_length = 64))]
    pub title: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateBook {
    pub title: String,
}

#[derive(ApiResponse)]
enum UpdateBookResponse {
    #[oai(status = 200)]
    Ok(Json<Book>),
}

#[derive(ApiResponse)]
enum GetBookResponse {
    #[oai(status = 200)]
    Ok(Json<Book>),
    #[oai(status = 404)]
    NotFound(PlainText<String>),
}

#[derive(Tags)]
enum ApiTags {
    Book,
}

pub struct BookApi {
    pub pool: sqlx::PgPool,
}

#[OpenApi]
impl BookApi {
    #[oai(path = "/book", method = "post", tag = "ApiTags::Book")]
    async fn create_book(&self, update: Json<UpdateBook>) -> Result<UpdateBookResponse> {
        sqlx::query!(
            r"
            INSERT INTO book (title)
            VALUES ($1)
            RETURNING id, title
            ",
            update.title
        )
        .fetch_one(&self.pool)
        .await
        .map_err(InternalServerError)
        .map(|book| UpdateBookResponse::Ok(Json(Book {
            id: book.id.into(),
            title: book.title,
        })))
    }

    #[oai(path = "/book/:id", method = "get", tag = "ApiTags::Book")]
    async fn get_book(&self, id: Path<i64>) -> Result<GetBookResponse> {
        sqlx::query_as!(
            models::Book,
            r"
            SELECT id, title
            FROM book
            WHERE id = $1
            ",
            id.0 as i64
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(InternalServerError)
        .map(|book| match book {
            Some(book) => GetBookResponse::Ok(Json(Book {
                id: book.id.into(),
                title: book.title,
            })),
            None => GetBookResponse::NotFound(PlainText(format!("Book {} not found", id.0))),
        })
    }
}
