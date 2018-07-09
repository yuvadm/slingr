# Slingr

[![Build Status](https://travis-ci.org/yuvadm/slingr.svg?branch=master)](https://travis-ci.org/yuvadm/slingr)
[![Crates.io](https://img.shields.io/crates/v/slingr.svg)](https://crates.io/crates/slingr)

A simple CLI for streaming media files over a local network to UPnP media renderers.

Designed to work with cheap HDMI/DLNA/UPnP/Miracast Dongles.

The `r` stands for Rust.

## Features

 - [ ] Stream a video file to a UPnP Media Renderer
 - [ ] Play and pause video during playback
 - [ ] Skip forward and backward during playback
 - [ ] Add subtitles to the video
 - [ ] Integrate with ffmpeg to do on the fly transcoding

## Usage

Make sure you have a UPnP streamer online on your local network, then run:

```bash
$ slingr ~/path/to/media.file
```

To target a specific device:

```bash
$ slingr --device 192.168.33.44 ~/path/to/media.file
```

### Controls

During playback use the following controls:

 - <kbd>Space</kbd> - Play / Pause
 - <kbd>q</kbd> - Quit

## License

[GPLv3](LICENSE)
