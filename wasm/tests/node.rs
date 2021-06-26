extern crate image;
extern crate read_image_wasm;
extern crate wasm_bindgen_test;

mod utils;

mod read_image {
    use crate::utils::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn successful() {
        let result = read_image_wasm::read_image(generate_dummy_png());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TEST_PIXEL);
    }
    #[wasm_bindgen_test]
    fn failed() {
        let result = read_image_wasm::read_image(vec![]);
        assert!(result.is_err());
        let errmsg = result.unwrap_err().as_string();
        assert!(errmsg.is_some());
        assert_eq!(errmsg.unwrap(), "The image format could not be determined");
    }
}
