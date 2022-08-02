// import the Filter trait from warp
use warp::Filter;

#[tokio::main]
async fn main() {
    // create a path filter
    let hi = warp::path("hello").map(|| format!("Hello, World!"));

    // start the server and pass the route filter to it
    warp::serve(hi).run(([127,0,0,1], 3030)).await;
}
