mod stream;

use crate::settings::WebCamSettings;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use v4l::framesize::FrameSizeEnum;
use v4l::video::Capture;
use v4l::{Device, FourCC};
use warp::Filter;

pub fn webcam(
    settings: WebCamSettings,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // create a refrance counted device
    let dev = Arc::new(Mutex::new(Device::with_path(settings.path).unwrap()));
    {
        let dev = (*dev).lock().unwrap();
        // get the current format
        let fmt = dev.format().unwrap();
        // use the MJPEG format
        let fourcc = FourCC::new(b"MJPG");
        // use the LOWEST resolution supported by the camera.
        let (width, height): (u32, u32) = match &dev.enum_framesizes(fourcc).unwrap()[0].size {
            FrameSizeEnum::Discrete(d) => (d.width, d.height),
            _ => todo!(),
        };
        // update the values we want to change, leave the rest the same
        dev.set_format(&v4l::Format {
            width,
            height,
            fourcc,
            ..fmt
        });
    }
    stream_route(dev.clone())
        .or(get_resolution(dev.clone()))
        .or(set_resolution(dev.clone()))
}
fn stream_route(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "stream")
        .and(warp::get())
        .map(move || dev.clone())
        .and_then(stream::stream)
}

fn get_resolution(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "resolution")
        .and(warp::get())
        .map(move || dev.clone())
        .map(|dev: Arc<Mutex<Device>>| {
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
        })
}
#[derive(Deserialize)]
struct Res {
    width: u32,
    height: u32,
}
fn set_resolution(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "resolution")
        .and(warp::query::<Res>())
        .and(warp::post())
        .and(warp::any().map(move || dev.clone()))
        .map(|Res { width, height }: Res, dev: Arc<Mutex<Device>>| {
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
        })
}
