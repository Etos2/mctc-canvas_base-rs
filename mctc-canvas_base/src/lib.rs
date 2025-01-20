use std::io::{Read, Write};

use error::PError;
use mctc_parser::{Codec, ReadRecord, RecordImpl, WriteRecord, data::RecordMeta};
use read::read_event;
use write::write_event;

pub mod error;
pub mod read;
pub(crate) mod util;
pub mod write;

pub const CURRENT_VERSION: u16 = 0;

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
    fn raw_id(&self) -> u64 {
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

impl RecordImpl for CanvasEvent {
    fn type_id(&self) -> u64 {
        self.raw_id()
    }

    fn length(&self) -> usize {
        todo!()
    }
}

impl ReadRecord<PError> for CanvasEvent {
    fn read_from(rdr: impl Read, meta: RecordMeta) -> Result<Self, PError> {
        read_event(meta.type_id(), meta.len(), rdr)
    }
}

impl WriteRecord<PError> for CanvasEvent {
    fn write_into(&self, wtr: impl Write) -> Result<(), PError> {
        write_event(wtr, self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasMeta {
    pub size: (u32, u32),
    pub time_start: i64,
    pub time_end: Option<i64>, // TODO: Remove option?
    pub name: String,
    pub platform: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasModify {
    pub size: (u32, u32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaletteChunk {
    pub offset: u64,
    pub colors: Vec<[u8; 4]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub pos: (u32, u32),
    pub time: i64,
    pub color_index: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlacementChunk {
    pub pos: (u32, u32),
    pub time: i64,
    pub color_indexes: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaId {
    Numerical(Vec<u8>),
    Username(String),
}

impl MetaId {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            MetaId::Numerical(v) => &v,
            MetaId::Username(s) => s.as_bytes(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyMatrix {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub matrix: [f32; 6],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyMap {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub map: Vec<(u32, u32)>,
}

pub struct CanvasBaseCodec {
    id: u64,
}

impl CanvasBaseCodec {
    pub fn new(id: u64) -> Self {
        CanvasBaseCodec { id }
    }
}

impl Codec for CanvasBaseCodec {
    const NAME: &'static str = "CANVAS_BASE";
    const VERSION: u16 = CURRENT_VERSION;
    type Err = PError;
    type Rec = CanvasEvent;

    fn codec_id(&self) -> u64 {
        self.id
    }

    fn write_record(&mut self, wtr: impl Write, rec: &Self::Rec) -> Result<(), Self::Err> {
        rec.write_into(wtr)
    }

    fn read_record(&mut self, rdr: impl Read, meta: RecordMeta) -> Result<Self::Rec, Self::Err> {
        CanvasEvent::read_from(rdr, meta)
    }
}
