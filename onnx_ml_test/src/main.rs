fn main() {
    println!("Hello, world!");
}

// Stub out some tests
#[cfg(test)]
mod test {

    #[test]
    fn guess_when_handwritten_digit_5_jpg_then_label_5(){
        let path = "test_data/handwritten_5.jpg".to_string();

        // Do something here
        let res = 0;

        assert_eq!(res, 5);
    }

    #[test]
    fn guess_when_handwritten_digit_3_png_then_label_3(){
        let path = "test_data/handwritten_3.png".to_string();

        // Do something here
        let res = 0;

        assert_eq!(res, 3);
    }
}