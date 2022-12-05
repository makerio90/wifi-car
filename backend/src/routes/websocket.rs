use super::DriveQuery;
use drivers::driver::Driver;
use drivers::drivers::Drivers;
use futures_util::{FutureExt, StreamExt};
use log::error;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub async fn drive(ws: WebSocket, driver: Arc<Mutex<Drivers>>) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_send, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!(target: "ws_drive:20", "error sending websocket msg: {}", e);
        }
    }));
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!(target: "ws_drive:27" , "error: {:?}", e );
                break;
            }
        };
        match handle_msg(msg, driver.clone()) {
            Ok(_) => continue,
            Err(e) => error!(target: "ws_drive:33", "error: {:?}", e),
        }
        client_send.send(Ok(Message::text("ACK"))).unwrap();
    }
}

fn handle_msg(msg: Message, driver: Arc<Mutex<Drivers>>) -> Result<(), Box<dyn std::error::Error>> {
    let msg: &str = match msg.to_str() {
        Ok(s) => Ok(s),
        Err(()) => Err("error converting to string"),
    }?;
    log::debug!("got msg: {}", msg);
    let cmd: DriveQuery = serde_json::from_str(&msg)?;

    log::debug!(target: "api", "waiting for driver lock");
    let mut driver = driver.lock().unwrap();
    log::debug!(target: "api", "got driver lock");

    (*driver).drive(cmd.accelerate, cmd.steer)?;
    Ok(())
}
