use actix::{Actor, Addr, StreamHandler};
use actix_web::{App, Error, HttpServer, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

use crate::ecu;

struct ControlWebSocket;

impl Actor for ControlWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ControlWebSocket {

    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

}

async fn control<D: std::marker::Unpin>(ecu_addr: web::Data<Addr<ecu::ECU<D>>>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ControlWebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}

async fn index<D: std::marker::Unpin>(ecu_addr: web::Data<Addr<ecu::ECU<D>>>, req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(include_str!("index.html"))
}

pub fn start<D: std::marker::Unpin>(ecu_addr: Addr<ecu::ECU<D>>) {
    let ecu_addr_data = web::Data::new(ecu_addr);
    HttpServer::new(|| App::new()
        .app_data(ecu_addr_data.clone())
        .service(web::resource("/").to(index::<D>))
        .service(web::resource("/ws/").to(control::<D>))
    ).workers(1).bind("127.0.0.1:8888").unwrap().run();
}