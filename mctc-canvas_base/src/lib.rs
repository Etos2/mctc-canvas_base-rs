pub mod error;
pub mod read;
pub(crate) mod util;

pub const CANVAS_META_ID: u64 = 0x00000000;
pub const CANVAS_MODIFY_ID: u64 = 0x00000001;
pub const PALETTE_CHUNK_ID: u64 = 0x00000010;
pub const PLACEMENT_ID: u64 = 0x00000020;
pub const PLACEMENT_QUIET_ID: u64 = 0x00000021;
pub const PLACEMENT_CHUNK_ID: u64 = 0x00000022;
pub const PLACEMENT_CHUNK_QUIET_ID: u64 = 0x00000023;
pub const META_ID: u64 = 0x00000030;
pub const META_UNIQUE_ID: u64 = 0x00000031;
pub const MODIFY_MATRIX_ID: u64 = 0x00000040;
pub const MODIFY_MAP_ID: u64 = 0x00000041;

macro_rules! event_from {
    ($t:ident) => {
        impl From<$t> for CanvasEvent {
            fn from(value: $t) -> Self {
                CanvasEvent::$t(value)
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
#[repr(u64)]
pub enum CanvasEvent {
    CanvasMeta(CanvasMeta) = CANVAS_META_ID,
    CanvasModify(CanvasModify) = CANVAS_MODIFY_ID,
    PaletteChunk(PaletteChunk) = PALETTE_CHUNK_ID,
    Placement(Placement) = PLACEMENT_ID,
    PlacementQuiet(Placement) = PLACEMENT_QUIET_ID,
    PlacementChunk(PlacementChunk) = PLACEMENT_CHUNK_ID,
    PlacementChunkQuiet(PlacementChunk) = PLACEMENT_CHUNK_QUIET_ID,
    MetaId(MetaId) = META_ID,
    MetaUniqueId(MetaId) = META_UNIQUE_ID,
    ModifyMatrix(ModifyMatrix) = MODIFY_MATRIX_ID,
    ModifyMap(ModifyMap) = MODIFY_MAP_ID,
}

event_from!(CanvasMeta);
event_from!(CanvasModify);
event_from!(PaletteChunk);
event_from!(Placement);
event_from!(PlacementChunk);
event_from!(MetaId);
event_from!(ModifyMatrix);
event_from!(ModifyMap);

impl CanvasEvent {
    pub fn type_id(&self) -> u64 {
        // SAFETY: Because `Self` is marked `repr(u64)` we can read the discriminant safely.
        unsafe { *<*const _>::from(self).cast::<u64>() }
    }

    pub fn is_silent(&self) -> bool {
        match self {
            Self::PlacementQuiet(_) | Self::PlacementChunkQuiet(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasMeta {
    pub(crate) size: (u32, u32),
    pub(crate) time_start: i64,
    pub(crate) time_end: Option<i64>,
    pub(crate) name: String,
    pub(crate) platform: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasModify {
    pub(crate) size: (u32, u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaletteChunk {
    pub(crate) offset: u64,
    pub(crate) colors: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub(crate) pos: (u32, u32),
    pub(crate) color_index: u16,
    pub(crate) time: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementChunk {
    pub(crate) pos: (u32, u32),
    pub(crate) time: i64,
    pub(crate) color_indexes: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaId {
    Numerical(Vec<u8>),
    Username(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyMatrix {
    pub(crate) pos: (u32, u32),
    pub(crate) size: (u32, u32),
    pub(crate) matrix: [f32; 6],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyMap {
    pub(crate) pos: (u32, u32),
    pub(crate) size: (u32, u32),
    pub(crate) map: Vec<(u32, u32)>,
}
