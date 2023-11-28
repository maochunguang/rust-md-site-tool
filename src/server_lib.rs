// use std::io::{Read, Write, self};
use std::fs;
use tiny_http::{Server, Response};
// use toml::Value;

pub fn run_command() {
    // 读取配置文件
    // let config = fs::read_to_string("md_config.toml").expect("Unable to read config file");
    // let parsed_config = config.parse::<Value>().expect("Unable to parse config");
    // let port = parsed_config.get("port").and_then(Value::as_str).unwrap_or("9900");

    let server = Server::http("0.0.0.0:9900").unwrap();
    println!("Running local server on port 9900");

    for request in server.incoming_requests() {
        let response = match request.url() {
            "/" | "/index.html" => {
                let html = fs::read_to_string("index.html").expect("Unable to read index.html");
                Response::from_string(html)
            }
            _ => Response::from_string("Not Found").with_status_code(404),
        };

        request.respond(response).unwrap();
    }
}