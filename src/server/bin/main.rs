

use actix::{Actor, Addr};
use actix_web::{
    get,
    web::{Data, Payload},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use lobby::Lobby;
use socket::WsConnection;

mod lobby;
mod messages;
mod socket;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("beleza le")
}

#[get("/chat")]
async fn chat_connection(
    request: HttpRequest,
    stream: Payload,
    main_lobby: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("{main_lobby:?}");

    let connection = WsConnection::new(main_lobby.get_ref().clone());

    let response = ws::start(connection, &request, stream)?;

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let main_lobby = Data::new(Lobby::default().start());

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(chat_connection)
            .app_data(Data::clone(&main_lobby))
        // .route("/{group_id}", web::get().to(start_connection))
        // .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
