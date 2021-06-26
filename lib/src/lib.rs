pub type ReadImageResult = Result<Vec<u8>, image::ImageError>;

pub fn read_image(data: Vec<u8>) -> ReadImageResult {
    let img = image::load_from_memory(&data)?;
    Ok(img.to_rgba8().to_vec())
}
