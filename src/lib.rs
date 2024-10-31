use std::{fs::{File}, env};
use ipp::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//#[wasm_bindgen]
pub fn get_printers() -> Vec<String> {
  let mut printer_list = Vec::new();
  // printer_name.starts_with("ZTC") add to the array
  // printer_name.starts_with("file://") don't add to the array
  printer_list
}

//#[wasm_bindgen]
#[cfg(target_os = "linux")]
pub fn print_file(printer_name: String) -> bool { // , file_path: String) {
  let payload = IppPayload::new(File::open("/home/dev/Downloads/declaracao_horas.pdf").unwrap());
  let uri: Uri = String::from("http://localhost:631/printers/".to_owned() + &printer_name).parse().unwrap();
  let builder = IppOperationBuilder::print_job(uri.clone(), payload)
    .user_name(env::var("USER").unwrap_or_else(|_| "noname".to_owned()))
    .job_title("None")
    .attribute(IppAttribute::new("document-format", IppValue::MimeMediaType(String::from("application/octet-stream"))));

  let operation = builder.build();
  let client = IppClient::new(uri);
  let response = client.send(operation).unwrap();

  println!("IPP Status code: {}", response.header().status_code());

  println!("{:?}", response.attributes());

  // let attrs = response
  //    .attributes()
  //    .groups_of(DelimiterTag::JobAttributes)
  //    .flat_map(|g| g.attributes().values());

  return response.header().status_code().is_success();
  
}

//#[wasm_bindgen]
#[cfg(target_os = "windows")]
pub fn print_file(printer_name: String) {

}
