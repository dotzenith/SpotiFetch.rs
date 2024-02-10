use crate::ArtType;
use crate::HexKolorize;

#[rustfmt::skip]
pub fn print_art(art_type: ArtType, one: &str, two: &str, three: &str, four: &str, five: &str) {
    match art_type {
        ArtType::ColorScheme(kolors) => {
            let border = kolors.green(";;");
            let bar = kolors.green("______");
            let middle = "_..**.._";

            println!("      {}", bar);
            println!("   {}        {}", border, border);
            println!(" {}            {}      {}", border, border, one);
            println!("{}   {}   {}     {}", border, kolors.purple(middle), border, two);
            println!("{}   {}   {}     {}", border, kolors.blue(middle), border, three);
            println!("{}   {}   {}     {}", border, kolors.orange(middle), border, four);
            println!(" {}            {}      {}", border, border, five);
            println!("   {}        {}", border, border);
            println!("      {}", bar);
        },
        ArtType::Artwork(colors) => {
            let border = ";;".kolorize(&colors[0]);
            let bar = "______".kolorize(&colors[0]);
            let middle = "_..**.._";

            println!("      {}", bar);
            println!("   {}        {}", border, border);
            println!(" {}            {}      {}", border, border, one);
            println!("{}   {}   {}     {}", border, middle.kolorize(&colors[1]), border, two);
            println!("{}   {}   {}     {}", border, middle.kolorize(&colors[2]), border, three);
            println!("{}   {}   {}     {}", border, middle.kolorize(&colors[3]), border, four);
            println!(" {}            {}      {}", border, border, five);
            println!("   {}        {}", border, border);
            println!("      {}", bar);
        }
    }
}
