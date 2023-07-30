use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};

pub struct UsersApi;

#[derive(Debug, Object, Clone, Eq, PartialEq, Default)]
pub struct User {
    #[oai(read_only)]
    pub id: i64,
    #[oai(validator(max_length = 64))]
    pub name: String,
    pub age: Option<i32>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UpdateUser {
    pub name: String,
}

#[derive(ApiResponse)]
enum UpdateUserResponse {
    #[oai(status = 200)]
    Ok(Json<User>),
}

#[derive(Tags)]
enum ApiTags {
    // Operations about user
    User,
}

#[OpenApi]
impl UsersApi {
    #[oai(path = "/user", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, update: Json<UpdateUser>) -> UpdateUserResponse {
        UpdateUserResponse::Ok(Json(User {
            id: 1,
            name: update.name.clone(),
            ..Default::default()
        }))
    }
}
