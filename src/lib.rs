pub mod config {
 use std::{path::PathBuf, io::BufReader, error::Error, fs::File};

 use clap::Parser;
 
 #[derive(Parser,Debug)]
 #[command(author, version, about, long_about = None)]
 pub struct Cli {
  /// Path of the file to read
  #[arg(id="FILE",required=true)]
  pub files: Vec<PathBuf>,

  /// Number of the line
  #[arg(short='n')]
  pub line_pos: bool,

  /// Number the non-blank output lines
  #[arg(short='b')]
  pub line_pos_nonblank: bool,

  /// Show end of the line with '$'
  #[arg(short='e')]
  pub line_end: bool,
 
 }

 pub fn get_args() -> Result<Cli, Box<dyn Error>> {

  let cli = Cli::parse();

  Ok(cli)
 }

 pub fn open(filename: PathBuf) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
  let file = File::open(filename)?;
  let reader = BufReader::new(file);
  Ok(reader)
 }

 pub fn output(line: String, line_number: usize, enum_lines: bool, enum_nonblank_lines: bool, line_end: bool) {
  if enum_lines {
   println!("{}\t{}", line_number + 1, line);
  } else if enum_nonblank_lines && !line.is_empty() {
   println!("{}\t{}", line_number + 1, line);
  } else if line_end {
   println!("{}{}", line,'$')
  } else {
   println!("{}", line);
  }
 }

}
