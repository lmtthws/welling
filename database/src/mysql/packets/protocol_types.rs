use std::fmt::{Display, Formatter, Error};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct u24(pub u32); //TODO, implement math ops for this

impl u24 {
    pub const MAX: u32 = 1 << 24;
}

impl From<u24> for u64 {
    fn from(u: u24) -> Self {
        u.0 as u64
    }
}


#[derive(Clone)]
pub enum FixedInteger {
    Int1([u8;1]),
    Int2([u8;2]),
    Int3([u8;3]),
    Int4([u8;4])
}

impl Display for FixedInteger {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      match *self {
          FixedInteger::Int1(ref i) =>  write!(f,"{:?}", i)?,
          FixedInteger::Int2(ref i) =>  write!(f,"{:?}", i)?,
          FixedInteger::Int3(ref i) =>  write!(f,"{:?}", i)?,
          FixedInteger::Int4(ref i) =>  write!(f,"{:?}", i)?
      }
      Ok(())
    }
}


#[derive(Clone,Copy)]
pub enum LengthInteger {
    U8(u8),
    U16(u16), //prefixed with 0xFC
    U24(u32), // prefixed with 0xFD
    U64(u64) //prefixed with 0xFe
}

impl LengthInteger {
    pub const TWO_BYTE_PREFIX: u8 = 0xFC;
    pub const THREE_BYTE_PREFIX: u8 = 0xFD;
    pub const EIGHT_BYTE_PREFIX: u8 = 0xFE;

    pub fn new(integer: u64) -> LengthInteger {
        if integer < 251 {
            LengthInteger::U8((integer & 0xFF) as u8)
        } else if integer < (1<<16) {
            LengthInteger::U16((integer & 0xFFFF) as u16)
        } else if integer < (1<<24) {
            LengthInteger::U24((integer & 0xFFFF_FF) as u32)
        } else {
           LengthInteger::U64(integer)
        }
    }
    
    pub fn total_bytes(&self) -> u64 {
        match *self {
            LengthInteger::U8(_) => 1,
            LengthInteger::U16(_) => 3,
            LengthInteger::U24(_) => 4,
            LengthInteger::U64(_) => 9
        }
    }

    pub fn value(&self) -> u64 {
        match *self {
            LengthInteger::U8(i) => i as u64,
            LengthInteger::U16(i) => i as u64,
            LengthInteger::U24(i) => i as u64,
            LengthInteger::U64(i) => i
        }
    }
}

impl Display for LengthInteger {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
       match *self {
            LengthInteger::U8(i) => write!(f,"{}",i)?,
            LengthInteger::U16(i) => write!(f,"{}{}",LengthInteger::TWO_BYTE_PREFIX,i)?, 
            LengthInteger::U24(i) => write!(f,"{}{}",LengthInteger::THREE_BYTE_PREFIX,i)?,
            LengthInteger::U64(i) => write!(f,"{}{}",LengthInteger::EIGHT_BYTE_PREFIX,i)?
        };
        Ok(())
    }
}


#[derive(Clone)]
pub struct LengthEncodedString(LengthInteger, String);

impl LengthEncodedString{
    pub fn from_unchecked(length: LengthInteger, text: String) -> LengthEncodedString {
        LengthEncodedString(length, text)
    }

    pub fn from(string: String) -> Self {
        let len = LengthInteger::new(string.len() as u64);
        LengthEncodedString(len, string)
    }

    //TODO: handle overflow
    pub fn packet_size(&self) -> u64 {
        self.0.total_bytes() + self.1.len() as u64
    }

    pub fn size(&self) -> LengthInteger {
        self.0
    }

    pub fn text(&self) -> &str {
        &self.1
    }
}

impl Display for LengthEncodedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, self.1)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct NullTerminatedString(String);

impl NullTerminatedString {
    pub fn from(text: &str) -> NullTerminatedString {
        NullTerminatedString(String::from(text))
    }

    //TODO: handle overflow...
    pub fn packet_size(&self) -> u64 {
        self.0.len() as u64 + 1
    }
}

impl Display for NullTerminatedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, '\0')?;
        Ok(())
    }
}