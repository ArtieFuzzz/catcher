use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use exif::Reader;

type ExifData = Option<Vec<(String, String)>>;

pub fn parse(file: String) -> Result<ExifData, Box<dyn Error>> {
    let file_path = Path::new(&file);

    if !file_path.exists() {
      println!("File does not exist");
      return Ok(None)
    }

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let exif = Reader::new();

    let data = match exif.read_from_container(&mut reader) {
        Ok(data) => data,
        Err(_) => return Ok(None),
    };

    let fields = data
        .fields()
        .map(|f| (f.tag.to_string(), f.display_value().to_string()))
        .collect::<Vec<(String, String)>>();

    Ok(Some(fields))
}
