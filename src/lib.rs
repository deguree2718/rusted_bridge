use devices::*;

pub fn get_devices() -> Vec<DeviceInfo> {
    Devices::get().unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_devices() {
        assert_ne!(Vec::<DeviceInfo>::new(), get_devices());
    }
}
