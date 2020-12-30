use room::{Room};

pub struct Unit {
    levels: Vec<Floor>,
    rooms: Option<Vec<Room>>
}

pub struct Floor {
    elevation: f32,
    rooms: Vec<Room>
}



impl Unit {
    pub fn add_room(&mut self, room: Room) -> Vec<Room>  {
        match self.rooms {
            None => self.rooms = Some(vec![room]),
            Some(ref mut r)=> r.insert(r.len(), room)
        }

        self.rooms.as_ref().unwrap().clone()
    }

}