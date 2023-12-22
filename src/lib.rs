pub mod config {
 use std::{path::PathBuf, io::{BufReader, BufRead}, error::Error, fs::File};

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

 pub fn run_file(reader: BufReader<File>, line_pos: bool, line_pos_nonblank: bool, line_end: bool) {
  // Counter for non blank lines option
  let mut num = 0;

  for (number, line) in reader.lines().enumerate(){
   if !line.as_ref().unwrap().is_empty() && line_pos_nonblank {
    num += 1;
    output(line.unwrap(), num, true, line_end);
   } else if line_pos {
     output(line.unwrap(), number, true, line_end);
   } else if line_end {
    output(line.unwrap(), 0, false, line_end);
   } else {
    output(line.unwrap(), 0, line_pos, line_end)
   }
  }
 }

 pub fn output(line: String, line_number: usize, enum_lines: bool, line_end: bool) {
  
  if line_end {
   println!("{}{}", line,'$')
  } else if enum_lines {
   println!("{}\t{}", line_number, line);
  }
  else {
   println!("{}", line);
  }
 }

}
