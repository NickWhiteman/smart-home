use smart_home::{
    smart_device::SmartDevice, 
    smart_plug::{SmartPlug, SmartPlugTrait}, 
    smart_room::SmartRoom, 
    smart_thermometer::{SmartThermometer, SmartThermometerTrait},
    smart_home::SmartHome,
};



fn main() {
    let thermometer = SmartThermometer::new(42.0);
    let plug = SmartPlug::new();
    let devices = vec![
        SmartDevice::Thermometer(thermometer),
        SmartDevice::Plug(plug),
    ];
    let room = SmartRoom::new(devices);
    let room1 = SmartRoom::new(
        vec![
            SmartDevice::Thermometer(SmartThermometer::new(42.0)), 
            SmartDevice::Plug(SmartPlug::new()), 
            SmartDevice::Plug(SmartPlug::new())
        ]
    );
    let room2 = SmartRoom::new(
        vec![
            SmartDevice::Plug(SmartPlug::new()), 
            SmartDevice::Plug(SmartPlug::new())
        ]
    );
    let mut home = SmartHome::new(vec![room, room1, room2]);
    println!("Report: {}", home.get_report_home());
    home.get_mut_room(2).get_mut_device(0).toggle_plug();
    println!("Report after toggling plug in room 2: {}", home.get_report_home());
}
