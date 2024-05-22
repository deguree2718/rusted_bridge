use devices::*;


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
        let device_list = get_devices();
        // shit works like a mother fucking charm in linux
        assert_ne!(Vec::<DeviceInfo>::new(), device_list);
    }
}
