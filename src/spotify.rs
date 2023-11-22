use anyhow::{Context, Result};
use platform_dirs::AppDirs;
use rspotify::model::{PlayableItem, TimeRange};
use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use std::collections::HashMap;
use std::fs::create_dir;

pub struct Spotify {
    client: AuthCodeSpotify,
}

impl Spotify {
    pub fn new(scopes: &str) -> Result<Self> {
        let creds = Credentials::from_env().context("Could not find Client ID and Client Secret")?;
        let oauth = OAuth::from_env(scopes!(scopes)).context("Invalid Scopes")?;

        let app_dirs = AppDirs::new(Some("SpotiFetch"), true).context("Unable to get AppDirs")?;
        if !app_dirs.cache_dir.exists() {
            create_dir(&app_dirs.cache_dir)?;
        }
        let config = Config {
            token_cached: true,
            cache_path: app_dirs.cache_dir.join(".spotify_token_cache.json"),
            ..Default::default()
        };
        let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
        let url = spotify.get_authorize_url(false)?;
        spotify.prompt_for_token(&url)?;

        Ok(Spotify { client: spotify })
    }

    pub fn profile(&self) -> Result<HashMap<&str, String>> {
        // Username
        // Now playing
        // Recent
        // Top Track
        // Top Artist

        let mut results: HashMap<&str, String> = std::collections::HashMap::new();

        let username = self
            .client
            .current_user()
            .context("Could not fetch user details")?
            .display_name
            .context("Could not get username")?;
        results.insert("Username", username);

        let now_playing = self.now_playing().unwrap_or("NO CURRENTLY PLAYING TRACK".to_string());
        results.insert("Now Playing", now_playing);

        let recent_track = self
            .client
            .current_user_recently_played(Some(1), None)?
            .items
            .first()
            .context("Could not get recently played")?
            .track
            .clone();

        results.insert(
            "Recent",
            format!(
                "{} - {}",
                recent_track.name,
                recent_track
                    .artists
                    .first()
                    .context("Could not fetch recent track artist")?
                    .name
            ),
        );

        Ok(results)
    }

    pub fn top_artists(&self, term: TimeRange) -> Result<Vec<String>> {
        Ok(self
            .client
            .current_user_top_artists_manual(Some(term), Some(5), None)?
            .items
            .into_iter()
            .map(|artist| artist.name)
            .collect())
    }

    pub fn top_tracks(&self, term: TimeRange) -> Result<Vec<String>> {
        Ok(self
            .client
            .current_user_top_tracks_manual(Some(term), Some(5), None)?
            .items
            .into_iter()
            .map(|track| format!("{} - {}", track.name, track.artists.first().unwrap().name))
            .collect())
    }

    fn now_playing(&self) -> Result<String> {
        let err_message = "Could not fetch currently playing";

        let now_playing = self.client.current_playing(None, None::<Vec<_>>).context(err_message)?;

        match now_playing.context(err_message)?.item.context(err_message)? {
            PlayableItem::Track(track) => Ok(format!(
                "{} - {}",
                track.name,
                track
                    .artists
                    .first()
                    .context("Could not fetch artist for the track")?
                    .name
            )),
            PlayableItem::Episode(episode) => Ok(format!("{} - {}", episode.name, episode.show.name)),
        }
    }
}
