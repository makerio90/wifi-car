use super::DriveQuery;
use drivers::drivers::Drivers;
use drivers::Driver;
use futures_util::{FutureExt, StreamExt};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub async fn drive(ws: WebSocket, driver: Arc<Mutex<Drivers>>) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            log::error!("error sending websocket msg: {}", e);
        }
    }));
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("error receiving message: {}", e);
                break;
            }
        };
        let msg = match msg.to_str() {
            Ok(v) => v,
            Err(_) => return,
        };
        let cmd: DriveQuery = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(e) => {
                log::error!("error parseing message: {}", e);
                break;
            }
        };
        log::debug!(target: "api", "waiting for driver lock");
        let mut driver = driver.lock().unwrap();
        log::debug!(target: "api", "got driver lock");
        client_sender.send(Ok(Message::text(format!(
            "{:?}",
            (*driver).drive(cmd.accelerate, cmd.steer)
        ))));
    }
}
