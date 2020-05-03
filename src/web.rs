use actix::{Actor, Addr, StreamHandler};
use actix_web::{App, Error, HttpServer, HttpRequest, HttpResponse, middleware, web};
use actix_web_actors::ws;

use crate::ecu;

struct ControlWebSocket {
    ecu_addr: Addr<ecu::JetRacerECU>,
}

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
            Ok(ws::Message::Text(text)) => {
                self.ecu_addr.do_send(ecu::Command(text.clone()));
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

}

async fn control(ecu_addr: web::Data<Addr<ecu::JetRacerECU>>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ControlWebSocket {ecu_addr: ecu_addr.get_ref().clone()}, &req, stream)
}

async fn index(_: web::Data<Addr<ecu::JetRacerECU>>, _: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(include_str!("index.html"))
}

pub fn start(ecu_addr: Addr<ecu::JetRacerECU>) {
    let ecu_addr_data = web::Data::new(ecu_addr);
    HttpServer::new(move || App::new()
        .wrap(middleware::Logger::default())
        .app_data(ecu_addr_data.clone())
        .service(web::resource("/").to(index))
        .service(web::resource("/ws/").to(control))
    ).workers(1).bind("127.0.0.1:8888").unwrap().run();
}