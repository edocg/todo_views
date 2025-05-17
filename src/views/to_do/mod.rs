mod create;
mod get;
mod edit;
mod delete;
use actix_web::web::{ServiceConfig, get, post, scope};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
 app.service(
    scope("/item")
    .route("create/{title}", post().to(create::create))
    .route("get", get().to(get::get))
    .route("edit", post().to(edit::edit)) // define view and URL
    .route("delete", post().to(delete::delete)) // define view and URL
 );
}