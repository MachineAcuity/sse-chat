use log::*;
use warp::Filter;

fn main() {
    let api_one = warp::path!("one").and(warp::get()).and_then(handler_one);

    let api_two = warp::path!("two").and(warp::get()).and_then(handler_two);

    let api_routes = warp::path("api").and(api_one.or(api_two));

    let routes = api_routes.or(warp::any().map(|| "Hello, World!"));

    info!("running webserver");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime should start")
        .block_on(async {
            warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
        });
}

async fn handler_one() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("You said one: {}", 1))
}

async fn handler_two() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("You said two: {}", 2))
}
