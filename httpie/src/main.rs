use clap::Parser;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use reqwest::Url;

// 以下部分用于处理 CLI

// 定义 httpie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其它 HTTP 方法
}

// get 子命令

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    // #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post 子命令。需要输入一个 url，和若干个可选的 key=value，用于提供 json body

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    // #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    // #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<String>,
}

/// 命令行中k=v, 可以通过parse_kv_pair 解析成KvPair结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

// FromStr是Rust标准库中定义的trait,实现后 可以调用字符串的parse()泛型函数，可以方便进行字符串的某个类型转换
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用"="进行split, 这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中依次取出值作为k/v, 迭代器返回 Some(T)/None
            // 我们将其转换成Ok(T)/Err(E),然后用？处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}
// 因为我们为KvPair实现了FromStr, 这里可以直接s.parse()得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
