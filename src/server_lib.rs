// use std::io::{Read, Write, self};
use std::{fs, path::PathBuf};
use tiny_http::{Header, Response, Server};
use toml::Value;
// use toml::Value;

pub fn run_command() {
    // 读取配置文件
    let config = fs::read_to_string("md_config.toml").expect("Unable to read config file");
    let parsed_config = config.parse::<Value>().expect("Unable to parse config");
    let port = parsed_config
        .get("port")
        .and_then(Value::as_str)
        .unwrap_or("9900");
    let output_dir = parsed_config
        .get("output_dir")
        .and_then(Value::as_str)
        .unwrap_or(".site");
    let address = format!("0.0.0.0:{}", port);
    let server = Server::http(address).unwrap();
    println!("Running local server on port {}", port);

    for request in server.incoming_requests() {
        let url = if request.url() == "/" {
            "/index.html" // 使用 index.html 作为根路径的默认页面
        } else {
            request.url()
        };

        let file_path = PathBuf::from(output_dir).join(&url[1..]); // 移除 URL 的首个斜杠
        match fs::read(&file_path) {
            Ok(contents) => {
                let response = Response::from_data(contents).with_header(
                    Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap(),
                );
                request.respond(response).unwrap();
            }
            Err(_) => {
                let response = Response::from_string("Not Found").with_status_code(404);
                request.respond(response).unwrap();
            }
        }
    }
}
