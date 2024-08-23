use std::{
	path::Path
// 	// process::Command
};
//use wasm_bindgen::prelude::wasm_bindgen;
// use pdfium_render::prelude::*;
// use printers;
// use printers::printer::Printer;
use winprint::{
  printer::{
    FilePrinter,
    PrinterDevice,
    XpsPrinter
  },
  ticket::{
    PrintCapabilities,
    PrintTicket,
    PrintTicketBuilder
  }
};
use winprint::printer::PdfiumPrinter;

pub fn get_printers() -> Vec<PrinterDevice> {
  let mut printer_list = Vec::new();
  for printer in PrinterDevice::all().expect("err") {
    if printer.name() == "Microsoft Print to PDF" {
      printer_list.push(printer);
    }
  }
  // for printer in printers::get_printers() {
  //   if printer.name != "Fax" && !printer.name.contains("Microsoft") {
  //     printer_list.push(printer);
  //   }
  // }
  printer_list
}

pub fn print(p_printer: PrinterDevice, file: &Path, opt: PrintTicket) {
  //let file_str = String::from(file.to_str().expect("Could not parse path"));
  let xps = PdfiumPrinter::new(p_printer);
  xps.print(file, opt).unwrap();
}