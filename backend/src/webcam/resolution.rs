use super::*;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use v4l::{framesize::FrameSizeEnum, video::Capture, Device};

pub async fn get(dev: Arc<Mutex<Device>>) -> Result<impl warp::Reply, Infallible> {
    let dev = (*dev).lock().unwrap();
    let fmt = dev.format().unwrap();
    let sizes: Vec<(u32, u32)> = dev
        .enum_framesizes(fmt.fourcc)
        .unwrap()
        .iter()
        .map(|frame_size| match &frame_size.size {
            FrameSizeEnum::Discrete(d) => (d.width, d.height),
            _ => todo!(),
        })
        .collect();
    Ok(warp::reply::json(&sizes))
}

pub async fn set(
    Res { width, height }: Res,
    dev: Arc<Mutex<Device>>,
) -> Result<impl warp::Reply, Infallible> {
    let dev = (*dev).lock().unwrap();
    let fmt = dev.format().unwrap();
    match dev.set_format(&v4l::Format {
        width,
        height,
        ..fmt
    }) {
        Ok(fmt) => {
            Ok(Response::builder().body(serde_json::to_string(&(fmt.width, fmt.height)).unwrap()))
        }
        Err(e) => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(e.to_string())),
    }
}
