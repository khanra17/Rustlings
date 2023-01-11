use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
pub enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion)
        }
        return Ok(Color{red:red as u8,green: green as u8,blue: blue as u8})
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let red = arr[0];
        let green = arr[1];
        let blue = arr[2];
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion)
        }
        return Ok(Color{red:red as u8,green: green as u8,blue: blue as u8})
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 { return Err(IntoColorError::BadLen) }
        let red = slice[0];
        let green = slice[1];
        let blue = slice[2];
        if red < 0 || red > 255 || green < 0 || green > 255 || blue < 0 || blue > 255 {
            return Err(IntoColorError::IntConversion)
        }
        return Ok(Color{red:red as u8,green: green as u8,blue: blue as u8})
    }
}


