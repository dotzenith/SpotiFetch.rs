use anyhow::{Context, Result};
use platform_dirs::AppDirs;
use rspotify::prelude::OAuthClient;
use rspotify::{scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use rspotify::model::PlayableItem;
use std::fs::create_dir;

pub struct Spotify {
    client: AuthCodeSpotify,
}

impl Spotify {
    pub fn new(scopes: &str) -> Result<Self> {
        let creds =
            Credentials::from_env().context("Could not find Client ID and Client Secret")?;
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

    pub fn currently_playing(&self) -> Result<()> {
        let playing = self
            .client
            .current_playing(None, None::<Vec<_>>)
            .context("Could not fetch currently playing")?;

        match playing.unwrap().item.unwrap() {
            PlayableItem::Track(track) => {
                println!("Track name: {}, by {}", track.name, track.artists.first().unwrap().name);
            },
            PlayableItem::Episode(_) => println!("It's an episode!"),
        }
        Ok(())
    }
}
