use crate::rest::handlers;
use warp::Filter;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_system_meta_info()
}

// A route to handle GET requests for a specific post
fn get_system_meta_info() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("systemMeta")
        .and(warp::get())
        .and_then(handlers::get_system_meta_info)
}
