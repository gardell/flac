use std::io::{self, Write};
use std::fs::File;

use clap::Args;

use flac::stream::StreamReader;
use flac::metadata::{self, Picture};

#[derive(Args)]
pub struct Arguments {
  #[arg(short, long, required = true, value_name = "FILE")]
  filename: String,
  #[arg(short, long, value_name = "FILE")]
  export: Option<String>,
  #[arg(short, long)]
  index: Option<usize>,
}

fn export_picture(picture: &Picture, filename: &str) -> io::Result<()> {
  File::create(filename).and_then(|mut file| file.write_all(&picture.data))
}

fn print_picture(picture: &Picture) {
  println!("Picture type: {}", picture.picture_type);
  println!("Mime type: \"{}\"", picture.mime_type);
  println!("Description: \"{}\"", picture.description);
  println!("Dimensions: {}x{}", picture.width, picture.height);
  println!("Depth: {}", picture.depth);
  println!("Colors: {}", picture.colors);
  println!("Data length: {} bytes", picture.data.len());
}

pub fn run(args: &Arguments) {
  let stream = StreamReader::<File>::from_file(&args.filename)
                 .expect("Couldn't parse file");

  let mut index = 0;
  let end_index = args.index.unwrap_or(0);

  for meta in stream.metadata() {
    match meta.data {
      metadata::Data::Picture(ref p) => {
        if index < end_index {
          index += 1;

          continue;
        }

        if let Some(ref filename) = args.export {
          export_picture(p, filename).expect("couldn't write to file");

          break;
        } else {
          print_picture(p);
        }
      }
       _                             => continue,
    }
  }
}
