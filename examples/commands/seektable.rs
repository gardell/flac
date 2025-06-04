use std::fs::File;

use clap::Args;

use flac::StreamReader;
use flac::metadata::{self, SeekPoint};

#[derive(Args)]
pub struct Arguments {
  #[arg(short, long, value_name = "FILE")]
  filename: String,
}

fn print_seek_table(seek_points: &[SeekPoint]) {
  let mut count = 0;

  println!("Number of Seek Points: {}", seek_points.len());

  for seek_point in seek_points {
    println!("Seek Point #{}", count);
    println!("  Sample number: {}", seek_point.sample_number);
    println!("  Stream offset: {}", seek_point.stream_offset);
    println!("  Frame samples: {}", seek_point.frame_samples);
    count += 1;
  }
}

pub fn run(args: &Arguments) {
  let stream = StreamReader::<File>::from_file(&args.filename)
                 .expect("Couldn't parse file");

  for meta in stream.metadata() {
    match meta.data {
      metadata::Data::SeekTable(ref s) => print_seek_table(s),
      _                                => continue,
    }
  }
}
