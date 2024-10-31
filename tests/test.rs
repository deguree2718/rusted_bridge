use rusted_bridge::*;

#[test]
fn find_devices() { // file_path: String
  let device_list = get_printers();
 	  println!("{:?}", device_list.unwrap());	
  let result = false;
//  for printer_name in &device_list {
//  	  result = print_file(("ZTC-GC420t--EPL-").to_string());
  //}
  assert!(!result);
}	
