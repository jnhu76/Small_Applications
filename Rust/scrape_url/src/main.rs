use std::fs;

fn main() {
    let url = "https://www.baidu.com/";
    let output = "baidu.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Convert markdown has been saved in {}.", output);
}