mod nautobot;

use crate::nautobot::WebhookRequest;
// use serde_json::Result;
use reqwest::Client;
use warp::{Filter, Reply};

pub async fn post_form(webhook: WebhookRequest) -> Result<impl warp::Reply, warp::Rejection> {
    println!(
        "postform address : {:?} name: {}",
        webhook.data.address, webhook.data.assigned_object.name
    );
    let token = String::from("672ace375be2dcccb85fa6add30138");
    let form = reqwest::multipart::Form::new()
        .text("token", token)
        .text("ref", "nautobot")
        .text("variables[ADDRESS]", webhook.data.address)
        .text("variables[NAME]", webhook.data.assigned_object.name);
    let url =
        String::from("http://dmz-gitlab.sjc.aristanetworks.com/api/v4/projects/5/trigger/pipeline");
    // let url = String::from(
    //     "http://dmz-gitlab.sjc.aristanetworks.com/api/v4/projects/5/ref/nautobot/trigger/pipeline",
    // );
    let response = Client::new().post(url).multipart(form).send().await;
    match response {
        Ok(_) => Ok(warp::reply()),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let hook = warp::post()
        .and(warp::path("nautobot"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .and_then(post_form);
    let routes = hook;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
