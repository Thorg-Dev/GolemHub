#[derive(sqlx::FromRow)]
pub struct GolemImage {
    id: u32,
    name: String,
    description: String,
    downloads_number: u32,
}
