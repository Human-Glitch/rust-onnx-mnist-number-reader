# zelis_ml_code_test

Created a rust application that guesses a number from a handwritten digit image, leveraging ORT (Onyx Runtime) with MNIST model (v1.2).
* Utilized Test Driven Development to ensure integrity of app while developing the solution
* Runs in approximately 50ms (depends on the image)
* Supports JPG/JPEG/WEBP
* Does not support with PNG/HEIC/SVG/TIFF 

Optimizations:
* Resize methodology (FilterType::Nearest) to 28x28 image
* Resizing before converting to grayscale image via image lib
* Leveraged ndarray for vector math
* Normalized grayscale vector so MNIST model can converge faster

Output:

<img width="809" alt="image" src="https://github.com/user-attachments/assets/0722ac84-5d2a-4494-901a-7742a9f037d9">

Example commands: 

cargo run -- "test_data/handwritten_5.jpg"

cargo run --release  -- "test_data/handwritten_5.jpg"
