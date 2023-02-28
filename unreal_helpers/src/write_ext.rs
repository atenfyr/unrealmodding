//! Extension for anything that implements Write to more easily write Unreal data formats.

use std::io::{Result, Write};
use std::mem::size_of;

use byteorder::{LittleEndian, WriteBytesExt};

/// Extension for anything that implements Write to more easily write Unreal data formats.
pub trait UnrealWriteExt {
    /// Write string of format \<length i32\>\<string\>\<null\>
    fn write_fstring(&mut self, string: Option<&str>) -> Result<usize>;
    /// Write bool as u8
    fn write_bool(&mut self, value: bool) -> Result<()>;
}

impl<W: Write> UnrealWriteExt for W {
    fn write_fstring(&mut self, string: Option<&str>) -> Result<usize> {
        if let Some(string) = string {
            let is_unicode = string.len() != string.chars().count();

            if is_unicode {
                let utf16 = string.encode_utf16().collect::<Vec<_>>();

                // this is safe because we know that string is utf16 and therefore can easily be aligned to u8
                // this is also faster than alternatives without unsafe block
                let (_, aligned, _) = unsafe { utf16.align_to::<u8>() };

                self.write_i32::<LittleEndian>(-(aligned.len() as i32 / 2) - 1)?;
                self.write_all(aligned)?;

                self.write_all(&[0u8; 2])?;
                Ok(size_of::<i32>() + aligned.len())
            } else {
                self.write_i32::<LittleEndian>(string.len() as i32 + 1)?;
                let bytes = string.as_bytes();
                self.write_all(bytes)?;
                self.write_all(&[0u8; 1])?;

                Ok(size_of::<i32>() + bytes.len() + 1)
            }
        } else {
            self.write_i32::<LittleEndian>(0)?;
            Ok(size_of::<i32>())
        }
    }

    fn write_bool(&mut self, value: bool) -> Result<()> {
        self.write_u8(match value {
            true => 1,
            false => 0,
        })
    }
}