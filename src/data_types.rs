#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub homepage: String,
    pub developer: String,
    pub images: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct ProjectCreationRequest {
    pub short_name: String,
    pub homepage: String,
    pub icon: String,
    pub developer: String,
    pub images: Vec<String>,
}

fn default_limit() -> u32 { 10 }

#[derive(serde::Deserialize)]
pub struct ProjectGetQuery {
    #[serde(default)]
    pub offset : u32,
    #[serde(default = "default_limit")]
    pub limit : u32,
}
