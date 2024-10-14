use rusted_bridge::{print_file, get_printers};
use std::fs;
use std::fs::read;
use std::path::Path;
#[test]
fn find_devices() { // file_path: String
  let device_list = get_printers();
  println!("{:?}", device_list);
  for printer in device_list {
  	println!("{}", printer);
    let result = crate::print_file(printer);
	break;
   }
}
