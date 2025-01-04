#[repr(u64)]
pub enum CanvasEvent {
    CanvasMeta(CanvasMeta) = 0x00,
    CanvasModify(CanvasModify) = 0x01,
    PaletteChunk(PaletteChunk) = 0x10,
    Placement(Placement) = 0x20,
    PlacementQuiet(Placement) = 0x21,
    PlacementChunk(PlacementChunk) = 0x22,
    PlacementChunkQuiet(PlacementChunk) = 0x23,
    PlacementMeta(PlacementMeta) = 0x30,
    PlacementModifyMatrix(PlacementModifyMatrix) = 0x40,
    PlacementModifyMap(PlacementModifyMap) = 0x41,
}

pub struct CanvasMeta {
    size: (u32, u32),
    time_start: i64,
    time_end: Option<i64>,
    name: String,
    platform: String,
}

pub struct CanvasModify {
    size: (u32, u32),
}

pub struct PaletteChunk {
    offset: u64,
    colors: Vec<u32>,
}

pub struct Placement {
    pos: (u32, u32),
    time: i64,
    color_index: u16,
}

pub struct PlacementChunk {
    pos: (u32, u32),
    time: i64,
    color_indexes: Vec<u16>,
}

pub struct PlacementMeta {
    username: String,
}

pub struct PlacementModifyMatrix {
    pos: (u32, u32),
    size: (u32, u32),
    matrix: [f32; 6],
}

pub struct PlacementModifyMap {
    pos: (u32, u32),
    size: (u32, u32),
    map: Vec<(u32, u32)>,
}
