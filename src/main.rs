use pretty_env_logger;
use warp::Filter;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let hi = warp::path("hi")
        .and(warp::path::param())
        .map(|name: String| {
            info!("Hi handler");
            format!("Hi, {}!", name)
        });

    let routes = warp::get().and(hi);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
