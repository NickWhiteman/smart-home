use crate::smart_room::SmartRoom;

pub(crate) struct SmartHome {
    rooms: Vec<SmartRoom>,
}

impl SmartHome {
    pub fn new(rooms: Vec<SmartRoom>) -> Self {
        SmartHome { rooms }
    }

    pub fn add_room(&mut self, room: SmartRoom) {
        self.rooms.push(room);
    }

    pub fn get_room(&self, index: usize) -> &SmartRoom {
        if index >= self.rooms.len() {
            panic!("Room index out of bounds");
        }
        
        &self.rooms[index]
    }

    pub fn get_mut_room(&mut self, index: usize) -> &mut SmartRoom {
        if index >= self.rooms.len() {
            panic!("Room index out of bounds");
        }

        &mut self.rooms[index]
    }

    pub fn get_report_home(&self) -> String {
        let mut report: String = String::new();
        for (index, room) in self.rooms.iter().enumerate() {
            report.push_str(&format!("Room {}: {}\n", index, room.get_report_rooms()));
        }
        report
    }
}
