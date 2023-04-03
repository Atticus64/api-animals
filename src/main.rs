// api in framework tide
use tide::{prelude::json, Request};

#[async_std::main]
async fn main() -> Result<(), tide::Error> {
    tide::log::start();

    let mut app = tide::new();

    app.with(tide::log::LogMiddleware::new());

    app.at("/animals").get(show_animals);

    // endpoint to respond hello world
    app.at("/").get(|_| async { Ok("hello") });
    app.at("/goodbye")
        .get(|_| async { Ok(json!({ "goodbye": "bye" })) });

    app.at("/www").serve_file("./public/index.html")?;

    // endpoint to serve content file hello.txt in dir public
    app.at("/hello").serve_file("./public/hello.txt")?;

    app.listen("localhost:3000").await?;

    Ok(())
}

pub async fn show_animals(_req: Request<()>) -> tide::Result<tide::Body> {
    let animals = vec!["Scoby Doo", "Jerry", "Pato", "Bunny"];
    Ok(tide::Body::from_json(&animals)?)
}
