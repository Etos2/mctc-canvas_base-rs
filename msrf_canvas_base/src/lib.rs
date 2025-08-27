use std::num::NonZeroU32;

pub mod codec;

pub const CURRENT_VERSION: u16 = 0;

pub const CANVAS_META_TYPE_ID: u16 = 0x0000;
pub const PALETTE_INSERT_TYPE_ID: u16 = 0x0010;
pub const PALETTE_REMOVE_TYPE_ID: u16 = 0x0011;
pub const PLACEMENT_INSERT_TYPE_ID: u16 = 0x0020;
pub const PLACEMENT_INSERT_SILENT_TYPE_ID: u16 = 0x0021;
pub const PLACEMENT_INSERT_FILL_TYPE_ID: u16 = 0x0022;
pub const PLACEMENT_INSERT_FILL_SILENT_TYPE_ID: u16 = 0x0023;
pub const PLACEMENT_REMOVE_TYPE_ID: u16 = 0x0024;
pub const PLACEMENT_REMOVE_SILENT_TYPE_ID: u16 = 0x0025;
pub const PLACEMENT_REMOVE_FILL_TYPE_ID: u16 = 0x0026;
pub const PLACEMENT_REMOVE_FILL_SILENT_TYPE_ID: u16 = 0x0027;
pub const IDENTIFIER_NUMERIC_TYPE_ID: u16 = 0x0030;
pub const IDENTIFIER_STRING_TYPE_ID: u16 = 0x0031;
pub const IDENTIFIER_SECRET_TYPE_ID: u16 = 0x0032;

macro_rules! event_from {
    ($t:ident) => {
        impl From<$t> for CanvasRecord {
            fn from(value: $t) -> Self {
                CanvasRecord::$t(value)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum CanvasRecord {
    CanvasMeta(CanvasMeta) = CANVAS_META_TYPE_ID,
    PaletteInsert(PaletteInsert) = PALETTE_INSERT_TYPE_ID,
    PaletteRemove(PaletteRemove) = PALETTE_REMOVE_TYPE_ID,
    PlacementInsert(PlacementInsert) = PLACEMENT_INSERT_TYPE_ID,
    PlacementInsertQuiet(PlacementInsert) = PLACEMENT_INSERT_SILENT_TYPE_ID,
    PlacementInsertFill(PlacementInsertFill) = PLACEMENT_INSERT_FILL_TYPE_ID,
    PlacementInsertFillQuiet(PlacementInsertFill) = PLACEMENT_INSERT_FILL_SILENT_TYPE_ID,
    PlacementRemove(PlacementRemove) = PLACEMENT_REMOVE_TYPE_ID,
    PlacementRemoveQuiet(PlacementRemove) = PLACEMENT_REMOVE_SILENT_TYPE_ID,
    PlacementRemoveFill(PlacementRemoveFill) = PLACEMENT_REMOVE_FILL_TYPE_ID,
    PlacementRemoveFillQuiet(PlacementRemoveFill) = PLACEMENT_REMOVE_FILL_SILENT_TYPE_ID,
    IdentifierNumeric(u64) = IDENTIFIER_NUMERIC_TYPE_ID,
    IdentifierString(String) = IDENTIFIER_STRING_TYPE_ID,
    IdentifierSecret(Vec<u8>) = IDENTIFIER_SECRET_TYPE_ID,
}

event_from!(CanvasMeta);
event_from!(PaletteInsert);
event_from!(PaletteRemove);
event_from!(PlacementInsert);
event_from!(PlacementInsertFill);
event_from!(PlacementRemove);
event_from!(PlacementRemoveFill);

impl CanvasRecord {
    fn raw_id(&self) -> u16 {
        // SAFETY: Because `Self` is marked `repr(u16)` we can read the discriminant safely.
        unsafe { *<*const _>::from(self).cast::<u16>() }
    }

    pub fn is_silent(&self) -> bool {
        match self {
            Self::PlacementInsertQuiet(_) | Self::PlacementInsertFillQuiet(_) | Self::PlacementRemoveQuiet(_) | Self::PlacementRemoveFillQuiet(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasMeta {
    pub name: String,
    pub platform: String,
    pub time: u64,
    pub size: (u32, u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaletteInsert {
    pub offset: u32,
    pub colors: Vec<[u8; 4]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaletteRemove {
    pub offset: u32,
    pub length: NonZeroU32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementInsert {
    pub time: u64,
    pub pos: u64,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementInsertFill {
    pub time: u64,
    pub pos: (u64, u64),
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementRemove {
    pub time: u64,
    pub pos: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementRemoveFill {
    pub time: u64,
    pub pos: (u64, u64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Identifier {
    Numerical(u64),
    String(String),
    Secret(Vec<u8>),
}

//TODO: Size optimisation (NonMaximum???)
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MetaIdIndex(u32);

impl MetaIdIndex {
    pub fn is_unique(&self) -> bool {
        (self.0 >> 31) & 1 > 0
    }

    pub fn is_none(&self) -> bool {
        self.0 == 0x7FFFFFFF || self.0 == 0xFFFFFFFF
    }

    pub fn into_index(self) -> usize {
        (self.0 & 0x7FFFFFFF) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn meta_id_index() {
        let id = MetaIdIndex(0x00000012);
        assert!(!id.is_unique());
        assert!(!id.is_none());
        let id = MetaIdIndex(0x80000012);
        assert!(id.is_unique());
        assert!(!id.is_none());
        let id = MetaIdIndex(0xFFFFFFFF);
        assert!(id.is_unique());
        assert!(id.is_none());
    }
}