use std::{fs::{File}, env};
use wasm_bindgen::prelude::wasm_bindgen;
use printers;
use ipp::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_printers() -> Vec<String> {
  let mut printer_list = Vec::new();
  for printer in printers::get_printers() {
    // if printer.name == "Microsoft Print to PDF" {
      printer_list.push(printer.name);
    // }
  }
  printer_list
}

#[wasm_bindgen]
#[cfg(target_os = "linux")]
pub fn print_file(printer_name: String) {
  let printer = printers::get_printer_by_name(&printer_name).unwrap();
  println!("{:?}", printer);
  let payload = IppPayload::new(File::open("/home/paulo-grechi/Downloads/GRIMOIRE_DIGITAL.pdf").unwrap());
  let uri: Uri = printer.uri.parse().unwrap();
  println!("{:?}", uri);
  let builder = IppOperationBuilder::print_job(uri.clone(), payload)
    .user_name(env::var("USER").unwrap_or_else(|_| "noname".to_owned()))
    .job_title("None");

  let operation = builder.build();
  let client = IppClient::new(uri);
  let response = client.send(operation).unwrap();

  println!("IPP Status code: {}", response.header().status_code());

  // let attrs = response
  //   .attributes()
  //   .groups_of(DelimiterTag::JobAttributes)
  //   .flat_map(|g| g.attributes().values());
}

#[wasm_bindgen]
#[cfg(target_os = "windows")]
pub fn print_file(printer_name: String) {

}
