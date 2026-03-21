use crate::{smart_plug::SmartPlugTrait, smart_thermometer::SmartThermometerTrait};
mod smart_thermometer;
mod smart_plug;
mod smart_device;
mod smart_room;
mod smart_home;


fn main() {
    let thermometer = smart_thermometer::SmartThermometer::new(42.0);
    let plug = smart_plug::SmartPlug::new();
    let devices = vec![
        smart_device::SmartDevice::Thermometer(thermometer),
        smart_device::SmartDevice::Plug(plug),
    ];
    let room = smart_room::SmartRoom::new(devices);
    let room1 = smart_room::SmartRoom::new(
        vec![
            smart_device::SmartDevice::Thermometer(smart_thermometer::SmartThermometer::new(42.0)), 
            smart_device::SmartDevice::Plug(smart_plug::SmartPlug::new()), 
            smart_device::SmartDevice::Plug(smart_plug::SmartPlug::new())
        ]
    );
    let room2 = smart_room::SmartRoom::new(
        vec![
            smart_device::SmartDevice::Plug(smart_plug::SmartPlug::new()), 
            smart_device::SmartDevice::Plug(smart_plug::SmartPlug::new())
        ]
    );
    let mut home = smart_home::SmartHome::new(vec![room, room1, room2]);
    println!("Report: {}", home.get_report_home());
    home.get_mut_room(2).get_mut_device(0).toggle_plug();
    println!("Report after toggling plug in room 2: {}", home.get_report_home());
}
