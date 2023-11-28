use std::fs::{self, File};
use std::io::Write;

pub fn init_command() {
    let config_content = r#"
title = "My Blog"
author = "Your Name"
description = "This is my blog."
port = 9900
md_source_dir = "docs"
output_dir = ".site"
default_css_header = "<link rel=\"stylesheet\" href=\"./css/style.css\">"
default_code_header = "<link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/styles/default.min.css\"><script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/highlight.min.js\"></script>"
default_code_plugin = "<script>hljs.highlightAll();</script>"
"#;
    let _ = File::create("md_config.toml").and_then(|mut file| file.write_all(config_content.as_bytes()));

    // 创建所需的目录
    let _ = fs::create_dir_all("docs").map(|_| println!("created 'docs' Success"));
    let _ = fs::create_dir_all(".site").map(|_| println!("created '.site' Success"));
    let _ = fs::create_dir_all("static").map(|_| println!("created 'static' Success"));
    let _ = fs::create_dir_all("static/js").map(|_| println!("created 'static/js' Success"));
    let _ = fs::create_dir_all("static/css").map(|_| println!("created 'static/css' Success"));
    let _ = fs::create_dir_all("static/images").map(|_| println!("created 'static/images' Success"));
    let _ = File::create("static/css/style.css").and_then(|mut file| file.write_all(get_style_content().as_bytes())); 
    println!("Blog initialized with default configuration.");
}
fn get_style_content() -> &'static str{
    let style_content = r#"
    body {
        font-family: 'Arial', sans-serif;
        margin: 0;
        padding: 0;
        display: flex;
    }
    
    .toc {
        width: 20%;
        background-color: #f0f0f0;
        padding: 20px;
        height: 100vh;
        overflow: auto;
    }
    
    .content {
        width: 80%;
        padding: 50px;
        overflow: auto;
    }
    
    a {
        text-decoration: none;
        color: #0366d6;
    }
    
    a:hover {
        text-decoration: underline;
    }
    .toc ul {
        list-style: none;
        padding: 15;
    } 
    
    ul {
        list-style: none;
        padding: 0;
    }     
    "#;
    style_content
}
