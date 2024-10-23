use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the route for GET requests to "/hi"
    let hello = warp::path("hi")
        .map(|| warp::reply::html("Hello"));

    // Start the server on localhost:3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
