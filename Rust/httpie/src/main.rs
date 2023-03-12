use clap::{AppSettings, Parser, SubCommand, Subcommand};

// 定义 HTTPie 的 CLI 主入口，它包含若干个子命令


/// A native httpie implementation with Rust, can you imagine how easy it is？
#[derive(Parser)]
#[clap(version = "1.0", author = "Jiaming Hu")]
#[clap(settings = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Subcommand)]
enum SubCommand {
    /// Get 方法
    Get {
        Get
    },
    /// Post 方法
    // 暂时没有实现其他方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Clap, Debug)]
struct Get {
    /// HTTP 请求的 URL
    url: String,
}

// post 子命令

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Clap, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}