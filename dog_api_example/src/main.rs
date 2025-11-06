use serde::Deserialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum DogAppError {
    Api(String),
    Network(String),
    Parse(String),
    Image(String),
}

impl fmt::Display for DogAppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DogAppError::Api(e) => write!(f, "API error: {}", e),
            DogAppError::Network(e) => write!(f, "Network error: {}", e),
            DogAppError::Parse(e) => write!(f, "Parse error: {}", e),
            DogAppError::Image(e) => write!(f, "Image error: {}", e),
        }
    }
}

impl Error for DogAppError {}

fn fetch_random_dog_image() -> Result<DogImage, DogAppError> {
    let url = "https://dog.ceo/api/breeds/image/random";
    let response = ureq::get(url)
        .call()
        .map_err(|e| DogAppError::Network(format!("{}", e)))?;

    if response.status() != 200 {
        return Err(DogAppError::Api(format!(
            "HTTP error: {}",
            response.status()
        )));
    }

    response
        .into_json::<DogImage>()
        .map_err(|e| DogAppError::Parse(format!("{}", e)))
}

fn display_image(url: &str) -> Result<(), DogAppError> {
    use std::io::Read;
    use tempfile::NamedTempFile;

    let tmp_file = NamedTempFile::new().map_err(|e| DogAppError::Image(e.to_string()))?;

    let response = ureq::get(url)
        .call()
        .map_err(|e| DogAppError::Network(format!("{}", e)))?
        .into_reader();

    // Read at most 5 MB of data to prevent excessive memory use
    let mut buffer = Vec::new();
    response
        .take(5_000_000)
        .read_to_end(&mut buffer)
        .map_err(|e| DogAppError::Image(e.to_string()))?;

    std::fs::write(tmp_file.path(), &buffer).map_err(|e| DogAppError::Image(e.to_string()))?;

    // Try to display using viuer (ASCII fallback)
    if let Err(e) = viuer::print_from_file(tmp_file.path(), &viuer::Config::default()) {
        println!("(Could not display image: {})", e);
        println!("🖼️ Image URL: {}", url);
    }

    Ok(())
}
fn main() -> Result<(), DogAppError> {
    println!("🐶 Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=3 {
        println!("Fetching random dog image #{}", i);

        match fetch_random_dog_image() {
            Ok(dog_image) => {
                println!("✅ Success!");
                println!("📊 Status: {}", dog_image.status);
                if let Err(e) = display_image(&dog_image.message) {
                    eprintln!("⚠️ Failed to show image: {}", e);
                }
            }
            Err(e) => eprintln!("❌ Error: {}", e),
        }

        println!();
    }

    Ok(())
}
