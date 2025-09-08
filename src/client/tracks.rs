use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::download;
use std::error::Error;
use std::path::PathBuf;

use crate::client::client::Client;
use crate::models::query::{Paging, TracksQuery};
use crate::models::response::{Track, Tracks};
use crate::response::{Stream, StreamType, Transcoding, Waveform};

impl Client {
    pub async fn search_tracks(
        &self,
        query: Option<&TracksQuery>,
    ) -> Result<Tracks, Box<dyn Error>> {
        let tracks: Tracks = self.get("search/tracks", query).await?;
        Ok(tracks)
    }

    pub async fn get_track_by_id(&self, id: &str) -> Result<Track, Box<dyn Error>> {
        let url = format!("tracks/{}", id);
        let resp: Track = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_track_by_urn(&self, urn: &str) -> Result<Track, Box<dyn Error>> {
        let url = format!("tracks/{}", urn);
        let resp: Track = self.get(&url, None::<&()>).await?;
        Ok(resp)
    }

    pub async fn get_track_related_by_id(
        &self,
        id: &str,
        pagination: Option<&Paging>,
    ) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("tracks/{}/related", id);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn get_track_related_by_urn(
        &self,
        urn: &str,
        pagination: Option<&Paging>,
    ) -> Result<Tracks, Box<dyn Error>> {
        let url = format!("tracks/{}/related", urn);
        let resp: Tracks = self.get(&url, pagination).await?;
        Ok(resp)
    }

    pub async fn download_track(
        &self,
        track: &Track,
        stream_type: Option<&StreamType>,
        destination: Option<&str>,
        filename: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        if track.title.is_none() {
            return Err("Track title is missing".into());
        }
        let title = match filename {
            Some(filename) => filename,
            None => track.title.as_ref().expect("Missing track title"),
        };

        let output_path = match destination {
            Some(destination) => PathBuf::from(destination).join(format!("{}.mp3", title)),
            None => PathBuf::from(format!("{}.mp3", title)),
        };
        if let Some(parent) = output_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let transcoding = Self::get_transcoding_by_stream_type(track, stream_type).await?;
        let stream_url = self.get_stream_url(track, stream_type).await?;

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
            _ => return Err("Invalid Stream Type".into()),
        }

        Ok(())
    }

    pub async fn get_track_waveform(&self, track: &Track) -> Result<Waveform, Box<dyn Error>> {
        let waveform_url = track.waveform_url.as_ref().expect("Missing waveform URL");
        let response = reqwest::get(waveform_url).await?;
        let waveform: Waveform = response.json::<Waveform>().await?;
        Ok(waveform)
    }

    pub async fn get_stream_url(
        &self,
        track: &Track,
        stream_type: Option<&StreamType>,
    ) -> Result<String, Box<dyn Error>> {
        let transcoding = Self::get_transcoding_by_stream_type(track, stream_type).await?;
        let path = transcoding.url.as_ref().ok_or("Missing transcoding URL")?;
        let stream: Stream = Self::get_json(path, None, None::<&()>, &self.client_id).await?;
        stream.url.ok_or("Missing resolved stream URL".into())
    }

    async fn get_transcoding_by_stream_type(
        track: &Track,
        stream_type: Option<&StreamType>,
    ) -> Result<Transcoding, Box<dyn Error>> {
        let transcodings = track
            .media
            .as_ref()
            .expect("Missing media")
            .transcodings
            .as_ref()
            .expect("Missing transcodings");
        if transcodings.is_empty() {
            return Err("No available download options".into());
        }

        let transcoding = match stream_type {
            Some(StreamType::Hls) => transcodings.iter().find(|t| {
                t.format
                    .as_ref()
                    .expect("Missing transcoding format")
                    .protocol
                    .as_ref()
                    .expect("Missing transcoding protocol")
                    == &StreamType::Hls
            }),
            Some(StreamType::Progressive) => transcodings.iter().find(|t| {
                t.format
                    .as_ref()
                    .expect("Missing transcoding format")
                    .protocol
                    .as_ref()
                    .expect("Missing transcoding protocol")
                    == &StreamType::Progressive
            }),
            _ => transcodings.iter().find(|t| {
                let protocol =t.format
                    .as_ref()
                    .expect("Missing transcoding format")
                    .protocol
                    .as_ref()
                    .expect("Missing transcoding protocol");
                protocol == &StreamType::Progressive || protocol == &StreamType::Hls
            }),
        };
        Ok(transcoding.expect("No available download options").clone())
    }

    async fn download_progressive(
        &self,
        stream_url: &str,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        let response = reqwest::get(stream_url).await?;
        let bytes = response.bytes().await?;
        tokio::fs::write(output_path, &bytes).await?;
        Ok(())
    }

    async fn download_hls(
        &self,
        stream_url: &str,
        output_path: &PathBuf,
    ) -> Result<(), Box<dyn Error>> {
        download::auto_download()?;
        let status = FfmpegCommand::new()
            .input(stream_url)
            .output(
                output_path
                    .to_str()
                    .expect("Failed to convert output path to string"),
            )
            .args(["-c", "copy"])
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err("Download HLS Failed".into());
        }
        Ok(())
    }
}
