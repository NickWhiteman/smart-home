#[derive(Clone, Debug)]
pub struct SmartPlug {
    is_turn_on: bool,
}

pub trait SmartPlugTrait {
    fn new() -> Self;
    fn current_state(&self) -> bool;
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn current_power(&self) -> f32;
    fn get_plug(&self) -> &SmartPlug;
    fn state(&self) -> String;
}

impl SmartPlugTrait for SmartPlug {
    fn new() -> Self {
        SmartPlug { is_turn_on: false }
    }
    
    fn current_state(&self) -> bool {
        self.is_turn_on
    }

    fn turn_on(&mut self) {
        self.is_turn_on = true;
        println!("Smart plug turned on!");
    }

    fn turn_off(&mut self) {
        self.is_turn_on = false;
        println!("Smart plug turned off!");
    }

    fn current_power(&self) -> f32 {
        if self.is_turn_on {
            42.0
        } else {
            0.0
        }
    }

    fn get_plug(&self) -> &SmartPlug {
        self
    }

    fn state(&self) -> String {
        let mut report: String = String::new();
        report.push_str(&format!("Current state: {}", if self.is_turn_on { "On" } else { "Off" }));
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_plug() {
        let mut plug = SmartPlug::new();
        assert!(!plug.current_state());
        plug.turn_on();
        assert!(plug.current_state());
        plug.turn_off();
        assert!(!plug.current_state());
    }
}