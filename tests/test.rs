use rusted_bridge::{print_file, get_printers};
use std::fs;
use std::fs::read;
use std::path::Path;

#[test]
fn find_devices() { // file_path: String
  let device_list = get_printers();
  let result = true;
  for printer_name in &device_list {
//  	  result = print_file((&printer_name).to_string());
  }
  assert!(result);
}
