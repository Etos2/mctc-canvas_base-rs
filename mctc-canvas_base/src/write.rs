use std::io::Write;

use crate::{error::PResult, util::WriteExt, *};

pub fn event_length(event: CanvasEvent) -> PResult<usize> {
    match event {
        CanvasEvent::CanvasMeta(canvas_meta) => todo!(),
        CanvasEvent::CanvasModify(canvas_modify) => todo!(),
        CanvasEvent::PaletteChunk(palette_chunk) => todo!(),
        CanvasEvent::Placement(placement) => todo!(),
        CanvasEvent::PlacementQuiet(placement) => todo!(),
        CanvasEvent::PlacementChunk(placement_chunk) => todo!(),
        CanvasEvent::PlacementChunkQuiet(placement_chunk) => todo!(),
        CanvasEvent::MetaId(meta_id) => todo!(),
        CanvasEvent::MetaUniqueId(meta_id) => todo!(),
        CanvasEvent::ModifyMatrix(modify_matrix) => todo!(),
        CanvasEvent::ModifyMap(modify_map) => todo!(),
    }
}

pub fn write_event(wtr: impl Write, event: CanvasEvent) -> PResult<()> {
    match event {
        CanvasEvent::CanvasMeta(canvas_meta) => write_canvas_meta(wtr, canvas_meta),
        CanvasEvent::CanvasModify(canvas_modify) => write_canvas_modify(wtr, canvas_modify),
        CanvasEvent::PaletteChunk(palette_chunk) => write_palette_chunk(wtr, palette_chunk),
        CanvasEvent::Placement(placement) => todo!(),
        CanvasEvent::PlacementQuiet(placement) => todo!(),
        CanvasEvent::PlacementChunk(placement_chunk) => todo!(),
        CanvasEvent::PlacementChunkQuiet(placement_chunk) => todo!(),
        CanvasEvent::MetaId(meta_id) => todo!(),
        CanvasEvent::MetaUniqueId(meta_id) => todo!(),
        CanvasEvent::ModifyMatrix(modify_matrix) => todo!(),
        CanvasEvent::ModifyMap(modify_map) => todo!(),
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

fn write_canvas_meta(mut wtr: impl Write, event_data: CanvasMeta) -> PResult<()> {
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

fn write_canvas_modify(mut wtr: impl Write, event_data: CanvasModify) -> PResult<()> {
    wtr.write_u32(event_data.size.0)?;
    wtr.write_u32(event_data.size.1)?;

    Ok(())
}

fn write_palette_chunk(mut wtr: impl Write, event_data: PaletteChunk) -> PResult<()> {
    wtr.write_u64(event_data.offset)?;
    wtr.write_all(cast_bytes(&event_data.colors))?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_canvas_meta() {
        let input = CanvasMeta {
            size: (32, 64),
            time_start: 96,
            time_end: None,
            name: String::from("test"),
            platform: String::from("testing!"),
        };
        let mut wtr = Cursor::new([0; 40]);
        let result = write_canvas_meta(&mut wtr, input);
        assert!(result.is_ok(), "write error: {:?}", result);
        assert_eq!(wtr.into_inner(), [
            32, 0, 0, 0, //                                     Width (32)
            64, 0, 0, 0, //                                     Height (64)
            96, 0, 0, 0, 0, 0, 0, 0, //                         Time Start (Some(96 ms))
            0, 0, 0, 0, 0, 0, 0, 0, //                          Time End (None)
            4, 0, //                                            Name Len (4)
            b't', b'e', b's', b't', //                          Name
            8, 0, //                                            Platform Name Len (8)
            b't', b'e', b's', b't', b'i', b'n', b'g', b'!', //  Platform Name
        ])
    }

    #[test]
    fn test_canvas_modify() {
        let input = CanvasModify { size: (96, 64) };
        let mut wtr = Cursor::new([0; 8]);
        let result = write_canvas_modify(&mut wtr, input);
        assert!(result.is_ok(), "write error: {:?}", result);
        assert_eq!(wtr.into_inner(), [
            96, 0, 0, 0, //                                     Width (32)
            64, 0, 0, 0, //                                     Height (64)
        ])
    }

    #[test]
    fn test_palette_chunk() {
        let input = PaletteChunk {
            offset: 96,
            colors: vec![[255, 255, 255, 255], [0, 0, 0, 255]],
        };
        let mut wtr = Cursor::new([0; 16]);
        let result = write_palette_chunk(&mut wtr, input);
        assert!(result.is_ok(), "write error: {:?}", result);
        assert_eq!(wtr.into_inner(), [
            96, 0, 0, 0, 0, 0, 0, 0, // Width (96)
            255, 255, 255, 255, //      Colors[0]
            0, 0, 0, 255, //            Colors[1]
        ])
    }
}
