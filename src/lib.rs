use devices::*;

pub fn get_devices() -> Vec<DeviceInfo> {
    let mut device_list: Vec<DeviceInfo> = Vec::new();
    for device in Devices::get().unwrap_or_default(){
        device_list.push(device);
    }
    device_list
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
//
// pub fn myNameIsPaul() -> String {
//     String::from("Paul")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_devices() {
        println!("{:?}", get_devices());
    }
}
