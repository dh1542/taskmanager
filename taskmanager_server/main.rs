mod rest;
mod system;

use byte_unit::{Byte, Unit, UnitType};
use sysinfo::{Components, Disks, Networks, System};
use system::system_meta::MetaInfo;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let routes = rest::routes::routes().with(warp::cors().allow_any_origin());
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    let sys_meta: Vec<system::system_meta::MetaInfo> = system::system_meta::get_system_meta_info();

    println!("{:?}", sys_meta)
}
