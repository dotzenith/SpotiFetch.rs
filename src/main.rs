mod colors;
mod output;
mod spotify;
use clap::{arg, command, Command};
use std::process::exit;

use kolorz::Kolor;
use rspotify::model::TimeRange;

fn main() {
    let spotify_client = match spotify::Spotify::new(
        "user-read-currently-playing user-top-read user-read-recently-played user-read-private",
    ) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Unable to establish connection to the Spotify API: {e}");
            exit(1)
        }
    };

    let matches = command!()
        .subcommand_required(true)
        .arg(
            arg!(--colorscheme <STR>)
                .short('c')
                .value_parser(clap::value_parser!(String))
                .default_value("catppuccin mocha")
                .default_missing_value("catppuccin mocha")
                .require_equals(false)
                .global(true)
                .help("See the readme for available colorschemes"),
        )
        .arg(
            arg!(--term <STR>)
                .short('t')
                .value_parser(clap::value_parser!(String))
                .default_value("mid")
                .default_missing_value("mid")
                .require_equals(false)
                .global(true)
                .help("The timeline for the top tracks/artists; short, mid, long"),
        )
        .arg(
            arg!(--art)
                .short('a')
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Use cover art for album or artist to generate a colorscheme"),
        )
        .arg(
            arg!(--random)
                .short('r')
                .action(clap::ArgAction::SetTrue)
                .global(true)
                .help("Use a random color as the outline"),
        )
        .subcommand(Command::new("profile").about("Fetch general information about user profile"))
        .subcommand(Command::new("top-tracks").about("Fetch the top tracks for a given term"))
        .subcommand(Command::new("top-artists").about("Fetch the top artists for a given term"))
        .get_matches();

    match matches.subcommand() {
        Some(("profile", _)) => {
            let term = match matches.get_one::<String>("term").unwrap().as_str() {
                "short" => TimeRange::ShortTerm,
                "mid" => TimeRange::MediumTerm,
                "long" => TimeRange::LongTerm,
                _ => TimeRange::MediumTerm,
            };
            let colorscheme = matches.get_one::<String>("colorscheme").unwrap();

            let profile = spotify_client.profile().unwrap();
            let top_tracks = spotify_client.top_tracks(term).unwrap();
            let top_artists = spotify_client.top_artists(term).unwrap();

            let mut lines: Vec<String> = vec![];
            lines.push(profile.data["Username"].clone());
            lines.push(profile.data["Now Playing"].clone());
            lines.push(profile.data["Recent"].clone());
            lines.push(top_tracks.data[0].clone());
            lines.push(top_artists.data[0].clone());

            if matches.get_flag("art") {
                output::custom_output(profile.link, lines, true)
            } else {
                output::kolorz_output(
                    Kolor::new(colorscheme.as_str()),
                    lines,
                    true,
                    matches.get_flag("random"),
                )
            }
        }
        Some(("top-tracks", _)) => {
            let term = match matches.get_one::<String>("term").unwrap().as_str() {
                "short" => TimeRange::ShortTerm,
                "mid" => TimeRange::MediumTerm,
                "long" => TimeRange::LongTerm,
                _ => TimeRange::MediumTerm,
            };
            let colorscheme = matches.get_one::<String>("colorscheme").unwrap();
            let top_tracks = spotify_client.top_tracks(term).unwrap();

            if matches.get_flag("art") {
                output::custom_output(top_tracks.link, top_tracks.data, false)
            } else {
                output::kolorz_output(
                    Kolor::new(colorscheme.as_str()),
                    top_tracks.data,
                    false,
                    matches.get_flag("random"),
                )
            }
        }
        Some(("top-artists", _)) => {
            let term = match matches.get_one::<String>("term").unwrap().as_str() {
                "short" => TimeRange::ShortTerm,
                "mid" => TimeRange::MediumTerm,
                "long" => TimeRange::LongTerm,
                _ => TimeRange::MediumTerm,
            };
            let colorscheme = matches.get_one::<String>("colorscheme").unwrap();
            let top_artists = spotify_client.top_artists(term).unwrap();

            if matches.get_flag("art") {
                output::custom_output(top_artists.link, top_artists.data, false)
            } else {
                output::kolorz_output(
                    Kolor::new(colorscheme.as_str()),
                    top_artists.data,
                    false,
                    matches.get_flag("random"),
                )
            }
        }
        _ => unreachable!(),
    }
}
