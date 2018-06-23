use std::fmt::{Display, Formatter, Error};

#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub struct u24(pub u32);

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
    pub fn new(integer: u64) -> LengthInteger {
        if integer < 251 {
            LengthInteger::U8(integer as u8)
        } else if integer < (1<<16) {
            LengthInteger::U16(integer as u16)
        } else if integer < (1<<24) {
            LengthInteger::U24(integer as u32)
        } else {
           LengthInteger::U64(integer)
        }
    }
    
    pub fn total_bytes(&self) -> u64 {
        match *self {
            LengthInteger::U8(i) => 1 + i as u64,
            LengthInteger::U16(i) => 3 + i as u64,
            LengthInteger::U24(i) => 4 + i as u64,
            LengthInteger::U64(i) => 9 + i
        }
    }
}

impl Display for LengthInteger {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
       match *self {
            LengthInteger::U8(i) => write!(f,"{}",i)?,
            LengthInteger::U16(i) => write!(f,"{}{}",0xFC,i)?,
            LengthInteger::U24(i) => write!(f,"{}{}",0xFD,i)?,
            LengthInteger::U64(i) => write!(f,"{}{}",0xFE,i)?
        };
        Ok(())
    }
}


#[derive(Clone)]
pub struct LengthEncodedString(pub LengthInteger, pub String);

impl Display for LengthEncodedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, self.1)?;
        Ok(())
    }
}


pub struct NullTerminatedString(String);

impl Display for NullTerminatedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f,"{}{}", self.0, '\0')?;
        Ok(())
    }
}