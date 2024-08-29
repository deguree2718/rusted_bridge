use rusted_bridge::{print_file, get_printers};
use std::fs;
use std::fs::read;
use std::path::Path;
#[test]
fn find_devices() {
  let device_list = get_printers();
  println!("{:?}", device_list);
  for printer in device_list {
    // let file = read(Path::new("C:\\Users\\paulo\\Downloads\\teste1.pdf")).unwrap();
    let result = print_file(printer);
   }
}