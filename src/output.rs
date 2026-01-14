use crate::colors;
use kolorz::{HexKolorize, Kolor, KoloredText};
use rand::Rng;

pub struct LogoFields {
    pub border: KoloredText,
    pub bar: KoloredText,
    pub middle: (KoloredText, KoloredText, KoloredText),
}

pub fn kolorz_output(colorscheme: Kolor, lines: Vec<String>, profile: bool, random_border: bool) {
    let middle = (
        colorscheme.purple("_..**.._"),
        colorscheme.blue("_..**.._"),
        colorscheme.orange("_..**.._"),
    );

    let mut logo_fields = LogoFields {
        border: colorscheme.green(";;"),
        bar: colorscheme.green("______"),
        middle,
    };

    if random_border {
        let mut rng = rand::thread_rng();
        let color_num = rng.gen_range(0..=6) as usize;

        logo_fields.border = colorscheme.numbered(";;", color_num).expect("invalid color number");
        logo_fields.bar = colorscheme.numbered("______", color_num).expect("invalid color number");
    }

    if profile {
        let output_lines: [String; 5] = [
            format!("{}            {}", colorscheme.purple("USER"), lines[0]),
            format!("{}     {}", colorscheme.blue("NOW PLAYING"), lines[1]),
            format!("{}    {}", colorscheme.green("RECENT TRACK"), lines[2]),
            format!("{}       {}", colorscheme.orange("TOP TRACK"), lines[3]),
            format!("{}      {}", colorscheme.yellow("TOP ARTIST"), lines[4]),
        ];
        print_art(logo_fields, output_lines);
    } else {
        let output_lines: [String; 5] = [
            format!("{}", colorscheme.purple(&lines[0])),
            format!("{}", colorscheme.blue(&lines[1])),
            format!("{}", colorscheme.green(&lines[2])),
            format!("{}", colorscheme.orange(&lines[3])),
            format!("{}", colorscheme.yellow(&lines[4])),
        ];
        print_art(logo_fields, output_lines);
    }
}

pub fn custom_output(img_url: String, lines: Vec<String>, profile: bool) {
    let image_colors: Vec<String> = colors::pigmnts(&img_url, 6)
        .unwrap()
        .into_iter()
        .map(|color| color.hex())
        .collect();

    let middle = (
        "_..**.._".kolorize(&image_colors[1]).expect("invalid hex color"),
        "_..**.._".kolorize(&image_colors[2]).expect("invalid hex color"),
        "_..**.._".kolorize(&image_colors[3]).expect("invalid hex color"),
    );
    let logo_fields = LogoFields {
        border: ";;".kolorize(&image_colors[0]).expect("invalid hex color"),
        bar: "______".kolorize(&image_colors[0]).expect("invalid hex color"),
        middle,
    };

    if profile {
        let output_lines: [String; 5] = [
            format!(
                "{}            {}",
                "USER".kolorize(&image_colors[1]).expect("invalid hex color"),
                lines[0]
            ),
            format!(
                "{}     {}",
                "NOW PLAYING".kolorize(&image_colors[2]).expect("invalid hex color"),
                lines[1]
            ),
            format!(
                "{}    {}",
                "RECENT TRACK".kolorize(&image_colors[3]).expect("invalid hex color"),
                lines[2]
            ),
            format!(
                "{}       {}",
                "TOP TRACK".kolorize(&image_colors[4]).expect("invalid hex color"),
                lines[3]
            ),
            format!(
                "{}      {}",
                "TOP ARTIST".kolorize(&image_colors[5]).expect("invalid hex color"),
                lines[4]
            ),
        ];
        print_art(logo_fields, output_lines);
    } else {
        let output_lines: [String; 5] = [
            format!("{}", &lines[0].kolorize(&image_colors[1]).expect("invalid hex color")),
            format!("{}", &lines[1].kolorize(&image_colors[2]).expect("invalid hex color")),
            format!("{}", &lines[2].kolorize(&image_colors[3]).expect("invalid hex color")),
            format!("{}", &lines[3].kolorize(&image_colors[4]).expect("invalid hex color")),
            format!("{}", &lines[4].kolorize(&image_colors[5]).expect("invalid hex color")),
        ];
        print_art(logo_fields, output_lines);
    }
}

#[rustfmt::skip]
pub fn print_art(logo_fields: LogoFields, lines: [String; 5]) {
    println!("      {}", logo_fields.bar);
    println!("   {}        {}", logo_fields.border, logo_fields.border);
    println!(" {}            {}      {}", logo_fields.border, logo_fields.border, lines[0]);
    println!("{}   {}   {}     {}", logo_fields.border, logo_fields.middle.0, logo_fields.border, lines[1]);
    println!("{}   {}   {}     {}", logo_fields.border, logo_fields.middle.1, logo_fields.border, lines[2]);
    println!("{}   {}   {}     {}", logo_fields.border, logo_fields.middle.2, logo_fields.border, lines[3]);
    println!(" {}            {}      {}", logo_fields.border, logo_fields.border, lines[4]);
    println!("   {}        {}", logo_fields.border, logo_fields.border);
    println!("      {}", logo_fields.bar);
}
