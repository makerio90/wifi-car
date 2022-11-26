use drivers::peripheral::{Example, Peripheral};

fn main() {
    let per = Example::init();
    println!("status: {:?}", per.status());
    println!("indata: {:?}", Example::InData);
}
