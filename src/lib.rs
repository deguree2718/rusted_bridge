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
    println!("{:?}", printer);
    if printer.name == "Microsoft Print to PDF" {
      printer_list.push(String::from(printer.name));
    }
  }
  printer_list
}

#[wasm_bindgen]
#[cfg(target_os = "linux")]
pub fn print_file(printer_name: String) {
  let printer = printers::get_printer_by_name(&printer_name).unwrap();
  let payload = IppPayload::new(File::open("C:\\Users\\paulo\\Downloads\\teste1.pdf").unwrap());
  let uri: Uri = printer.location.parse().unwrap();
  let mut builder = IppOperationBuilder::print_job(uri.clone(), payload)
    .user_name(env::var("USER").unwrap_or_else(|_| "noname".to_owned()))
    .job_title("None");

  let operation = builder.build();
  let client = IppClient::new(uri);
  let response = client.send(operation)?;

  println!("IPP Status code: {}", response.header().status_code());

  let attrs = response
    .attributes()
    .groups_of(DelimiterTag::JobAttributes)
    .flat_map(|g| g.attributes().values());
}

#[wasm_bindgen]
#[cfg(target_os = "windows")]
pub fn print_file(printer_name: String) {

}