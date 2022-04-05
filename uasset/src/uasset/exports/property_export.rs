use std::io::{Cursor, Error, ErrorKind};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::implement_get;
use crate::uasset::Asset;
use crate::uasset::exports::normal_export::NormalExport;
use crate::uasset::exports::unknown_export::UnknownExport;
use crate::uasset::uproperty::UProperty;

use super::ExportNormalTrait;

pub struct PropertyExport {
    pub normal_export: NormalExport,

    pub property: UProperty
}

implement_get!(PropertyExport);

impl PropertyExport {
    pub fn from_unk(unk: &UnknownExport, asset: &mut Asset) -> Result<Self, Error> {
        let normal_export = NormalExport::from_unk(unk, asset)?;

        asset.cursor.read_i32::<LittleEndian>()?;

        let export_class_type = asset.get_export_class_type(normal_export.class_index).ok_or(Error::new(ErrorKind::Other, "No such class type"))?;
        let property = UProperty::new(asset, export_class_type)?;

        Ok(PropertyExport {
            normal_export,
            property
        })
    }
}