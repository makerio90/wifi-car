mod controll;
mod resolution;
mod stream;

use crate::settings::WebCamSettings;
use hyper::StatusCode;
use serde::Deserialize;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use v4l::framesize::FrameSizeEnum;
use v4l::video::Capture;
use v4l::{Device, FourCC};
use warp::http::Response;
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
        .or(get_controlls(dev.clone()))
        .or(set_controll(dev))
}
fn stream_route(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "stream")
        .and(warp::get())
        .map(move || dev.clone())
        .and_then(stream::stream)
}
fn get_controlls(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "control")
        .and(warp::get())
        .map(move || dev.clone())
        .and_then(controll::get)
}
#[derive(Deserialize)]
pub struct Control {
    pub id: u32,
    pub value: Value,
}

#[derive(Deserialize)]
pub enum Value {
    None,
    Integer(i64),
    Boolean(bool),
    String(String),
    CompoundU8(Vec<u8>),
    CompoundU16(Vec<u16>),
    CompoundU32(Vec<u32>),
    CompoundPtr(Vec<u8>),
}

use v4l::control::Value as ForenValue;
impl From<Value> for ForenValue {
    fn from(value: Value) -> Self {
        match value {
            Value::None => ForenValue::None,
            Value::Integer(v) => ForenValue::Integer(v),
            Value::Boolean(v) => ForenValue::Boolean(v),
            Value::String(v) => ForenValue::String(v),
            Value::CompoundU8(v) => ForenValue::CompoundU8(v),
            Value::CompoundU16(v) => ForenValue::CompoundU16(v),
            Value::CompoundU32(v) => ForenValue::CompoundU32(v),
            Value::CompoundPtr(v) => ForenValue::CompoundPtr(v),
        }
    }
}

fn set_controll(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "control")
        .and(warp::query::<Control>())
        .and(warp::post())
        .and(warp::any().map(move || dev.clone()))
        .and_then(controll::set)
}
fn get_resolution(
    dev: Arc<Mutex<Device>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("webcam" / "resolution")
        .and(warp::get())
        .map(move || dev.clone())
        .and_then(resolution::get)
}
#[derive(Deserialize)]
pub struct Res {
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
        .and_then(resolution::set)
}
