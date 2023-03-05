//add the modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, delete_user, get_all_user, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount(
        "/",
        routes![
            create_user,
            get_user,
            update_user,
            delete_user,
            get_all_user
        ],
    )
}
