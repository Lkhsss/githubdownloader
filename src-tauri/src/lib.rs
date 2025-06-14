use reqwest::header;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::{Host, Position, Url};

#[tauri::command]
async fn handle(url: String) -> GetResult {
    let handled_url = match Url::parse(&url) {
        Ok(o) => o,
        Err(_) => return GetResult::fail("无法解析链接"),
    };
    if handled_url.host_str() != Some("github.com") {
        return GetResult::fail("无法解析除了github之外的服务");
    }
    let url_data: Vec<_> = handled_url.path().split("/").collect();
    let (author, repo) = (url_data[1], url_data[2]);
    let data = match get_data(author.to_string(), repo.to_string()).await {
        Ok(d) => d,
        Err(_) => return GetResult::fail("获取失败"),
    };
    GetResult::new(data)
}

async fn get_data(author: String, repo: String) -> Result<Vec<Data>, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{}/{}/releases", author, repo);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Accept",
        header::HeaderValue::from_static("application/vnd.github+json'"),
    );
    headers.insert(
        "User-Agent",
        header::HeaderValue::from_static("Chrome/122.0.6261.95 Safari/537.36"),
    );
    let body = reqwest::Client::builder()
        .default_headers(headers)
        .build()?
        .get(url)
        .send()
        .await?
        .json::<Vec<Data>>()
        .await?;

    // println!("body = {body:?}");
    Ok(body)
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    url: String,
    assets_url: String,
    upload_url: String,
    html_url: String,
    id: usize,
    author: Author,
    assets: Vec<Asset>,
    node_id: String,
    tag_name: String,
    target_commitish: String,
    name: String,
    draft: bool,
    prerelease: bool,
    created_at: String,
    published_at: String,
    tarball_url:String,
    zipball_url:String
}
#[derive(Debug, Deserialize, Serialize)]
struct Author {
    login: String,
    id: usize,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    user_view_type: String,
    site_admin: bool,
}
#[derive(Debug, Deserialize, Serialize)]
struct Asset {
    url: String,
    id: usize,
    node_id: String,
    name: String,
    label: String,
    browser_download_url: String,
    created_at:String,
    updated_at:String,
    size:usize
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("链接格式错误")]
    ParseError(#[from] url::ParseError),
    #[error("链接格式错误")]
    NotTargetHost,
    #[error("unknown data store error")]
    Unknown,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Deserialize, Serialize)]
struct GetResult {
    success: bool,
    status: String,
    data: Option<Vec<Data>>,
}

impl GetResult {
    fn new(data: Vec<Data>) -> Self {
        Self {
            success: true,
            status: "获取成功".into(),
            data: Some(data),
        }
    }
    fn fail<S: ToString>(s: S) -> Self {
        Self {
            success: false,
            status: s.to_string(),
            data: None,
        }
    }
}
