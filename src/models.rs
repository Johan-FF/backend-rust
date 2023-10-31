
#[derive(Queryable)]
pub struct PostModel {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
}