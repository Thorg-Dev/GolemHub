#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectResponse {
    pub id: i32,
    pub name: String,
    pub homepage: String,
    pub developer: String,
    pub images: Vec<String>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image {
    pub icon: Vec<u8>,
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectCreationRequest {
    pub short_name: String,
    pub homepage: String,
    pub developer: String,
    pub images: Vec<String>,
}

fn default_limit() -> i32 {
    10
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectGetQuery {
    #[serde(default)]
    pub offset: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProjectModificationRequest {
    pub id: i32,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub homepage: Option<String>,
    pub developer: Option<String>,
    pub add_image: Option<Vec<String>>,
    pub del_image: Option<Vec<String>>,
}
