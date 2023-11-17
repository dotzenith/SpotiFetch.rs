mod colors;

fn main() {
    let image = colors::pigmnts("https://upload.wikimedia.org/wikipedia/en/4/44/Kids_See_Ghosts_Cover.png", 6).unwrap();

    for color in image.iter() {
        println!("{}", color.hex());
    }
}
