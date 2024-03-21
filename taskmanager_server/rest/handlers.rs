use crate::system;

// A function to handle GET requests at /posts/{id}
pub async fn get_system_meta_info() -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let sys_meta: Vec<system::system_meta::MetaInfo> = system::system_meta::get_system_meta_info();
    Ok(warp::reply::json(&sys_meta))
}
