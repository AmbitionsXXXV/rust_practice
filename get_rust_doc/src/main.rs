use get_rust_doc::Config;
use std::{env, fs, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err); // 打印错误信息
        process::exit(1); // 退出程序，返回错误状态码
    });
    println!("{:?}", config);

    println!("Fetching url: {}", config.url);
    let body = reqwest::blocking::get(config.url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(&config.output_path, md.as_bytes()).unwrap();
    println!(
        "Converted markdown has been saved in {}.",
        config.output_path
    );
}
