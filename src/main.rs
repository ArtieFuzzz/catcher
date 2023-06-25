mod reader;

use std::error::Error;

use clap::Parser;

#[derive(Parser)]
struct Arguments {
  file: String,
  clean: Option<bool>
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    let exif_data = reader::parse(arguments.file)?;

    if exif_data.is_none() {
      println!("No EXIF data was found");
      return Ok(())
    }

    let exif_data = exif_data.unwrap();

    for (tag, value) in exif_data.iter() {
      println!("{tag} : {value}")
    }

  Ok(())
}
