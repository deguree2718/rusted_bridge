//use std::{fs::{File}, env};
//use ipp::prelude::*;
use cups_rs::*;



#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

//#[wasm_bindgen]
pub fn get_printers() -> Vec<Destination> {
	get_all_destinations().unwrap()
}

//#[wasm_bindgen]
pub fn print_file(_printer_name: String, _file_path: String) -> bool {
/*	cfg_select! {
		unix => {
			let payload = IppPayload::new(File::open(file_path).unwrap());
			let uri: Uri = ("http://localhost:631/printers/".to_owned() + &printer_name).parse().unwrap();
			let builder = IppOperationBuilder::print_job(uri.clone(), payload)
				.user_name(env::var("USER").unwrap_or_else(|_| "noname".to_owned()))
				.job_title("None")
				.attribute(IppAttribute::new("document-format", IppValue::MimeMediaType(String::from("application/octet-stream"))));

			let operation = builder.build();
			let client = IppClient::new(uri);
			let response = client.send(operation).unwrap();

			println!("IPP Status code: {}", response.header().status_code());

			return response.header().status_code().is_success();
		},
		_ => {
			false
		}
	}  */
	false
}


#[cfg(test)]
mod tests{
	use super::*;
	
	
	#[test]
	fn find_devices() { // file_path: String
		let printers = get_all_destinations().unwrap();
		println!("Found {} printers", printers.len());
		for printer in printers {
			println!("device_uri: {:?}", printer.device_uri());
		}
	}
}
