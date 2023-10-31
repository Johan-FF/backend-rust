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

    let mut conn = PgConnection::establish(&db_url).expect("db connection failed");

    use self::models::{Post, NewPost};
    use self::schema::posts;
    use self::schema::posts::dsl::*;

    let new_post = NewPost{
        title: "Mi primer blog",
        body: "Lorem ipsum",
        slug: "primer-post",
    };
    let _post: Post = diesel::insert_into(posts::table)
        .values(new_post)
        .get_result(&mut conn)
        .expect("insert failed");

    let mut post_result = posts.load::<Post>(&mut conn).expect("query error");

    for post in post_result {
        println!("{}", post.title);
    }

    post_result = posts.limit(1).load::<Post>(&mut conn).expect("query error");

    for post in post_result {
        println!("{}", post.title);
    }
}
