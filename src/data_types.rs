#[derive(sqlx::FromRow, serde::Serialize)]
pub struct GolemImage {
    Id: i32,
    Name: String,
    Description: String,
    User: i32,
}
