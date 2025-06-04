use std::fs::File;

use clap::Args;
use flac::{Stream, StreamProducer, StreamReader};

#[derive(Args)]
pub struct Arguments {
  #[arg(short, long, required = true)]
  filename: String,
  #[arg(long)]
  block_size: bool,
  #[arg(long)]
  frame_size: bool,
  #[arg(long)]
  sample_rate: bool,
  #[arg(long)]
  channels: bool,
  #[arg(long)]
  bits_per_sample: bool,
  #[arg(long)]
  total_samples: bool,
  #[arg(long)]
  md5: bool,
}

fn print_stream_info<P>(stream: &Stream<P>, args: &Arguments)
 where P: StreamProducer {
  let info     = stream.info();
  let no_flags = (args.block_size      || args.frame_size    ||
                  args.sample_rate     || args.channels      ||
                  args.bits_per_sample || args.total_samples ||
                  args.md5) == false;

  if no_flags || args.block_size {
    let block_size_str = if info.is_fixed_block_size() {
      format!("{} samples", info.max_block_size)
    } else {
      format!("{} - {} samples", info.min_block_size, info.max_block_size)
    };

    format_print!("{}{}", "Block size: ", block_size_str, no_flags);
  }

  if no_flags || args.frame_size {
    println!("Frame size: {} - {} bytes", info.min_frame_size,
                                          info.max_frame_size);
  }

  if no_flags || args.sample_rate {
    format_print!("{}{} Hz", "Sample rate: ", info.sample_rate, no_flags);
  }

  if no_flags || args.channels {
    format_print!("{}{}", "Number of channels: ", info.channels, no_flags);
  }

  if no_flags || args.bits_per_sample {
    format_print!("{}{}", "Bits per samples: ", info.bits_per_sample,
                                                no_flags);
  }

  if no_flags || args.total_samples {
    format_print!("{}{}", "Total samples: ", info.total_samples, no_flags);
  }

  if no_flags || args.md5 {
    let mut md5  = String::with_capacity(32);

    for byte in &info.md5_sum {
      let hex = format!("{:02x}", byte);

      md5.push_str(&hex);
    }

    format_print!("{}{}", "MD5 sum: ", md5, no_flags);
  }
}

pub fn run(args: &Arguments) {
  let stream = StreamReader::<File>::from_file(&args.filename)
                 .expect("Couldn't parse file");

  print_stream_info(&stream, &args);
}
