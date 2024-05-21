use devices::*;

// static mut DEVICES: Vec<DeviceInfo> = Vec::new();
//
// unsafe fn get_devices() {
//     let mut device_list = Vec::new();
//     device_list.append(&mut Devices::usb().unwrap_or_default());
//     device_list.append(&mut Devices::pci().unwrap_or_default());
//     DEVICES.append(&mut device_list);
// }

pub fn get_devices() -> Vec<DeviceInfo> {
    let mut device_list: Vec<DeviceInfo> = Vec::new();
    device_list.append(&mut Devices::usb().unwrap_or_default());
    device_list.append(&mut Devices::pci().unwrap_or_default());
    device_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_devices() {
        let device_lsit = get_devices();
        assert_ne!(Vec::<DeviceInfo>::new(), device_lsit);
    }
}