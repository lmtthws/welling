use mysql::packets::Header;

pub struct OkPacket {
    _header: Header,
}

pub struct ErrPacket {
    _header: Header,
}

pub struct EOFPacket {
    _header: Header,
}