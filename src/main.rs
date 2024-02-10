mod colors;
mod spotify;
use rspotify::model::TimeRange;
use kolorz::HexKolorize;

fn main() {
    let client =
        spotify::Spotify::new("user-read-currently-playing user-top-read user-read-recently-played user-read-private")
            .unwrap();

    let profile = client.profile().unwrap();
    println!("{:?}", profile.data);
    let image = colors::pigmnts(&profile.link, 6).unwrap();
    for color in image.iter() {
        println!("{}", color.hex().kolorize(&color.hex()));
    }

    let top_artists = client.top_artists(TimeRange::ShortTerm).unwrap();
    println!("{:?}", top_artists.data);
    let image = colors::pigmnts(&top_artists.link, 6).unwrap();
    for color in image.iter() {
        println!("{}", color.hex().kolorize(&color.hex()));
    }

    let top_tracks = client.top_tracks(TimeRange::MediumTerm).unwrap();
    println!("{:?}", top_tracks.data);
    let image = colors::pigmnts(&top_tracks.link, 6).unwrap();
    for color in image.iter() {
        println!("{}", color.hex().kolorize(&color.hex()));
    }
}
