use std::{time::Duration, collections::HashSet};

use anyhow::Result;
use futures_util::StreamExt;
use upnp_client::{
    device_client::DeviceClient,
    discovery::discover_pnp_locations,
    media_renderer::MediaRendererClient,
    types::{Device, LoadOptions, Metadata, ObjectClass},
};

async fn try_start_on_device(media_url: &str, load_options: LoadOptions, device: Device) -> Result<()> {
    let supports_render_control = device.services
        .iter()
        .find(|s| s.service_id == "urn:upnp-org:serviceId:RenderingControl")
        .is_some();

    if supports_render_control {
        let device_client = DeviceClient::new(&device.location).connect().await?;
        let media_renderer = MediaRendererClient::new(device_client);

        println!("Sending UPNP/DLNA Control to: {}", device.friendly_name);
        media_renderer.load(media_url, load_options).await?;
    }
    Ok(())
}

pub async fn start_via_upnp(media_url: &str) -> Result<()> {
    let options = LoadOptions {
        dlna_features: Some(
            "DLNA.ORG_OP=01;DLNA.ORG_CI=0;DLNA.ORG_FLAGS=01700000000000000000000000000000"
                .to_string(),
        ),
        content_type: Some("application/x-rtsp".to_string()),
        metadata: Some(Metadata {
            title: "Desktop".to_string(),
            ..Default::default()
        }),
        autoplay: true,
        object_class: Some(ObjectClass::Video),
        ..Default::default()
    };

    let start_task = async {
        let mut seen_devices = HashSet::new();
        let device_stream = discover_pnp_locations().await.unwrap();
        tokio::pin!(device_stream);
        while let Some(device) = device_stream.next().await {
            if !seen_devices.contains(&device.location) {
                seen_devices.insert(device.location.clone());
                let _ = try_start_on_device(media_url, options.clone(), device).await;
            }
        }
    };

    let _ = tokio::time::timeout(Duration::from_secs(5), start_task).await;

    Ok(())
}