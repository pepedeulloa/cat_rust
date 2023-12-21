use std::io::BufRead;

use cat_rust::config::{ Cli, get_args, open, output } ;
fn main() {
 let Cli{files, line_pos, line_pos_nonblank, line_end } = get_args().unwrap();

 for f in files {
  let reader = open(f).unwrap();

  for (line_number, line) in reader.lines().enumerate() {
   output(line.unwrap(), line_number, line_pos, line_pos_nonblank, line_end)
  }

 }

}
