mod avd;
mod batfish;
mod gitlab;
mod nautobot;

use crate::nautobot::Data::Ipaddress;
use crate::nautobot::WebhookRequest;
// use serde_json::Result;
use bytes;
use nautobot::{IPAddress, Nautobot, Query};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::str;
use warp::{Buf, Filter, Reply};

fn add_critical_ip(ipaddr: IPAddress) -> avd::StandardACLs {
    // Optimize by reducing the query only if a critical was changed
    let critical_ips = ipaddr.tags.iter().filter(|t| t.slug == "critical");
    // Query nautobot for all critical ips to build acl from scratch (stateless)
    let query = query_nautobot();
    match query {
        Ok(data) => build_acls(data.data.ip_addresses),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn build_acls(ips: Vec<nautobot::IpAddressType>) -> avd::StandardACLs {
    let mut sacls = HashMap::new();
    let mut seqn = HashMap::new();
    for (i, ip) in ips.iter().enumerate() {
        let action = format!("permit ip any {}", ip.address);
        let ale = avd::AccessListEntry { action: action };
        seqn.insert((i as i32 + 1) * 10, ale);
    }
    let sacl = avd::StandardACL {
        sequence_numbers: seqn,
    };
    sacls.insert("critical".to_owned(), sacl);
    avd::StandardACLs {
        standard_access_lists: sacls,
    }
}

fn query_nautobot() -> Result<nautobot::GqlData, Box<dyn Error>> {
    let nb = Nautobot {
        hostname: "nautobot".to_string(),
        token: "f6df868dfa674ff1d5fdfaac169eda87a55d2d93".to_string(),
        url: "https://nautobot/api/graphql/".to_string(),
        allow_insecure: true,
    };
    let tag = "\"critical\"".to_string();
    let query_string = format!("query {{ip_addresses(tag:{}){{address dns_name}}}}", tag);
    println!("query_string: {}", query_string);
    let query = Query {
        query: query_string,
    };
    let result = nb.query(query);
    match result {
        Ok(r) => Ok(r.json::<nautobot::GqlData>()?),
        Err(e) => Err(e),
    }
}
pub async fn post_form(body: bytes::Bytes) -> Result<impl warp::Reply, warp::Rejection> {
    let bodystr = str::from_utf8(body.as_ref()).unwrap();
    println!("{:?}", bodystr);
    // Parse webhook data into WebhookRequest struct
    let webhook: WebhookRequest = serde_json::from_str(bodystr).unwrap();
    let ipaddr = webhook.data;
    println!("{:?}", ipaddr);
    // Pattern match to get ip address from enum possibilities / tags
    match ipaddr {
        Ipaddress(ip) => {
            let aclmap = add_critical_ip(ip);
            let yaml = serde_yaml::to_string(&aclmap);
            println!("Generated Yaml: {}", yaml.unwrap());
            // Build batfish
            // Commit to repo
        }
    }
    // TODO: add reply
    Ok(warp::reply())
    // println!(
    //     "postform address : {:?} name: {}",
    //     webhook.data, webhook.data.assigned_object.name
    // );
    // let token = String::from("672ace375be2dcccb85fa6add30138");
    // let form = reqwest::multipart::Form::new()
    //     .text("token", token)
    //     .text("ref", "nautobot")
    //     .text("variables[ADDRESS]", webhook.data.address)
    //     .text("variables[NAME]", webhook.data.assigned_object.name);
    // let url =
    //     String::from("http://dmz-gitlab.sjc.aristanetworks.com/api/v4/projects/5/trigger/pipeline");
    // let url = String::from(
    //     "http://dmz-gitlab.sjc.aristanetworks.com/api/v4/projects/5/ref/nautobot/trigger/pipeline",
    // );
    // let response = Client::new().post(url).multipart(form).send().await;
    // match response {
    //     Ok(_) => Ok(warp::reply()),
    //     Err(_) => Err(warp::reject::not_found()),
    // }
}

#[tokio::main]
async fn main() {
    let hook = warp::post()
        .and(warp::path("nautobot"))
        .and(warp::body::content_length_limit(1024 * 32))
        // .and(warp::body::json())
        .and(warp::body::bytes())
        .and_then(post_form);
    let routes = hook;
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_critical_ip() {
        let ipaddr = IPAddress {
            address: "1.1.1.1/32".to_owned(),
            assigned_object: None,
            family: nautobot::Family {
                label: "ipv4".to_owned(),
                value: 4,
            },
            tags: Vec::new(),
            id: "".to_owned(),
            url: "".to_owned(),
        };
        add_critical_ip(ipaddr);
        // match result {
        //     Ok(r) => assert_eq!(r.status(), 200),
        //     Err(e) => assert!(false, "Failed query: {}", e),
        // }
    }
}
