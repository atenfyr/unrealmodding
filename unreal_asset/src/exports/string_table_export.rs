use crate::asset_reader::AssetReader;
use crate::asset_writer::AssetWriter;
use crate::cursor_ext::CursorExt;
use crate::error::Error;
use crate::exports::base_export::BaseExport;
use crate::exports::normal_export::NormalExport;
use crate::exports::ExportTrait;
use crate::implement_get;
use crate::unreal_types::StringTable;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

use super::ExportBaseTrait;
use super::ExportNormalTrait;

#[derive(Clone)]
pub struct StringTableExport {
    normal_export: NormalExport,

    table: StringTable,
}

implement_get!(StringTableExport);

impl StringTableExport {
    pub fn from_base<Reader: AssetReader>(
        base: &BaseExport,
        asset: &mut Reader,
    ) -> Result<Self, Error> {
        let normal_export = NormalExport::from_base(base, asset)?;
        asset.read_i32::<LittleEndian>()?;

        let mut table = StringTable::new(asset.read_string()?);

        let num_entries = asset.read_i32::<LittleEndian>()?;
        for _i in 0..num_entries {
            table.value.insert(
                asset
                    .read_string()?
                    .ok_or_else(|| Error::no_data("StringTable key is None".to_string()))?,
                asset
                    .read_string()?
                    .ok_or_else(|| Error::no_data("StringTable value is None".to_string()))?,
            );
        }

        Ok(StringTableExport {
            normal_export,
            table,
        })
    }
}

impl ExportTrait for StringTableExport {
    fn write<Writer: AssetWriter>(
        &self,
        asset: &Writer,
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<(), Error> {
        self.normal_export.write(asset, cursor)?;
        cursor.write_i32::<LittleEndian>(0)?;

        cursor.write_string(&self.table.namespace)?;
        cursor.write_i32::<LittleEndian>(self.table.value.len() as i32)?;
        for (key, value) in &self.table.value {
            cursor.write_string(&Some(key.clone()))?;
            cursor.write_string(&Some(value.clone()))?;
        }
        Ok(())
    }
}
