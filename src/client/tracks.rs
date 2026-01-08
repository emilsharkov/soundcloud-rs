use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::download;
use std::path::{Path, PathBuf};

use crate::models::client::Client;
use crate::models::client::Identifier;
use crate::models::error::Error;
use crate::models::query::{Paging, TracksQuery};
use crate::models::response::{Track, Tracks};
use crate::response::{Stream, StreamType, Transcoding, Waveform};

impl Client {
    pub async fn search_tracks(
        &self,
        query: Option<&TracksQuery>,
    ) -> Result<Tracks, Error> {
        let tracks: Tracks = self.get("search/tracks", query).await?;
        Ok(tracks)
    }

    pub async fn get_track(
        &self,
        identifier: &Identifier,
    ) -> Result<Track, Error> {
        let url = format!("tracks/{identifier}");
        let resp: Track = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_track_related(
        &self,
        identifier: &Identifier,
        pagination: Option<&Paging>,
    ) -> Result<Tracks, Error> {
        let url = format!("tracks/{identifier}/related");
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn download_track(
        &self,
        identifier: &Identifier,
        stream_type: Option<&StreamType>,
        destination: Option<&str>,
        filename: Option<&str>,
    ) -> Result<(), Error> {
        let track = self.get_track(identifier).await?;

        let stream = match stream_type {
            Some(stream_type) => stream_type,
            None => &StreamType::Progressive,
        };

        if track.title.is_none() {
            return Err(Error::new("Track title is missing"));
        }

        let title = match filename {
            Some(filename) => filename,
            None => track.title.as_ref().expect("Missing track title"),
        };

        let output_path = match destination {
            Some(destination) => PathBuf::from(destination).join(format!("{title}.mp3")),
            None => PathBuf::from(format!("{title}.mp3")),
        };
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let transcoding = self.get_transcoding_by_stream_type(&track, stream).await?;
        let stream_url = self.get_stream_url(identifier, Some(stream)).await?;

        match transcoding
            .format
            .as_ref()
            .expect("Missing transcoding format")
            .protocol
            .as_ref()
        {
            Some(StreamType::Progressive) => {
                self.download_progressive(&stream_url, &output_path).await?
            }
            Some(StreamType::Hls) => self.download_hls(&stream_url, &output_path).await?,
            _ => return Err(Error::new("Invalid Stream Type")),
        }

        Ok(())
    }

    pub async fn get_track_waveform(
        &self,
        identifier: &Identifier,
    ) -> Result<Waveform, Error> {
        let track = self.get_track(identifier).await?;
        let waveform_url = track.waveform_url.as_ref().expect("Missing waveform URL");
        let response = reqwest::get(waveform_url).await?;
        let waveform: Waveform = response.json::<Waveform>().await?;
        Ok(waveform)
    }

    pub async fn get_stream_url(
        &self,
        identifier: &Identifier,
        stream_type: Option<&StreamType>,
    ) -> Result<String, Error> {
        let track = self.get_track(identifier).await?;
        let stream = match stream_type {
            Some(stream_type) => stream_type,
            None => &StreamType::Progressive,
        };
        let transcoding = self.get_transcoding_by_stream_type(&track, stream).await?;
        let path = transcoding.url.as_ref().ok_or_else(|| Error::new("Missing transcoding URL"))?;
        let client_id = self.get_client_id_value().await;
        let (stream, _): (Stream, _) = Self::get_json(path, None, None::<&()>, &client_id).await?;
        stream.url.ok_or_else(|| Error::new("Missing resolved stream URL"))
    }

    async fn get_transcoding_by_stream_type(
        &self,
        track: &Track,
        stream_type: &StreamType,
    ) -> Result<Transcoding, Error> {
        let transcodings = track
            .media
            .as_ref()
            .expect("Missing media")
            .transcodings
            .as_ref()
            .expect("Missing transcodings");
        if transcodings.is_empty() {
            return Err(Error::new("No available download options"));
        }

        let transcoding: Option<Transcoding> = {
            for t in transcodings {
                let protocol = match t.format.as_ref().and_then(|f| f.protocol.as_ref()) {
                    Some(p) => p,
                    None => continue,
                };
                if *protocol != *stream_type {
                    continue;
                }

                let path = match t.url.as_ref() {
                    Some(u) => u,
                    None => continue,
                };

                let client_id = self.get_client_id_value().await;
                let (stream, _): (Stream, _) =
                    Self::get_json(path, None, None::<&()>, &client_id).await?;
                if stream.url.is_some() {
                    return Ok(t.clone());
                }
            }
            None
        };
        Ok(transcoding.expect("No available download options"))
    }

    async fn download_progressive(
        &self,
        stream_url: &str,
        output_path: &Path,
    ) -> Result<(), Error> {
        let response = reqwest::get(stream_url).await?;
        let bytes = response.bytes().await?;
        tokio::fs::write(output_path, &bytes).await?;
        Ok(())
    }

    async fn download_hls(
        &self,
        stream_url: &str,
        output_path: &Path,
    ) -> Result<(), Error> {
        download::auto_download().map_err(|e| Error::new(format!("FFmpeg download failed: {}", e)))?;
        let status = FfmpegCommand::new()
            .input(stream_url)
            .output(
                output_path
                    .to_str()
                    .expect("Failed to convert output path to string"),
            )
            .args(["-c", "copy"])
            .spawn()
            .map_err(|e| Error::new(format!("FFmpeg spawn failed: {}", e)))?
            .wait()
            .map_err(|e| Error::new(format!("FFmpeg wait failed: {}", e)))?;

        if !status.success() {
            return Err(Error::new("Download HLS Failed"));
        }
        Ok(())
    }
}
