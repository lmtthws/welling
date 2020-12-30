use item::Item;

#[derive(Clone)]
pub struct Room {
    pub room_type: RoomType,
    walls: Vec<Wall>,
    items: Vec<Item>
}

#[derive(Clone,PartialEq,Debug)]
pub enum RoomType {
    LivingRoom,
    DiningRoom,
    Kitchen,
    Bathroom,
    Hallway,
    Bedroom
}

#[derive(Clone)]
pub struct Wall {
    length: usize,
    height: usize
}

pub trait Dimensioned {
    fn get_area(&self) -> usize;
    fn get_walls(&self) -> Vec<Wall>; 
}

impl Dimensioned for Room {
    fn get_area(&self) -> usize {
        let walls = self.get_walls();
        walls[0].length * walls[1].length //assumes rectangle
    }

    fn get_walls(&self) -> Vec<Wall> {
        self.walls.clone()
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn truo() { }
}