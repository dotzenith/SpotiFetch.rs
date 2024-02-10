mod colors;
mod print;
mod spotify;
use kolorz::{HexKolorize, Kolor};
use rspotify::model::TimeRange;

pub enum ArtType {
    ColorScheme(Kolor),
    Artwork(Vec<String>),
}

fn main() {
    let client =
        spotify::Spotify::new("user-read-currently-playing user-top-read user-read-recently-played user-read-private")
            .unwrap();


    let profile = client.profile().unwrap();
    let top_tracks = client.top_tracks(TimeRange::MediumTerm).unwrap();
    let top_artists = client.top_artists(TimeRange::ShortTerm).unwrap();

    let kolorscheme = Kolor::new("catppuccin mocha");
    let one = format!("{}            {}", kolorscheme.purple("USER"), profile.data["Username"]);
    let two = format!("{}     {}", kolorscheme.blue("NOW PLAYING"), profile.data["Now Playing"]);
    let three = format!("{}    {}", kolorscheme.green("RECENT TRACK"), profile.data["Recent"]);
    let four = format!("{}       {}", kolorscheme.orange("TOP TRACK"), top_tracks.data[0]);
    let five = format!("{}      {}", kolorscheme.yellow("TOP ARTIST"), top_artists.data[0]);
    print::print_art(ArtType::ColorScheme(kolorscheme), &one, &two, &three, &four, &five);

    let image_colors: Vec<String> = colors::pigmnts(&profile.link, 6)
        .unwrap()
        .into_iter()
        .map(|color| color.hex())
        .collect();
    let one = format!("{}            {}", "USER".kolorize(&image_colors[1]), profile.data["Username"]);
    let two = format!("{}     {}", "NOW PLAYING".kolorize(&image_colors[2]), profile.data["Now Playing"]);
    let three = format!("{}    {}", "RECENT TRACK".kolorize(&image_colors[3]), profile.data["Recent"]);
    let four = format!("{}       {}", "TOP TRACK".kolorize(&image_colors[4]), top_tracks.data[0]);
    let five = format!("{}      {}", "TOP ARTIST".kolorize(&image_colors[5]), top_artists.data[0]);
    print::print_art(ArtType::Artwork(image_colors), &one, &two, &three, &four, &five);
}
