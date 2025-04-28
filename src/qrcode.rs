
mod tests {
    use rqrr::PreparedImage;

    #[test]
    fn test_parse_qr_from_image() {
        let img = image::open("./src/assets/example.png").expect("Failed to open image");
        let mut img = PreparedImage::prepare(img.to_luma8());
        let grids = img.detect_grids();

        assert!(grids.len() > 0);

        // decode the grids
        let qr = grids[0].decode();
        println!("{:?}", qr);
    }
}
