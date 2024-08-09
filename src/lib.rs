use std::path::Path;
//use wasm_bindgen::prelude::wasm_bindgen;
use printers;
use printers::printer::Printer;

pub fn get_printers() -> Vec<Printer> {
  let mut printer_list = Vec::new();
  for printer in printers::get_printers() {
    if printer.name.contains("Microsoft") && printer.name != "Fax" && printer.name.to_lowercase() == printer.driver_name.to_lowercase() {
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

#[cfg(test)]
mod tests {
  use std::fs;
  use super::*;
  use imcon::{Format, Image};
  use recibo::{Alignment, FileDriver, GraphicSize};
  use recibo::Printer;

  #[test]
  fn find_devices() {
    let device_list = get_printers();
    let mut result: bool = true;
    let file = fs::read("C:\\Users\\paulo\\Downloads\\0dd30670-4e01-499b-8ea3-6c86882e8400.pdf");
    let mut img = Image::read(file.unwrap().as_slice(), Format::Pdf);
    img.unwrap().save("C:\\Users\\paulo\\Desktop\\pdf.png");
    let driver = FileDriver::open("C:\\Users\\paulo\\OneDrive\\Área de Trabalho\\arquivo-teste.txt");
    let mut printer = Printer::open(driver).unwrap();
    printer.init().unwrap()
      .align(Alignment::Center).unwrap()
      .graphic(move |builder| {
        builder.path("C:\\Users\\paulo\\Desktop\\pdf.png")
          .size(GraphicSize::Normal)
      }).unwrap();
    // "C:\\Users\\paulo\\OneDrive\\Área de Trabalho\\arquivo-teste.txt"
    // for printer in device_list {
    //   let file = fs::read("C:\\Users\\paulo\\Downloads\\0dd30670-4e01-499b-8ea3-6c86882e8400.pdf");
    //   result = printer.print(file.unwrap().as_slice(), None).expect("Could not print file");
    //   //result = printer.print_file("C:\\Users\\paulo\\Downloads\\0dd30670-4e01-499b-8ea3-6c86882e8400.pdf", None).expect("Could not print file");
    // }
    // assert!(result);
  }
}