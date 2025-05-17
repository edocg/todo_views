use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};
mod json_serialization;
mod processes;
mod state;
mod to_do;
mod views;
mod jwt;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors=Cors::permissive();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{}-{}", req.method(), 
                          req.uri());
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
