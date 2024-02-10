use anyhow::Result;
use image::GenericImageView;
use pigmnts::{
    color::{LAB, RGB},
    pigments_pixels, weights, Pixels,
};

pub fn pigmnts(image_path: &str, count: u8) -> Result<Vec<RGB>> {
    let mut res = reqwest::blocking::get(image_path)?;
    let mut buf: Vec<u8> = vec![];
    res.copy_to(&mut buf)?;
    let mut img = image::load_from_memory(buf.as_slice())?;

    img = img.resize(512, 512, image::imageops::FilterType::CatmullRom);

    let pixels: Pixels = img
        .pixels()
        .map(|(_, _, pix)| LAB::from_rgb(pix[0], pix[1], pix[2]))
        .collect();

    let weightfn = weights::resolve_mood(&weights::Mood::Dominant);
    let output = pigments_pixels(&pixels, count, weightfn, None);

    Ok(output.iter().map(|(color, _)| RGB::from(color)).collect())
}
