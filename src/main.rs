use anyhow::{anyhow, Result};
use config::{DesktopCastConfig, Resolution};
use stream_server::StreamServer;

mod config;
mod stream_server;
mod upnp;

fn get_own_ip() -> Result<String> {
    Ok(get_if_addrs::get_if_addrs()?
        .iter()
        .filter(|a| !a.is_loopback())
        .next()
        .ok_or_else(|| anyhow!("No public ip address found"))?
        .ip()
        .to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    gstreamer::init()?;
    gstretimestamp::plugin_register_static()?;

    let config = DesktopCastConfig {
        target_resolution: Some(Resolution {
            width: 1920,
            height: 1080,
        }),
    };

    let mut stream_server = StreamServer::new();
    stream_server.start(&config).await?;

    let own_ip = get_own_ip()?;
    upnp::start_on_kodi(&format!("rtsp://{}:8554", own_ip)).await?;

    stream_server.run()?;

    Ok(())
}
