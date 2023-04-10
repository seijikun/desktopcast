use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use upnp_client::{
    device_client::DeviceClient,
    discovery::discover_pnp_locations,
    media_renderer::MediaRendererClient,
    types::{Device, LoadOptions, Metadata, ObjectClass},
};

const KODI_MEDIA_RENDERER: &str = "Kodi - Media Renderer";

pub async fn start_on_kodi(media_url: &str) -> Result<()> {
    let devices = discover_pnp_locations();
    tokio::pin!(devices);

    let kodi_device = async {
        let mut kodi_device: Option<Device> = None;
        while let Some(device) = devices.next().await {
            // Select the first Kodi device found
            if device.model_description == Some(KODI_MEDIA_RENDERER.to_string()) {
                kodi_device = Some(device);
                break;
            }
        }

        kodi_device.ok_or_else(|| anyhow!("Error while searching for supported UPNP/DLNA player"))
    };

    let kodi_device = kodi_device.await?;
    let device_client = DeviceClient::new(&kodi_device.location).connect().await?;
    let media_renderer = MediaRendererClient::new(device_client);

    let options = LoadOptions {
        dlna_features: Some(
            "DLNA.ORG_OP=01;DLNA.ORG_CI=0;DLNA.ORG_FLAGS=01700000000000000000000000000000"
                .to_string(),
        ),
        content_type: Some("application/x-rtp".to_string()),
        metadata: Some(Metadata {
            title: "Desktop".to_string(),
            ..Default::default()
        }),
        autoplay: true,
        object_class: Some(ObjectClass::Video),
        ..Default::default()
    };

    media_renderer.load(media_url, options).await?;

    Ok(())
}
