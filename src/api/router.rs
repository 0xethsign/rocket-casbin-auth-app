use rocket;

use crate::api;
use crate::connection;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/posts",
            routes![
                api::handler::all_posts,
                api::handler::create_post,
                api::handler::get_post,
                api::handler::update_post,
                api::handler::delete_post
            ],
        )
        .launch();
}
