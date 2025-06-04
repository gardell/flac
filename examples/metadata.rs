extern crate clap;
extern crate flac;
extern crate rustc_serialize;

#[macro_use]
mod commands;

use commands::{streaminfo, comments, seektable, picture, list_block_names};

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  List {
    #[arg(short, long)]
    filename: String,
  },
  StreamInfo(streaminfo::Arguments),
  Comments(comments::Arguments),
  SeekTable(seektable::Arguments),
  Picture(picture::Arguments),
}

fn main() {
  let args = Arguments::parse();

  match args.command {
    Command::List { filename } => list_block_names(&filename),
    Command::StreamInfo(stream_info) => streaminfo::run(&stream_info),
    Command::Comments(comments) => comments::run(&comments),
    Command::SeekTable(seek_table) => seektable::run(&seek_table),
    Command::Picture(picture) => picture::run(&picture),
  }
}
