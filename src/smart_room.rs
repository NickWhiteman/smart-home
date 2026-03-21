use crate::smart_device::SmartDevice;

#[derive(Debug, Clone)]
pub struct SmartRoom {
    devices: Vec<SmartDevice>,
}

impl SmartRoom {
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        SmartRoom { devices }
    }

    pub fn add_device(&mut self, device: SmartDevice) {
        self.devices.push(device);
    }

    pub fn get_device(&self, index: usize) -> &SmartDevice {
        if index >= self.devices.len() {
            panic!("Device index out of bounds");
        }
        
        &self.devices[index]
    }

    pub fn get_mut_device(&mut self, index: usize) -> &mut SmartDevice {
        if index >= self.devices.len() {
            panic!("Device index out of bounds");
        }

        &mut self.devices[index]
    }

    pub fn get_report_rooms(&self) -> String {
        let mut report = String::new();
        for (index, device) in self.devices.iter().enumerate() {
            report.push_str(&format!("Device {}: {:?}\n", index, device.state()));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_plug::SmartPlugTrait;
    use crate::smart_thermometer::SmartThermometerTrait;

    #[test]
    fn test_smart_room() {
        let thermometer = SmartThermometerTrait::new(42.0);
        let plug = SmartPlugTrait::new();
        let devices = vec![
            SmartDevice::Thermometer(thermometer),
            SmartDevice::Plug(plug),
        ];
        let room = SmartRoom::new(devices);
        assert_eq!(room.get_device(0).state(), "Current temperature: 42°C");
        assert_eq!(room.get_device(1).state(), "Current state: Off");
    }
}