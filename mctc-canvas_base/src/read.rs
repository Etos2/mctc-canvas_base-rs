use std::io::Cursor;

use crate::{
    error::{PError, PResult},
    util::ReadExt,
    *,
};

pub fn read_event(id: u64, value: &[u8]) -> PResult<CanvasEvent> {
    match id {
        CANVAS_META_ID => read_canvas_meta(value).map(CanvasEvent::from),
        CANVAS_MODIFY_ID => read_canvas_modify(value).map(CanvasEvent::from),
        PALETTE_CHUNK_ID => read_palette_chunk(value).map(CanvasEvent::from),
        PLACEMENT_ID => read_placement(value).map(CanvasEvent::from),
        PLACEMENT_QUIET_ID => read_placement(value).map(CanvasEvent::PlacementQuiet),
        PLACEMENT_CHUNK_ID => read_placement_chunk(value).map(CanvasEvent::from),
        PLACEMENT_CHUNK_QUIET_ID => {
            read_placement_chunk(value).map(CanvasEvent::PlacementChunkQuiet)
        }
        META_ID => read_meta_id(value).map(CanvasEvent::from),
        META_UNIQUE_ID => read_meta_id(value).map(CanvasEvent::MetaUniqueId),
        MODIFY_MATRIX_ID => read_modify_matrix(value).map(CanvasEvent::from),
        MODIFY_MAP_ID => read_modify_map(value).map(CanvasEvent::from),
        id => Err(PError::InvalidTypeId(id)),
    }
}

// TODO: Remove length check? Use Cursor::position to assert amount read.
fn check_len(len: usize, expected_len: usize) -> PResult<()> {
    if len != expected_len {
        Err(PError::InvalidLength(len, expected_len))
    } else {
        Ok(())
    }
}

fn read_canvas_meta(data: &[u8]) -> PResult<CanvasMeta> {
    let mut rdr = Cursor::new(data);

    let size = (rdr.read_u32()?, rdr.read_u32()?);
    let time_start = rdr.read_i64()?;
    let time_end = rdr.read_i64().map(|t| if t > 0 { Some(t) } else { None })?; // TODO: Assert greater than time_start
    let name_len = rdr.read_u16()? as usize;
    let name = rdr.read_vec(name_len).map(String::from_utf8)??;
    let platform_len = rdr.read_u16()? as usize;
    let platform = rdr.read_vec(platform_len).map(String::from_utf8)??;

    check_len(data.len(), name.len() + platform.len() + 28)?;
    Ok(CanvasMeta {
        size,
        time_start,
        time_end,
        name,
        platform,
    })
}

fn read_canvas_modify(data: &[u8]) -> PResult<CanvasModify> {
    let mut rdr = Cursor::new(data);
    check_len(data.len(), 8)?;
    Ok(CanvasModify {
        size: (rdr.read_u32()?, rdr.read_u32()?),
    })
}

fn read_palette_chunk(data: &[u8]) -> PResult<PaletteChunk> {
    let mut rdr = Cursor::new(data);

    let offset = rdr.read_u64()?;
    let colors = rdr.read_vec(data.len() - 8)?;
    let colors = colors
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .collect::<Vec<_>>();

    check_len(data.len(), 8 + colors.len() * 4)?;
    Ok(PaletteChunk { offset, colors })
}

fn read_placement(data: &[u8]) -> PResult<Placement> {
    let mut rdr = Cursor::new(data);
    check_len(data.len(), 18)?;

    let pos = (rdr.read_u32()?, rdr.read_u32()?);
    let color_index = rdr.read_u16()?;
    let time = rdr.read_i64()?;

    Ok(Placement {
        pos,
        time,
        color_index,
    })
}

fn read_placement_chunk(data: &[u8]) -> PResult<PlacementChunk> {
    let mut rdr = Cursor::new(data);
    let pos = (rdr.read_u32()?, rdr.read_u32()?);
    let time = rdr.read_i64()?;
    let color_indexes = rdr.read_vec(data.len() - 16)?;
    let color_indexes = color_indexes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect::<Vec<_>>();
    check_len(data.len(), 16 + color_indexes.len())?;

    Ok(PlacementChunk {
        pos,
        time,
        color_indexes,
    })
}

fn read_meta_id(data: &[u8]) -> PResult<MetaId> {
    let mut rdr = Cursor::new(data);
    let layout = rdr.read_u8()?;
    let id = rdr.read_vec(data.len() - 1)?;
    check_len(data.len(), 1 + id.len())?;

    match layout {
        0 => Ok(MetaId::Numerical(id)),
        1 => Ok(MetaId::Username(String::from_utf8(id)?)),
        l => Err(PError::InvalidValue(l as u64)),
    }
}

fn read_modify_matrix(data: &[u8]) -> PResult<ModifyMatrix> {
    let mut rdr = Cursor::new(data);
    check_len(data.len(), 40)?;

    let pos = (rdr.read_u32()?, rdr.read_u32()?);
    let size = (rdr.read_u32()?, rdr.read_u32()?);
    let matrix = [
        rdr.read_f32()?,
        rdr.read_f32()?,
        rdr.read_f32()?,
        rdr.read_f32()?,
        rdr.read_f32()?,
        rdr.read_f32()?,
    ];

    Ok(ModifyMatrix { pos, size, matrix })
}

fn read_modify_map(data: &[u8]) -> PResult<ModifyMap> {
    let mut rdr = Cursor::new(data);

    let pos = (rdr.read_u32()?, rdr.read_u32()?);
    let size = (rdr.read_u32()?, rdr.read_u32()?);
    let map = rdr.read_vec((size.0 * size.1 * 8) as usize)?;
    let map = map
        .chunks_exact(8)
        .map(|chunk| {
            (
                u32::from_le_bytes(chunk[..4].try_into().unwrap()),
                u32::from_le_bytes(chunk[4..].try_into().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    check_len(data.len(), 16 + (size.0 * size.1 * 8) as usize)?;

    Ok(ModifyMap { pos, size, map })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_canvas_meta() {
        let data = [
            32, 0, 0, 0, //                                     Width (32)
            64, 0, 0, 0, //                                     Height (64)
            96, 0, 0, 0, 0, 0, 0, 0, //                         Time Start (Some(96 ms))
            0, 0, 0, 0, 0, 0, 0, 0, //                          Time End (None)
            4, 0, //                                            Name Len (4)
            b't', b'e', b's', b't', //                          Name
            8, 0, //                                            Platform Name Len (8)
            b't', b'e', b's', b't', b'i', b'n', b'g', b'!', //  Platform Name
        ];
        let result = read_canvas_meta(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), CanvasMeta {
            size: (32, 64),
            time_start: 96,
            time_end: None,
            name: String::from("test"),
            platform: String::from("testing!")
        })
    }

    #[test]
    fn test_canvas_modify() {
        let data = [
            96, 0, 0, 0, // Width (96)
            64, 0, 0, 0, // Height (64)
        ];
        let result = read_canvas_modify(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), CanvasModify { size: (96, 64) })
    }

    #[test]
    fn test_palette_chunk() {
        let data = [
            96, 0, 0, 0, 0, 0, 0, 0, // Width (96)
            255, 255, 255, 255, //      Colors[0]
            0, 0, 0, 255, //            Colors[1]
        ];
        let result = read_palette_chunk(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), PaletteChunk {
            offset: 96,
            colors: vec![[255, 255, 255, 255], [0, 0, 0, 255],]
        })
    }

    #[test]
    fn test_placement() {
        let data = [
            24, 0, 0, 0, //             X (24)
            32, 0, 0, 0, //             Y (32)
            8, 0, //                    Color (8)
            64, 0, 0, 0, 0, 0, 0, 0, // Time Start (96)
        ];
        let result = read_placement(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), Placement {
            pos: (24, 32),
            color_index: 8,
            time: 64,
        })
    }

    #[test]
    fn test_meda_id() {
        let data = [
            0, //                       Layout
            0xD2, 0x04, 0x00, 0x00, //  ID
        ];
        let result = read_meta_id(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(
            result.unwrap(),
            MetaId::Numerical(u32::to_le_bytes(1234).to_vec())
        )
    }

    #[test]
    fn test_modify_matrix() {
        let data = [
            24, 0, 0, 0, //             X (24)
            32, 0, 0, 0, //             Y (32)
            255, 0, 0, 0, //            Width (24)
            128, 0, 0, 0, //            Height (32)
            0x00, 0x00, 0xc0, 0x3f, //  A (1.5)
            0x00, 0x00, 0x40, 0x40, //  B (3.0)
            0x00, 0x00, 0x00, 0x00, //  C (0.0)
            0x00, 0x00, 0x00, 0x00, //  D (0.0)
            0xcd, 0xcc, 0x4c, 0xc0, //  E (-3.2)
            0x00, 0x00, 0x00, 0xc3, //  F (-128.0)
        ];
        let result = read_modify_matrix(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), ModifyMatrix {
            pos: (24, 32),
            size: (255, 128),
            matrix: [1.5, 3.0, 0.0, 0.0, -3.2, -128.0],
        })
    }

    #[test]
    fn test_modify_map() {
        let data = [
            24, 0, 0, 0, //           X (24)
            32, 0, 0, 0, //           Y (32)
            1, 0, 0, 0, //            Width (24)
            2, 0, 0, 0, //            Height (32)
            0x32, 0x00, 0x00, 0x00, //  X1
            0x64, 0x00, 0x00, 0x00, //  Y1
            0x04, 0x00, 0x00, 0x00, //  X2
            0x06, 0x00, 0x00, 0x00, //  Y2
        ];
        let result = read_modify_map(&data);
        assert!(result.is_ok(), "parse error: {:?}", result);
        assert_eq!(result.unwrap(), ModifyMap {
            pos: (24, 32),
            size: (1, 2),
            map: vec![(0x32, 0x64), (0x04, 0x06)],
        })
    }
}
