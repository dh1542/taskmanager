mod rest;
mod system;
use byte_unit::{Byte, Unit, UnitType};
use sysinfo::{Components, Disks, Networks, System};
use warp::Filter;

#[tokio::main]
async fn main() {
    //let mut s = System::new_all();
    //s.refresh_all();
    //get_system_meta_info(s)
    //println!("{} bytes", _s.total_memory());

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let routes = rest::routes::routes();
    // println!("Server started at http://localhost:8000");
    // warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    system::system_meta::get_system_meta_info();
}
