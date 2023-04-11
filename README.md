# desktopcast

Desktopcast is a little CLI application that allows you to cast your Linux desktop to any UPNP/DLNA device capable of the AVTransfer service, like Kodi using the RTSP streaming protocol.

At the moment, desktopcast is pretty much tailored to my use-case, which is casting my desktop to Kodi. Though is supports both X11 and Wayland (.. if your window manager properly supports Wayland).

## Hot it works
GStreamer is used to create an `rtsp://` server.
This server is fed from a pipeline that captures your desktop as well as your current audio output (the sound that other applications output to your speakers) and forwards this to any client connecting to the server. This server is reachable under `rtsp://<yourip>:8554`.

As soon as this server is setup, the uri under which this server can be reached within your local network is sent to all reachable UPNP/DLNA AVTransfer-capable devices that are found within 5 seconds.
These devices then connect to the rtsp server that desktopcast hosts and displays the stream sent to them via the GStreamer pipeline.

### Screencapture
To specify what should be screencaptured and cast, desktopcast attempts to use `xdg-desktop-portal`, which requires pipewire.
If the portal API is supported, a small window pops up that lets you select a screen or window to cast.

**Hint**: At the moment, under KDE, `xdg-desktop-portal` screencasting is only supported with the Wayland backend, not with the X11 backend.

If using `xdg-desktop-portal` fails, desktopcast falls back casting your primary monitor using X11 screencapture.
And if that fails, it falls back to screencasting the entire X11 screen surface.

### Audiocapture
For audio capturing, only pulseaudio is supported at the moment. (works also with pipewire if the pipewire-pulse bridge is installed).

The GStreamer API is used to discover your soundcard's `monitor` device, which is then forwarded. If this is not found, desktopcast fails.
