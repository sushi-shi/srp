use actix_web::{
    get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use foundation::{browser_server, lobby_server};

#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct QueryParams {
    unused: Option<String>,
    r#type: Option<u8>,
    local_ip: Option<String>,
    login_ip: Option<String>,
}

#[get("/hello")]
async fn handle_request_lobby_server(
    _req: HttpRequest,
    query: web::Query<QueryParams>,
) -> impl Responder {
    log::error!("Received: {query:#?}");

    HttpResponse::Ok().body(format!("{}:{}", lobby_server::ADDRESS, lobby_server::PORT))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(handle_request_lobby_server)
    })
    .bind(format!(
        "{}:{}",
        browser_server::ADDRESS,
        browser_server::PORT
    ))?
    .run()
    .await
}
