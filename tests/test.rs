use rusted_bridge::*;
use std::fs;
use std::path::Path;
use winprint::ticket::{PredefinedMediaName, PrintTicket, PrintTicketBuilder, PrintCapabilities};

#[test]
fn find_devices() {
  let device_list = get_printers();
  println!("{:?}", device_list);
  // let mut result: bool = false;
  // for printer in device_list {
  //   let opt = PrintCapabilities::fetch(&printer).unwrap();
  //   let a4media = opt
  //     .page_media_size()
  //     .find(|x| x.as_predefined_name() == Some(PredefinedMediaName::ISOA4))
  //     .unwrap();
  //   let mut bd = PrintTicketBuilder::new(&printer).unwrap();
  //   bd.merge(a4media).unwrap();
  //   let tic = bd.build().unwrap();
  //   print(printer, Path::new("C:\\Users\\paulo\\Downloads\\b045218a-f1ac-4b5b-b17b-8eeae96c954a.pdf"), tic);
  //  }
  //  assert!(result);
}