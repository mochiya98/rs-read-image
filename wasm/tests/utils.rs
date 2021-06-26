
pub const TEST_PIXEL: &'static [u8; 4] = &[12, 34, 56, 78];

pub fn generate_dummy_png() -> Vec<u8> {
    let mut imgbuf = image::ImageBuffer::new(1, 1);
    let pixel = imgbuf.get_pixel_mut(0, 0);
    let image::Rgba(_) = *pixel;
    *pixel = image::Rgba(*TEST_PIXEL);
    let mut buf: Vec<u8> = Vec::new();
    let encoder = image::png::PngEncoder::new_with_quality(
        &mut buf,
        image::png::CompressionType::Fast,
        image::png::FilterType::NoFilter,
    );
    assert!(encoder
        .encode(&imgbuf.into_raw(), 1, 1, image::ColorType::Rgba8)
        .is_ok());
    buf
}
