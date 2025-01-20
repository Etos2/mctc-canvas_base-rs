use std::io::Write;

use crate::{error::PResult, util::WriteExt, *};

pub const CANVAS_MODIFY_LEN: usize = 8;
pub const PLACEMENT_LEN: usize = 18;
pub const MODIFY_MATRIX_LEN: usize = 40;

pub const CANVAS_META_MIN_LEN: usize = 28;
pub const PALETTE_CHUNK_MIN_LEN: usize = 8;
pub const PLACEMENT_CHUNK_MIN_LEN: usize = 16;
pub const META_ID_MIN_LEN: usize = 1;
pub const MODIFY_MAP_MIN_LEN: usize = 16;

// TODO: Consider not pre-calculating length? (This would involve writing into a vec, then writing to IO.)
pub fn event_len(event: &CanvasEvent) -> usize {
    match event {
        CanvasEvent::CanvasMeta(e) => CANVAS_META_MIN_LEN + e.name.len() + e.platform.len(),
        CanvasEvent::CanvasModify(_) => CANVAS_MODIFY_LEN,
        CanvasEvent::PaletteChunk(e) => {
            PALETTE_CHUNK_MIN_LEN + e.colors.len() * (u32::BITS / 8) as usize
        }
        CanvasEvent::Placement(_) | CanvasEvent::PlacementQuiet(_) => PLACEMENT_LEN,
        CanvasEvent::PlacementChunk(e) | CanvasEvent::PlacementChunkQuiet(e) => {
            PLACEMENT_CHUNK_MIN_LEN + e.color_indexes.len() * (u16::BITS / 8) as usize
        }
        CanvasEvent::MetaId(e) | CanvasEvent::MetaUniqueId(e) => {
            META_ID_MIN_LEN + e.as_bytes().len()
        }
        CanvasEvent::ModifyMatrix(_) => MODIFY_MATRIX_LEN,
        CanvasEvent::ModifyMap(e) => {
            MODIFY_MAP_MIN_LEN + e.map.len() * 2 * (u32::BITS / 8) as usize
        }
    }
}

pub fn write_event(wtr: impl Write, event: &CanvasEvent) -> PResult<()> {
    match event {
        CanvasEvent::CanvasMeta(canvas_meta) => write_canvas_meta(wtr, canvas_meta),
        CanvasEvent::CanvasModify(canvas_modify) => write_canvas_modify(wtr, canvas_modify),
        CanvasEvent::PaletteChunk(palette_chunk) => write_palette_chunk(wtr, palette_chunk),
        CanvasEvent::Placement(placement) => write_placement(wtr, placement),
        CanvasEvent::PlacementQuiet(placement) => write_placement(wtr, placement),
        CanvasEvent::PlacementChunk(placement_chunk) => write_placement_chunk(wtr, placement_chunk),
        CanvasEvent::PlacementChunkQuiet(placement_chunk) => {
            write_placement_chunk(wtr, placement_chunk)
        }
        CanvasEvent::MetaId(meta_id) => write_meta_id(wtr, meta_id),
        CanvasEvent::MetaUniqueId(meta_id) => write_meta_id(wtr, meta_id),
        CanvasEvent::ModifyMatrix(modify_matrix) => write_modify_matrix(wtr, modify_matrix),
        CanvasEvent::ModifyMap(modify_map) => write_modify_map(wtr, modify_map),
    }
}

fn cast_bytes<T>(src: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            src.as_ptr() as *const u8,
            src.len() * std::mem::size_of::<T>(),
        )
    }
}

fn write_canvas_meta(mut wtr: impl Write, event_data: &CanvasMeta) -> PResult<()> {
    wtr.write_u32(event_data.size.0)?;
    wtr.write_u32(event_data.size.1)?;
    wtr.write_i64(event_data.time_start)?;
    wtr.write_i64(event_data.time_end.unwrap_or(0))?;
    wtr.write_u16(event_data.name.len() as u16)?; // TODO: Assert truncation
    wtr.write_all(event_data.name.as_bytes())?;
    wtr.write_u16(event_data.platform.len() as u16)?; // TODO: Assert truncation
    wtr.write_all(event_data.platform.as_bytes())?;

    Ok(())
}

fn write_canvas_modify(mut wtr: impl Write, event_data: &CanvasModify) -> PResult<()> {
    wtr.write_u32(event_data.size.0)?;
    wtr.write_u32(event_data.size.1)?;

    Ok(())
}

fn write_palette_chunk(mut wtr: impl Write, event_data: &PaletteChunk) -> PResult<()> {
    wtr.write_u64(event_data.offset)?;
    wtr.write_all(cast_bytes(&event_data.colors))?;

    Ok(())
}

fn write_placement(mut wtr: impl Write, event_data: &Placement) -> PResult<()> {
    wtr.write_u32(event_data.pos.0)?;
    wtr.write_u32(event_data.pos.1)?;
    wtr.write_i64(event_data.time)?;
    wtr.write_u16(event_data.color_index)?;

    Ok(())
}

fn write_placement_chunk(mut wtr: impl Write, event_data: &PlacementChunk) -> PResult<()> {
    wtr.write_u32(event_data.pos.0)?;
    wtr.write_u32(event_data.pos.1)?;
    wtr.write_i64(event_data.time)?;
    for c in &event_data.color_indexes {
        wtr.write_u16(*c)?;
    }

    Ok(())
}

fn write_meta_id(mut wtr: impl Write, event_data: &MetaId) -> PResult<()> {
    match event_data {
        MetaId::Numerical(int_vec) => {
            wtr.write_u8(0x00)?;
            wtr.write_all(&int_vec)?;
        }
        MetaId::Username(user_str) => {
            wtr.write_u8(0x01)?;
            wtr.write_all(&user_str.as_bytes())?;
        }
    }

    Ok(())
}

fn write_modify_matrix(mut wtr: impl Write, event_data: &ModifyMatrix) -> PResult<()> {
    wtr.write_u32(event_data.pos.0)?;
    wtr.write_u32(event_data.pos.1)?;
    wtr.write_u32(event_data.size.0)?;
    wtr.write_u32(event_data.size.1)?;

    for f in event_data.matrix {
        wtr.write_f32(f)?;
    }

    Ok(())
}

fn write_modify_map(mut wtr: impl Write, event_data: &ModifyMap) -> PResult<()> {
    wtr.write_u32(event_data.pos.0)?;
    wtr.write_u32(event_data.pos.1)?;
    wtr.write_u32(event_data.size.0)?;
    wtr.write_u32(event_data.size.1)?;

    for (x, y) in &event_data.map {
        wtr.write_u32(*x)?;
        wtr.write_u32(*y)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    fn verify_write(input: CanvasEvent, output: &[u8]) {
        // STEP 1: Write
        let len = event_len(&input);
        let mut wtr = Cursor::new(vec![0; len]);
        write_event(&mut wtr, &input).expect("write error");

        // STEP 2: Assert length
        let data = wtr.into_inner();
        assert_eq!(
            data.len(),
            len,
            "length error: output was {} bytes but expected {len} bytes",
            data.len()
        );

        // STEP 3: Assert contents
        assert_eq!(data, output, "write error: mismatched results");
    }

    #[test]
    fn test_canvas_meta() {
        let input = CanvasEvent::CanvasMeta(CanvasMeta {
            size: (32, 64),
            time_start: 96,
            time_end: None,
            name: String::from("test"),
            platform: String::from("testing!"),
        });
        let output = [
            32, 0, 0, 0, //                                     Width (32)
            64, 0, 0, 0, //                                     Height (64)
            96, 0, 0, 0, 0, 0, 0, 0, //                         Time Start (Some(96 ms))
            0, 0, 0, 0, 0, 0, 0, 0, //                          Time End (None)
            4, 0, //                                            Name Len (4)
            b't', b'e', b's', b't', //                          Name
            8, 0, //                                            Platform Name Len (8)
            b't', b'e', b's', b't', b'i', b'n', b'g', b'!', //  Platform Name
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_canvas_modify() {
        let input = CanvasEvent::CanvasModify(CanvasModify { size: (96, 64) });
        let output = [
            96, 0, 0, 0, //                                     Width (32)
            64, 0, 0, 0, //                                     Height (64)
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_palette_chunk() {
        let input = CanvasEvent::PaletteChunk(PaletteChunk {
            offset: 96,
            colors: vec![[255, 255, 255, 255], [0, 0, 0, 255]],
        });
        let output = [
            96, 0, 0, 0, 0, 0, 0, 0, // Width (96)
            255, 255, 255, 255, //      Colors[0]
            0, 0, 0, 255, //            Colors[1]
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_placement() {
        let input = CanvasEvent::Placement(Placement {
            pos: (24, 32),
            time: 64,
            color_index: 8,
        });
        let output = [
            24, 0, 0, 0, //             X (24)
            32, 0, 0, 0, //             Y (32)
            64, 0, 0, 0, 0, 0, 0, 0, // Time Start (96)
            8, 0, //                    Color (8)
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_placement_chunk() {
        let input = CanvasEvent::PlacementChunk(PlacementChunk {
            pos: (24, 32),
            time: 64,
            color_indexes: vec![8, 15, 2],
        });
        let output = [
            24, 0, 0, 0, //             X (24)
            32, 0, 0, 0, //             Y (32)
            64, 0, 0, 0, 0, 0, 0, 0, // Time Start (96)
            8, 0, //                    Color (8)
            15, 0, //                   Color (15)
            2, 0, //                    Color (2)
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_meta_id() {
        let input = CanvasEvent::MetaId(MetaId::Numerical(u32::to_le_bytes(1234).to_vec()));
        let output = [
            0, //                       Layout
            0xD2, 0x04, 0x00, 0x00, //  ID
        ];
        verify_write(input, &output);
    }

    #[test]
    fn test_modify_matrix() {
        let input = CanvasEvent::ModifyMatrix(ModifyMatrix {
            pos: (24, 32),
            size: (255, 128),
            matrix: [1.5, 3.0, 0.0, 0.0, -3.2, -128.0],
        });
        let output = [
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
        verify_write(input, &output);
    }

    #[test]
    fn test_modify_map() {
        let input = CanvasEvent::ModifyMap(ModifyMap {
            pos: (24, 32),
            size: (1, 2),
            map: vec![(0x32, 0x64), (0x04, 0x06)],
        });
        let output = [
            24, 0, 0, 0, //           X (24)
            32, 0, 0, 0, //           Y (32)
            1, 0, 0, 0, //            Width (24)
            2, 0, 0, 0, //            Height (32)
            0x32, 0x00, 0x00, 0x00, //  X1
            0x64, 0x00, 0x00, 0x00, //  Y1
            0x04, 0x00, 0x00, 0x00, //  X2
            0x06, 0x00, 0x00, 0x00, //  Y2
        ];
        verify_write(input, &output);
    }
}
