mod zip_code;
mod address;
mod room;
mod floor;
mod item;
mod currency;

use ::address::Address;
use ::floor::{Floor, Unit};

pub struct Property {
    address: Address,
    buildings: Vec<Building>, //should this be elevation or something to encompasss interior and exterior?
    grounds: Grounds,
    parcel: Parcel
}

pub struct Grounds {

}

pub struct Parcel {
    sides: Vec<usize>
}

pub struct Building {
    floors: Vec<Floor>,
    units: Vec<Unit>
}

