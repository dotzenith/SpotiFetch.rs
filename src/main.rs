mod colors;
mod spotify;

fn main() {
    let client = spotify::Spotify::new(
        "user-read-currently-playing user-top-read user-read-recently-played user-read-private",
    ).unwrap();

    let artists = client.currently_playing().unwrap();
    // let image = colors::pigmnts("https://deeprunsentinelonline.org/wp-content/uploads/2021/02/cudi.jpg", 6).unwrap();
    //
    // for color in image.iter() {
    //     println!("{}", color.hex().kolorize(&color.hex()));
    // }
}
