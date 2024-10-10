use std::env;
use processor_utils::{preprocess_image, build_model, guess};

fn main() {
    let args: Vec<String> = env::args().collect();
    let img_path = &args[0];

    let ready_img = preprocess_image(img_path);
    let model = build_model();
    let guess = guess(&model, ready_img);
}

mod processor_utils {
    use image::{imageops::{resize, FilterType}, open, DynamicImage, ImageBuffer, Luma};
    use ort::{GraphOptimizationLevel, Session};

    const MODEL_PATH: &str = "src/models/mnist_model.onnx";

    pub fn preprocess_image(img_path: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let img: DynamicImage = open(&img_path)
        .expect("File path did not contain an image.");

        let gray_scale = img.to_luma8();
        let ready_img = resize(&gray_scale, 28, 28, FilterType::CatmullRom);
    
        return ready_img;
    }

    pub fn build_model() -> Session {
        return Session::builder().unwrap()
            .with_optimization_level(GraphOptimizationLevel::Level3).unwrap()
            .with_intra_threads(4).unwrap()
            .commit_from_file(MODEL_PATH).unwrap();
    }

    pub fn guess(model: &Session, ready_img: ImageBuffer<Luma<u8>, Vec<u8>>) -> u32 {
        return 0;
    }
}

// Stub out some tests
#[cfg(test)]
mod test {
    use std::any::{Any, TypeId};
    use ort::Session;
    use super::*;

    #[test]
    fn preprocess_image_when_handwritten_digit_5_jpg_then_is_gray_scale_vector(){
        let img_path = "test_data/handwritten_5.jpg";

        let ready_img = preprocess_image(img_path);

        let (width, height) = ready_img.dimensions();
        assert_eq!(28, width);
        assert_eq!(28, height);
        for &value in ready_img.iter() {
            assert!(value >= 0 && value <= 255, "Value {} is out of bounds for grayscale!", value);
        }
    }

    #[test]
    fn preprocess_image_when_handwritten_digit_3_png_then_is_gray_scale_vector(){
        let img_path = "test_data/handwritten_3.png";

        let ready_img = preprocess_image(img_path);

        let (width, height) = ready_img.dimensions();
        assert_eq!(28, width);
        assert_eq!(28, height);
        for &value in ready_img.iter() {
            assert!(value >= 0 && value <= 255, "Value {} is out of bounds for grayscale!", value);
        }
    }

    #[test]
    fn build_model_when_using_mnist_onnx_model_then_return_session(){
        let model = build_model();

        assert_eq!(TypeId::of::<Session>(), model.type_id());
    }

    #[test]
    fn guess_when_handwritten_digit_5_jpg_then_label_5(){
        let img_path = "test_data/handwritten_5.jpg";
        let ready_img = preprocess_image(img_path);
        let session = build_model();

        let res = guess(&session, ready_img);

        assert_eq!(res, 5);
    }

    #[test]
    fn guess_when_handwritten_digit_3_png_then_label_3(){
        let img_path = "test_data/handwritten_3.png";
        let ready_img = preprocess_image(img_path);
        let session = build_model();

        let res = guess(&session, ready_img);

        assert_eq!(res, 3);
    }
}