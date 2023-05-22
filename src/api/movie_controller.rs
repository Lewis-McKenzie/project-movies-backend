#[get("/movies")]
pub fn get_movies() -> &'static str {
    "All Movies"
}
