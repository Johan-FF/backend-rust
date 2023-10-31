#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenvy::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("db_url nor found");

    let conn = PgConnection::establish(&db_url).expect("db connection failed");

    use self::models::PostModel;
    use self::schema::Post::dsl::*;

    let post_result = PostModel.load::<Post>(&conn).expect("query error");

    for post in post_result {
        println!("{}", post.title);
    }
}
