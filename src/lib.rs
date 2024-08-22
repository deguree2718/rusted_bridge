use std::{
	path::Path
// 	// process::Command
};
//use wasm_bindgen::prelude::wasm_bindgen;
// use pdfium_render::prelude::*;
// use printers;
use printers::printer::Printer;

pub fn get_printers() -> Vec<Printer> {
  let mut printer_list = Vec::new();
  // for printer in printers::get_printers() {
  //   if printer.name.contains("Microsoft") && printer.name != "Fax" && printer.name.to_lowercase() == printer.driver_name.to_lowercase() {
  //     printer_list.push(printer);
  //   }
  // }
  for printer in printers::get_printers() {
    if printer.name != "Fax" && !printer.name.contains("Microsoft") {
      printer_list.push(printer);
    }
  }
  printer_list
}


pub fn print(p_printer: &Printer, file: &Path) {
	p_printer.print_file(file.to_str().expect("couldn't parse"), None).expect("couldn't print");
}
