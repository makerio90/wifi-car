use super::*;
use serde::Serialize;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use v4l::{framesize::FrameSizeEnum, video::Capture, Device};

pub async fn get(dev: Arc<Mutex<Device>>) -> Result<impl warp::Reply, Infallible> {
    let dev = (*dev).lock().unwrap();
    let controlls: Vec<Description> = dev
        .query_controls()
        .unwrap()
        .iter()
        .map(|s| s.into())
        .collect();
    Ok(warp::reply::json(&controlls))
}

pub async fn set(
    Control { id, value }: Control,
    dev: Arc<Mutex<Device>>,
) -> Result<impl warp::Reply, Infallible> {
    let dev = (*dev).lock().unwrap();
    let fmt = dev.format().unwrap();
    dev.set_control(v4l::Control {
        id,
        value: value.into(),
    });
    Ok("")
}

#[derive(Serialize)]
struct Description {
    id: u32,
    typ: Type,
    name: String,
    minimum: i64,
    maximum: i64,
    step: u64,
    default: i64,
    //pub flags: Flags,
    items: Option<Vec<(u32, MenuItem)>>,
}

impl From<&v4l::control::Description> for Description {
    fn from(value: &v4l::control::Description) -> Self {
        Description {
            id: value.id,
            typ: value.typ.into(),
            name: value.name.clone(),
            minimum: value.minimum,
            maximum: value.maximum,
            step: value.step,
            default: value.default,
            items: value
                .items
                .as_ref()
                .map(|v| v.iter().map(|d| (d.0, (&d.1).into())).collect()),
        }
    }
}
#[derive(Serialize)]
enum MenuItem {
    Name(String),
    Value(i64),
}

impl From<&v4l::control::MenuItem> for MenuItem {
    fn from(value: &v4l::control::MenuItem) -> Self {
        match &value {
            v4l::control::MenuItem::Name(n) => Self::Name(n.to_string()),
            v4l::control::MenuItem::Value(v) => Self::Value(v.to_owned()),
        }
    }
}
#[derive(Serialize)]
enum Type {
    Integer,
    Boolean,
    Menu,
    Button,
    Integer64,
    CtrlClass,
    String,
    Bitmask,
    IntegerMenu,
    U8,
    U16,
    U32,
    Area,
}
use v4l::control::Type as RemoteType;
impl From<RemoteType> for Type {
    fn from(value: RemoteType) -> Self {
        match value {
            RemoteType::Integer => Type::Integer,
            RemoteType::Boolean => Type::Boolean,
            RemoteType::Menu => Type::Menu,
            RemoteType::Button => Type::Button,
            RemoteType::Integer64 => Type::Integer64,
            RemoteType::CtrlClass => Type::CtrlClass,
            RemoteType::String => Type::String,
            RemoteType::Bitmask => Type::Bitmask,
            RemoteType::IntegerMenu => Type::IntegerMenu,
            RemoteType::U8 => Type::U8,
            RemoteType::U16 => Type::U16,
            RemoteType::U32 => Type::U32,
            RemoteType::Area => Type::Area,
        }
    }
}
