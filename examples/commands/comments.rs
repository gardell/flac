use std::io::{self, Write};
use std::fs::File;

use clap::Args;

use flac::StreamReader;
use flac::metadata::{self, VorbisComment};

#[derive(Args)]
pub struct Arguments {
  #[arg(short, long)]
  filename: String,
  #[arg(short, long)]
  vendor: bool,
  #[arg(short, long, value_name = "NAME")]
  name: Option<String>,
  #[arg(short, long, value_name = "FILE")]
  export: Option<String>,
}


fn print_vorbis_comments(vorbis_comment: &VorbisComment, args: &Arguments) {
  let no_flags  = (args.vendor || args.name.is_some()) == false;

  if no_flags || args.vendor {
    format_print!("{}{}", "Vendor string: ", vorbis_comment.vendor_string,
                                             no_flags);
  }

  if no_flags {
    let mut index = 1;

    println!("Number of Comments: {}", vorbis_comment.comments.len());

    for (name, value) in &vorbis_comment.comments {
      println!("  {}: \"{}\" = {}", index, name, value);

      index += 1;
    }
  } else {
    if let Some(ref name) = args.name {
      let error_str = format!("Couldn't find tag name: \"{}\"", name);
      let result    = vorbis_comment.comments.get(name).unwrap_or(&error_str);

      println!("{}", result)
    }
  }
}

fn export_vorbis_comments(vorbis_comment: &VorbisComment, filename: &str)
                          -> io::Result<()> {
  let mut file = try!(File::create(filename));

  for (name, value) in &vorbis_comment.comments {
    try!(write!(file, "{}={}\n", name, value));
  }

  Ok(())
}

pub fn run(args: &Arguments) {
  let stream = StreamReader::<File>::from_file(&args.filename)
                 .expect("Couldn't parse file");

  for meta in stream.metadata() {
    match meta.data {
      metadata::Data::VorbisComment(ref v) => {
        if let Some(ref filename) = args.export {
          export_vorbis_comments(v, filename)
            .expect("couldn't write to file")
        } else {
          print_vorbis_comments(v, &args)
        }
      }
      _                                    => continue,
    }
  }
}
