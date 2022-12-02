mod helpers;
mod api;

use anyhow::Result;
use api::bulk_call_api;
use helpers::parse_config;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = parse_config()?;

    let tleo_pid = &cfg.tleo_pids[0];

    let ep_fut = tokio::spawn(bulk_call_api(tleo_pid.clone(), cfg.api_environment, "episodes".to_string()));
    let clip_fut = tokio::spawn(bulk_call_api(tleo_pid.clone(), cfg.environment, "clips".to_string()));

    let episodes = ep_fut.await??;
    let clips = clip_fut.await??;

    println!("Episodes: {:?}", episodes);
    println!("Clips: {:?}", clips);
    Ok(())
}
