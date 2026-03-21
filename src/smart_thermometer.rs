#[derive(Debug, Clone)]
pub struct SmartThermometer {
    current_temperature: f32,
}

pub trait SmartThermometerTrait {
    fn new(current_temperature: f32) -> Self;
    fn get_temperature(&self) -> f32;
    fn get_thermometer(&self) -> &SmartThermometer;
    fn state(&self) -> String;
}

impl SmartThermometerTrait for SmartThermometer {
    fn new(current_temperature: f32) -> Self {
        SmartThermometer { current_temperature }
    }

    fn get_temperature(&self) -> f32 {
        self.current_temperature
    }

    fn get_thermometer(&self) -> &SmartThermometer {
        self
    }

    fn state(&self) -> String {
        let mut report: String = String::new();
        report.push_str(&format!("Current temperature: {}°C", self.current_temperature));
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_thermometer() {
        let thermometer = SmartThermometer::new(42.0);
        assert_eq!(thermometer.get_temperature(), 42.0);
    }
}