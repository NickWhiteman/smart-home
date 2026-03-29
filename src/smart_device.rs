use crate::{smart_plug::*, smart_thermometer::*};

#[derive(Debug, Clone)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Plug(SmartPlug),
}

impl SmartDevice {
    pub fn new(device: SmartDevice) -> Self {
        device
    }

    pub fn report(&self) -> String {
        match self {
            SmartDevice::Thermometer(thermometer) => thermometer.report(),
            SmartDevice::Plug(plug) => plug.report(),
            _ => String::from("Unknown device"),
        }
    }

    pub fn toggle_plug(&mut self) {
        if let SmartDevice::Plug(plug) = self {
            if plug.current_state() {
                plug.turn_off();
            } else {
                plug.turn_on();
            }
        }
    }
}