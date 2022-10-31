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
    let fmt = dev
        .set_format(&v4l::Format {
            width,
            height,
            ..fmt
        })
        .unwrap();
    Ok(warp::reply::json(&(fmt.width, fmt.height)))
}
