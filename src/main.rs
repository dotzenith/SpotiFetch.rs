mod colors;
mod spotify;
use rspotify::model::TimeRange;

fn main() {
    let client =
        spotify::Spotify::new("user-read-currently-playing user-top-read user-read-recently-played user-read-private")
            .unwrap();

    let profile = client.profile().unwrap();
    println!("{profile:?}");

    let top_artists = client.top_artists(TimeRange::ShortTerm).unwrap();
    println!("{top_artists:?}");

    let top_tracks = client.top_tracks(TimeRange::MediumTerm).unwrap();
    println!("{top_tracks:?}")
    // let image = colors::pigmnts("https://deeprunsentinelonline.org/wp-content/uploads/2021/02/cudi.jpg", 6).unwrap();
    //
    // for color in image.iter() {
    //     println!("{}", color.hex().kolorize(&color.hex()));
    // }
}
