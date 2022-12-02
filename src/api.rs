use reqwest as requests;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    page: ApiPageInformation,
    items: Vec<ProgrammeItem>
}

#[derive(Deserialize, Debug)]
struct ApiPageInformation {
    total: i32,
}

#[derive(Deserialize, Debug)]
struct ProgrammeItem {
    pid: String
}

fn create_api_url(
    api_env: &str,
    endpoint: &str,
    tleo_pid: &str,
    offset: i32
) -> String {
    match api_env {
        "live" => format!("https://programmes-metadata.api.bbci.co.uk/programmes/{}?ancestor={}&limit=100&offset={}&sort=releaseDate", endpoint, tleo_pid, offset),
        "int" | "test" => format!("https://programmes-metadata.{}.api.bbci.co.uk/programmes/{}?ancestor={}&limit=100&offset={}&sort=releaseDate", api_env, endpoint, tleo_pid, offset),
        _ => "".to_string()
    }
}

pub async fn bulk_call_api(tleo_pid: String, environment: String, endpoint: String) -> Result<Vec<String>> {
    let api_key = match std::env::var("PROGS_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("API Key not set!")
    };

    println!("Bulk calling API ({})", tleo_pid);
    let mut pids = Vec::new();
    let mut offset = 100;
    let mut url = create_api_url(&environment, &endpoint, &tleo_pid, 0);
    let mut response = call_api(&url, &api_key).await?;

    let mut remaining_items = response.page.total - 100;
    pids.extend(response.items.drain(..).map(|p| p.pid));

    while remaining_items > 0 {
        url = create_api_url(&environment, &endpoint, &tleo_pid, offset);
        response = call_api(&url, &api_key).await?;
        pids.extend(response.items.drain(..).map(|p| p.pid));
        offset += 100;
        remaining_items -= 100;
    }

    Ok(pids)
}

pub async fn call_api(url: &str, api_key: &str) -> Result<ApiResponse> {
    let client = requests::Client::new();
    let response_text = client.get(url)
        .header("x-api-key", api_key)
        .send()
        .await?
        .text()
        .await?;

    let programmes = serde_json::from_str(&response_text)?;

    Ok(programmes)
}