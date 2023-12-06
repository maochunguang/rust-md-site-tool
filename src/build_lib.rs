use pulldown_cmark::{Parser, html};
use toml::Value;
use std::fs::{self, File};
use std::io::{Read, self};
use std::path::{PathBuf, Path};

pub fn build_command() {
    // ... 读取配置文件和设置目录 ...
    let config = fs::read_to_string("md_config.toml").expect("Unable to read config file");
    let parsed_config = config.parse::<Value>().expect("Unable to parse config");
    let source_dir =  parsed_config.get("md_source_dir").and_then(Value::as_str).unwrap_or("docs");
    let output_dir =  parsed_config.get("output_dir").and_then(Value::as_str).unwrap_or(".site");
    let static_dir =  parsed_config.get("static_dir").and_then(Value::as_str).unwrap_or("static");
    let default_css_header =  parsed_config.get("default_css_header").and_then(Value::as_str).unwrap_or("");
    let default_code_header =  parsed_config.get("default_code_header").and_then(Value::as_str).unwrap_or("");
    let default_code_plugin =  parsed_config.get("default_code_plugin").and_then(Value::as_str).unwrap_or("");
    println!("load config source_dir :{}, output_dir:{}", source_dir, output_dir);
    let md_source_dir = Path::new(source_dir);
    let summary_path =format!("{}{}", source_dir, "/summary.md");

 
    // 解析 summary.md 并构建目录HTML
    let toc_html = build_toc_content(summary_path.clone());

    let md_files = read_dir_recursive(md_source_dir).expect("read md dir failed");

    // 为每个Markdown文件生成HTML页面
    for entry in md_files {
        let path = entry;
        if path.ends_with("summary.md"){
            continue;
        }
        let output_file_path = transform_path(&path, source_dir, output_dir);
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let mut md_content = String::new();
            File::open(&path).and_then(|mut file| file.read_to_string(&mut md_content)).expect("Failed to read Markdown file");
            let parser = Parser::new(&md_content);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);
            let _ = html_content.replace(".md", ".html");
            let output_file = output_file_path.with_extension("html");
            println!("output file:{}", output_file.as_path().display());
            let mut full_html;
            // 处理index页面逻辑
            full_html = format!("<html><head></head><body><div class=\"toc\">{}</div><div class=\"content\">{}</div></body></html>", toc_html, html_content);
            full_html = append_html(&full_html, "</head>", &[default_css_header, default_code_header]);
            // 增加highlight.js处理代码块
            full_html = append_html(&full_html, "</body>", &[default_code_plugin]);
            // 处理相对路径
            if contains_sub_dir(&output_file, output_dir){
                full_html = full_html.replace("./", "../").replace("<a href=\"", "<a href=\"../")
            }

            if let Some(parent) = output_file.parent() {
                // 创建所有必要的父文件夹
                fs::create_dir_all(parent).expect("create html parent dir failed");
            }
            fs::write(output_file, full_html).expect("Failed to write HTML file");
        }
    }

    // ... 复制静态资源 ...
    // 定义静态资源目录和目标目录
    let static_dir = Path::new(static_dir);
    let output_dir = Path::new(output_dir);

    // 复制静态资源
    if static_dir.exists() {
        copy_dir_all(static_dir, output_dir).expect("Failed to copy static files");
    }

    println!("Site built successfully.");
}

fn contains_sub_dir( path:&PathBuf, site_path:&str)-> bool{
    let path_str = path.to_str().unwrap();
    println!("path_str:{}", path_str);
    let start_index = path_str.find(site_path).unwrap();
    let end_index = path_str.find(".html").unwrap();
    
    let count = path_str[start_index + site_path.len()..end_index].matches('/').count();
    count > 1
}
fn append_html(original_html:&str, pos:&str, insert_htmls:&[&str])-> String{
    // 查找 </head> 标签的位置
    let mut new_html = String::from(original_html);
    let insert_html = insert_htmls.join("");
    if let Some(pos) = original_html.find(pos) {
        // 将原始 HTML 字符串分为两部分，并在中间插入新的字符串
        new_html = format!(
            "{}{}{}",
            &original_html[..pos],
            insert_html,
            &original_html[pos..]
        );

        println!("{}", new_html);
    } else {
        // 如果没有找到 </head> 标签，可能需要错误处理
        println!("No </head> tag found in the HTML string.");
    }
    new_html
}

fn build_toc_content(path: String)-> String{
    let mut md_content = String::new();
    File::open(&path).and_then(|mut file| file.read_to_string(&mut md_content)).expect("Failed to read Markdown file");
    let parser = Parser::new(&md_content);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    html_content = html_content.replace(".md", ".html");
    html_content
}

fn transform_path(original: &Path, source_dir:&str, target_dir:&str) -> PathBuf {
    let mut new_path = PathBuf::new();

    let mut components = original.components();

    // 遍历路径的各个部分
    while let Some(component) = components.next() {
        // 当遇到源目录名时，替换为目标目录名
        if component.as_os_str() == source_dir {
            new_path.push(target_dir);
            break;
        } else {
            new_path.push(component);
        }
    }

    // 将剩余的路径部分添加到新路径中
    new_path.extend(components);

    new_path
}

//读取所有md文件
fn read_dir_recursive(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(read_dir_recursive(&path)?);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

// 递归地复制目录
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            // let file_type = entry.file_type()?;
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = dst.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::copy(src, dst)?;
    }
    Ok(())
}
