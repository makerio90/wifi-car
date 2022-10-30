use futures::channel::mpsc;
use hyper::Body;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::thread;
use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::MmapStream;
use v4l::Device;
use warp::http::Response;

pub async fn stream(dev: Arc<Mutex<Device>>) -> Result<impl warp::Reply, Infallible> {
    // start a mpsc chanel from the futures crate.
    let (mut tx, rx) = mpsc::channel(10);
    // spawn a thread
    thread::spawn(move || {
        // start streaming the video
        let mut stream =
            MmapStream::with_buffers(&(*dev).lock().unwrap(), Type::VideoCapture, 4).expect("1");
        loop {
            let (buf, meta) = stream.next().expect("2");
            let mut data: Vec<u8> = format!(
                "--boundarydonotcross\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\nX-Timestamp: {}\r\n\r\n",
                buf.len() - 2,
                meta.timestamp
            )
            .into_bytes();
            let mut buf = buf.to_vec();
            data.append(&mut buf);
            data.append(&mut vec![0x0D, 0x0A]);
            let out: Result<Vec<u8>, Infallible> = Ok(data);
            // send it. OK() means it worked, Err() means that the client has disconnected
            match tx.try_send(out) {
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    });
    // build the response
    Ok(Response::builder()
        // dont cache this
        .header("Cache-Control", "no-cache, private")
        // again
        .header("Pragma", "no-cache")
        // MIME type. `boundry` is the data sent inbetween each frame
        .header(
            "content-type",
            "multipart/x-mixed-replace; boundary=boundarydonotcross",
        )
        // wrap are mpsc chanel as a stream so data is sent in real time
        .body(Body::wrap_stream(rx)))
}
