use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub thumbnail: Option<String>,
    pub playback_status: String,
    pub position: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSession {
    pub id: String,
    pub name: String,
}

#[cfg(windows)]
mod windows_media {
    use super::*;
    use windows::{
        Foundation::IAsyncOperation,
        Media::Control::{
            GlobalSystemMediaTransportControlsSession,
            GlobalSystemMediaTransportControlsSessionManager,
            GlobalSystemMediaTransportControlsSessionMediaProperties,
            GlobalSystemMediaTransportControlsSessionPlaybackStatus,
        },
        Storage::Streams::{DataReader, DataReaderLoadOperation},
    };

    // Blocking wait for async operations
    fn wait_for_async<T: windows::core::RuntimeType + 'static>(
        op: IAsyncOperation<T>,
    ) -> windows::core::Result<T> {
        use windows::Foundation::AsyncStatus;
        loop {
            let status = op.Status()?;
            match status {
                AsyncStatus::Completed => return op.GetResults(),
                AsyncStatus::Error => return Err(op.ErrorCode()?.into()),
                AsyncStatus::Canceled => return Err(windows::core::Error::from_win32()),
                _ => std::thread::sleep(std::time::Duration::from_millis(10)),
            }
        }
    }

    fn wait_for_load(op: DataReaderLoadOperation) -> windows::core::Result<u32> {
        use windows::Foundation::AsyncStatus;
        loop {
            let status = op.Status()?;
            match status {
                AsyncStatus::Completed => return op.GetResults(),
                AsyncStatus::Error => return Err(op.ErrorCode()?.into()),
                AsyncStatus::Canceled => return Err(windows::core::Error::from_win32()),
                _ => std::thread::sleep(std::time::Duration::from_millis(10)),
            }
        }
    }

    pub fn get_media_sessions_sync() -> Result<Vec<MediaSession>, String> {
        let manager_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| format!("Failed to get session manager: {}", e))?;

        let manager = wait_for_async(manager_op)
            .map_err(|e| format!("Failed to await session manager: {}", e))?;

        let sessions = manager
            .GetSessions()
            .map_err(|e| format!("Failed to get sessions: {}", e))?;

        let mut result = Vec::new();
        for i in 0..sessions.Size().unwrap_or(0) {
            if let Ok(session) = sessions.GetAt(i) {
                if let Ok(source_info) = session.SourceAppUserModelId() {
                    let id = source_info.to_string();
                    let name = id.split('.').last().unwrap_or(&id).to_string();
                    result.push(MediaSession { id, name });
                }
            }
        }

        Ok(result)
    }

    pub fn get_current_media_info_sync(session_id: Option<String>) -> Result<MediaInfo, String> {
        let manager_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| format!("Failed to get session manager: {}", e))?;

        let manager = wait_for_async(manager_op)
            .map_err(|e| format!("Failed to await session manager: {}", e))?;

        let session = if let Some(id) = session_id {
            let sessions = manager
                .GetSessions()
                .map_err(|e| format!("Failed to get sessions: {}", e))?;

            let mut found_session: Option<GlobalSystemMediaTransportControlsSession> = None;
            for i in 0..sessions.Size().unwrap_or(0) {
                if let Ok(s) = sessions.GetAt(i) {
                    if let Ok(source_info) = s.SourceAppUserModelId() {
                        if source_info.to_string() == id {
                            found_session = Some(s);
                            break;
                        }
                    }
                }
            }

            found_session.ok_or_else(|| format!("Session not found: {}", id))?
        } else {
            manager
                .GetCurrentSession()
                .map_err(|e| format!("Failed to get current session: {}", e))?
        };

        let media_props_op = session
            .TryGetMediaPropertiesAsync()
            .map_err(|e| format!("Failed to get media properties: {}", e))?;

        let media_properties = wait_for_async(media_props_op)
            .map_err(|e| format!("Failed to await media properties: {}", e))?;

        let playback_info = session
            .GetPlaybackInfo()
            .map_err(|e| format!("Failed to get playback info: {}", e))?;

        let playback_status = playback_info
            .PlaybackStatus()
            .map_err(|e| format!("Failed to get playback status: {}", e))?;

        let timeline = session
            .GetTimelineProperties()
            .map_err(|e| format!("Failed to get timeline: {}", e))?;

        let position = timeline
            .Position()
            .map_err(|e| format!("Failed to get position: {}", e))?
            .Duration as f64
            / 10_000_000.0;

        let duration = timeline
            .EndTime()
            .map_err(|e| format!("Failed to get duration: {}", e))?
            .Duration as f64
            / 10_000_000.0;

        let title = media_properties
            .Title()
            .map_err(|e| format!("Failed to get title: {}", e))?
            .to_string();

        let artist = media_properties
            .Artist()
            .map_err(|e| format!("Failed to get artist: {}", e))?
            .to_string();

        let album = media_properties
            .AlbumTitle()
            .unwrap_or_default()
            .to_string();

        let thumbnail = get_thumbnail_sync(media_properties).ok();

        let status_str = match playback_status {
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => "playing",
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => "paused",
            GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped => "stopped",
            _ => "unknown",
        }
        .to_string();

        Ok(MediaInfo {
            title,
            artist,
            album,
            thumbnail,
            playback_status: status_str,
            position,
            duration,
        })
    }

    fn get_thumbnail_sync(
        properties: GlobalSystemMediaTransportControlsSessionMediaProperties,
    ) -> Result<String, String> {
        let thumbnail_ref = properties
            .Thumbnail()
            .map_err(|e| format!("Failed to get thumbnail: {}", e))?;

        let stream_op = thumbnail_ref
            .OpenReadAsync()
            .map_err(|e| format!("Failed to open stream: {}", e))?;

        let stream =
            wait_for_async(stream_op).map_err(|e| format!("Failed to await stream: {}", e))?;

        let size = stream
            .Size()
            .map_err(|e| format!("Failed to get size: {}", e))?;

        let reader = DataReader::CreateDataReader(&stream)
            .map_err(|e| format!("Failed to create reader: {}", e))?;

        let load_op = reader
            .LoadAsync(size as u32)
            .map_err(|e| format!("Failed to load data: {}", e))?;

        wait_for_load(load_op).map_err(|e| format!("Failed to await load: {}", e))?;

        let mut buffer = vec![0u8; size as usize];
        reader
            .ReadBytes(&mut buffer)
            .map_err(|e| format!("Failed to read bytes: {}", e))?;

        let base64 = base64_encode(&buffer);
        Ok(format!("data:image/png;base64,{}", base64))
    }

    fn base64_encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        let mut i = 0;

        while i < data.len() {
            let b1 = data[i];
            let b2 = if i + 1 < data.len() { data[i + 1] } else { 0 };
            let b3 = if i + 2 < data.len() { data[i + 2] } else { 0 };

            result.push(CHARS[(b1 >> 2) as usize] as char);
            result.push(CHARS[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);

            if i + 1 < data.len() {
                result.push(CHARS[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char);
            } else {
                result.push('=');
            }

            if i + 2 < data.len() {
                result.push(CHARS[(b3 & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }

        result
    }

    pub fn control_playback_sync(action: &str) -> Result<(), String> {
        let manager_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
            .map_err(|e| format!("Failed to get session manager: {}", e))?;

        let manager = wait_for_async(manager_op)
            .map_err(|e| format!("Failed to await session manager: {}", e))?;

        let session = manager
            .GetCurrentSession()
            .map_err(|e| format!("Failed to get current session: {}", e))?;

        match action {
            "play" => {
                let op = session
                    .TryPlayAsync()
                    .map_err(|e| format!("Failed to play: {}", e))?;
                wait_for_async(op).map_err(|e| format!("Failed to await play: {}", e))?;
            }
            "pause" => {
                let op = session
                    .TryPauseAsync()
                    .map_err(|e| format!("Failed to pause: {}", e))?;
                wait_for_async(op).map_err(|e| format!("Failed to await pause: {}", e))?;
            }
            "next" => {
                let op = session
                    .TrySkipNextAsync()
                    .map_err(|e| format!("Failed to skip next: {}", e))?;
                wait_for_async(op).map_err(|e| format!("Failed to await skip next: {}", e))?;
            }
            "previous" => {
                let op = session
                    .TrySkipPreviousAsync()
                    .map_err(|e| format!("Failed to skip previous: {}", e))?;
                wait_for_async(op).map_err(|e| format!("Failed to await skip previous: {}", e))?;
            }
            _ => return Err(format!("Unknown action: {}", action)),
        };

        Ok(())
    }
}

#[cfg(not(windows))]
mod windows_media {
    use super::*;

    pub fn get_media_sessions_sync() -> Result<Vec<MediaSession>, String> {
        Err("This feature is only available on Windows".to_string())
    }

    pub fn get_current_media_info_sync(_session_id: Option<String>) -> Result<MediaInfo, String> {
        Err("This feature is only available on Windows".to_string())
    }

    pub fn control_playback_sync(_action: &str) -> Result<(), String> {
        Err("This feature is only available on Windows".to_string())
    }
}

#[tauri::command]
async fn get_media_sessions() -> Result<Vec<MediaSession>, String> {
    tokio::task::spawn_blocking(|| windows_media::get_media_sessions_sync())
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn get_media_info(session_id: Option<String>) -> Result<MediaInfo, String> {
    tokio::task::spawn_blocking(move || windows_media::get_current_media_info_sync(session_id))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[tauri::command]
async fn control_playback(action: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || windows_media::control_playback_sync(&action))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_media_sessions,
            get_media_info,
            control_playback
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
