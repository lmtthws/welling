pub enum Endian {
    Little,
    Big
}


pub fn get_bytes(num: u32) -> [u8;4] {
    let first : u8 = ((num >> 24) & 0xff) as u8;
    let second : u8 = ((num >> 16) & 0xff) as u8;
    let third : u8 = ((num >> 8) & 0xff) as u8;
    let fourth : u8 = (num & 0xff) as u8;
    [first, second, third, fourth]
}