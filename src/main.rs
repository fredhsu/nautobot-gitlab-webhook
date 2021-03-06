mod avd;
mod batfish;
mod gitlab;
mod nautobot;

use log::{debug, info};
use nautobot::Data::Ipaddress;
use nautobot::{IPAddress, Nautobot, Query, WebhookRequest};
use std::env;
use std::str;
use warp::Filter;

#[derive(Debug)]
pub enum Error {
    YAMLError(serde_yaml::Error),
    JSONError(serde_json::Error),
    CommitError(gitlab::Error),
    WarpError(warp::Rejection),
    TokioError(tokio::task::JoinError),
}

fn get_gitlab_from_env() -> gitlab::Gitlab {
    let host = env::var("GITLAB_HOST").unwrap_or(String::from("dmz-gitlab"));
    let project = env::var("GITLAB_PROJECT").unwrap_or(String::from("5"));
    let branch = env::var("GITLAB_BRANCH").unwrap_or(String::from("nautobot"));
    let token = env::var("GITLAB_TOKEN").unwrap_or(String::from("NnnPwyihFTVRsnqk_dfi"));
    gitlab::Gitlab {
        host,
        project: project.parse().unwrap_or(5),
        branch,
        token,
    }
}

fn add_critical_ip(_ipaddr: IPAddress) -> Result<gitlab::CommitResponse, Error> {
    // Optimize by reducing the query only if a critical was changed
    // let critical_ips = ipaddr.tags.iter().filter(|t| t.slug == "critical");
    let files = generate_files()?;
    let gl = get_gitlab_from_env();
    let cr = gl.commit_files(files).map_err(Error::CommitError)?;
    Ok(cr)
}

/// generate_files() queries Nautobot for ip addresses that are critical, then generates the yaml and json for avd and batfish respectively
fn generate_files() -> Result<Vec<gitlab::FileEntry>, Error> {
    let query = query_nautobot();
    match query {
        Ok(data) => {
            let ip_addresses = data.data.ip_addresses;
            let avd_yaml = generate_avd(&ip_addresses).map_err(Error::YAMLError)?;
            let avd_file = gitlab::FileEntry {
                file_path: "avd.yaml".to_owned(),
                content: avd_yaml,
            };
            let bf_json = generate_batfish(&ip_addresses).map_err(Error::JSONError)?;
            let bf_file = gitlab::FileEntry {
                file_path: "bf.json".to_owned(),
                content: bf_json,
            };
            let files = vec![avd_file, bf_file];
            Ok(files)
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn generate_avd(ips: &[nautobot::IpAddressType]) -> Result<String, serde_yaml::Error> {
    info!("generating avd");
    let avd_acls = avd::permit_from_ips(ips);
    let yaml = serde_yaml::to_string(&avd_acls)?;
    debug!("Generated AVD: \n{}", &yaml);
    Ok(yaml)
}
fn generate_batfish(ips: &[nautobot::IpAddressType]) -> Result<String, serde_json::Error> {
    info!("generating batfish");
    let bfpolicy = batfish::permit_from_ips(ips);
    let bfjson = serde_json::to_string(&bfpolicy)?;
    debug!("Generated batfish: \n{}", &bfjson);
    Ok(bfjson)
}

fn query_nautobot() -> Result<nautobot::GqlData, Box<dyn std::error::Error>> {
    let hostname = env::var("NAUTOBOT").unwrap_or("nautobot".to_string());
    let url = format!("https://{}/api/graphql/", hostname);
    let allow_insecure = "true" == env::var("NAUTOBOT_INSECURE").unwrap_or("true".to_string());
    let token = env::var("NAUTOBOT_TOKEN")
        .unwrap_or("f6df868dfa674ff1d5fdfaac169eda87a55d2d93".to_string());
    let nb = Nautobot {
        hostname,
        token,
        url,
        allow_insecure,
    };
    let tag = "\"critical\"".to_string();
    let query_string = format!("query {{ip_addresses(tag:{}){{address dns_name}}}}", tag);
    debug!("query_string: {}", query_string);
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
    // Parse webhook data into WebhookRequest struct
    let webhook: WebhookRequest = serde_json::from_str(bodystr).unwrap();
    let ipaddr = webhook.data;
    // Pattern match to get ip address from enum possibilities / tags
    let _aclmap = match ipaddr {
        Ipaddress(ip) => {
            // IP address doesn't really matter.. really just using this for error checking
            // Spawning as a blocking thread
            tokio::task::spawn_blocking(move || {
                let res = add_critical_ip(ip).unwrap();

                debug!("Files commited with response: {:?}", res);
            })
            .await;
        }
    };
    // TODO: add reply
    Ok(warp::reply())
}

#[tokio::main]
async fn main() {
    env_logger::init();
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
        assert!(add_critical_ip(ipaddr).is_ok());
    }
    #[test]
    fn test_generate_avd() {
        let ipaddr = nautobot::IpAddressType {
            address: "1.1.1.1/32".to_owned(),
        };
        let ip_addresses = vec![ipaddr];
        let avd_yaml = generate_avd(&ip_addresses).unwrap();
        assert!(avd_yaml.ends_with("permit ip any 1.1.1.1/32\n"))
    }
    #[test]
    fn test_generate_batfish() {
        let ipaddr = nautobot::IpAddressType {
            address: "1.1.1.1/32".to_owned(),
        };
        let ip_addresses = vec![ipaddr];
        let bf_json = generate_batfish(&ip_addresses).unwrap();
        assert!(bf_json.contains("1.1.1.1/32"))
    }
}
