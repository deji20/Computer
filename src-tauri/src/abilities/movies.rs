#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct MovieArgs {
    pub movie: String,
    pub year: Option<i32>,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MovieResponse {
    pub page: i32,
    pub total_results: i32,
    pub total_pages: i32,
    pub results: Vec<Movie>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Movie {
    pub poster_path: String,
    pub adult: bool,
    pub overview: String,
    pub release_date: String,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_title: String,
    pub original_language: String,
    pub title: String,
    pub backdrop_path: Option<String>,
    pub popularity: f32,
    pub vote_count: i32,
    pub video: bool,
    pub vote_average: f32,
}