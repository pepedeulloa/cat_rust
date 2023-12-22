use cat_rust::config::{ Cli, get_args, open, run_file } ;
fn main() {
 let Cli{files, line_pos, line_pos_nonblank, line_end } = get_args().unwrap();

 for f in files {
  let reader = open(f).unwrap();
  run_file(reader, line_pos, line_pos_nonblank, line_end);
 }

}
