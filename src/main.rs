use actix_web::{web, App, HttpServer};
mod app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .data()
            .service(
                web::scope("/api")
                    .service(web::scope("/tags").service(app::tag::api::index))
                    .service(
                        web::scope("/users")
                            .service(app::auth::signin)
                            .service(app::auth::signup),
                    )
                    .service(
                        web::scope("/user")
                            .service(app::users::me)
                            .service(app::users::update),
                    )
                    .service(
                        web::scope("/profiles")
                            .service(app::profiles::show)
                            .service(app::profiles::follow)
                            .service(app::profiles::unfollow),
                    )
                    .service(
                        web::scope("/articles/{id}/comments")
                            .service(app::articles::comments::index)
                            .service(app::articles::comments::create)
                            .service(app::articles::comments::delete),
                    )
                    .service(
                        web::scope("/articles/{id}/favorites")
                            .service(app::articles::favorites::favorite)
                            .service(app::articles::favorites::unfavorite),
                    )
                    .service(
                        web::scope("/articles")
                            .service(app::articles::index)
                            .service(app::articles::feed)
                            .service(app::articles::show)
                            .service(app::articles::create)
                            .service(app::articles::update)
                            .service(app::articles::delete),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
