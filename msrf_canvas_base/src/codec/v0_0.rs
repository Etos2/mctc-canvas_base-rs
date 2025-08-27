use std::num::NonZeroU32;

use msrf_io::{ByteStream, MutByteStream, RecordSerialise};

use super::Error;
use crate::{
    CanvasMeta, CanvasRecord, PaletteInsert, PaletteRemove, PlacementInsert, PlacementInsertFill,
    PlacementRemove, PlacementRemoveFill,
};

pub struct Serialiser;

// TODO: Multiple insert, remove, etc
impl RecordSerialise for Serialiser {
    type Err = Error;

    type Record = CanvasRecord;

    fn deserialise_record(&self, id: u16, value: &[u8]) -> Result<Self::Record, Self::Err> {
        match id {
            crate::CANVAS_META_TYPE_ID => des_canvas_meta(value).map(Self::Record::from),
            crate::PALETTE_INSERT_TYPE_ID => des_palette_insert(value).map(Self::Record::from),
            crate::PALETTE_REMOVE_TYPE_ID => des_palette_remove(value).map(Self::Record::from),
            crate::PLACEMENT_INSERT_TYPE_ID => des_placement_insert(value).map(Self::Record::from),
            crate::PLACEMENT_INSERT_SILENT_TYPE_ID => {
                des_placement_insert(value).map(CanvasRecord::PlacementInsertQuiet)
            }
            crate::PLACEMENT_INSERT_FILL_TYPE_ID => {
                des_placement_insert_fill(value).map(Self::Record::from)
            }
            crate::PLACEMENT_INSERT_FILL_SILENT_TYPE_ID => {
                des_placement_insert_fill(value).map(CanvasRecord::PlacementInsertFillQuiet)
            }
            crate::PLACEMENT_REMOVE_TYPE_ID => des_placement_remove(value).map(Self::Record::from),
            crate::PLACEMENT_REMOVE_SILENT_TYPE_ID => {
                des_placement_remove(value).map(CanvasRecord::PlacementRemoveQuiet)
            }
            crate::PLACEMENT_REMOVE_FILL_TYPE_ID => {
                des_placement_remove_fill(value).map(Self::Record::from)
            }
            crate::PLACEMENT_REMOVE_FILL_SILENT_TYPE_ID => {
                des_placement_remove_fill(value).map(CanvasRecord::PlacementRemoveFillQuiet)
            }
            crate::IDENTIFIER_NUMERIC_TYPE_ID => {
                des_identify_numeric(value).map(CanvasRecord::IdentifierNumeric)
            }
            crate::IDENTIFIER_STRING_TYPE_ID => {
                des_identify_string(value).map(CanvasRecord::IdentifierString)
            }
            crate::IDENTIFIER_SECRET_TYPE_ID => {
                des_identify_secret(value).map(CanvasRecord::IdentifierSecret)
            }
            _ => return Err(Error::UnexpectedType(id)),
        }
    }

    fn serialise_record(
        &self,
        value: &mut [u8],
        record: &Self::Record,
    ) -> Result<usize, Self::Err> {
        match record {
            CanvasRecord::CanvasMeta(canvas_meta) => ser_canvas_meta(value, canvas_meta),
            CanvasRecord::PaletteInsert(palette_insert) => {
                ser_palette_insert(value, palette_insert)
            }
            CanvasRecord::PaletteRemove(palette_remove) => {
                ser_palette_remove(value, palette_remove)
            }
            CanvasRecord::PlacementInsert(placement_insert)
            | CanvasRecord::PlacementInsertQuiet(placement_insert) => {
                ser_placement_insert(value, placement_insert)
            }
            CanvasRecord::PlacementInsertFill(placement_insert_fill)
            | CanvasRecord::PlacementInsertFillQuiet(placement_insert_fill) => {
                ser_placement_insert_fill(value, placement_insert_fill)
            }
            CanvasRecord::PlacementRemove(placement_remove)
            | CanvasRecord::PlacementRemoveQuiet(placement_remove) => {
                ser_placement_remove(value, placement_remove)
            }
            CanvasRecord::PlacementRemoveFill(placement_remove_fill)
            | CanvasRecord::PlacementRemoveFillQuiet(placement_remove_fill) => {
                ser_placement_remove_fill(value, placement_remove_fill)
            }
            CanvasRecord::IdentifierNumeric(n) => ser_identify_numeric(value, *n),
            CanvasRecord::IdentifierString(s) => ser_identify_string(value, s),
            CanvasRecord::IdentifierSecret(raw) => ser_identify_secret(value, raw),
        }
    }
}

fn ser_canvas_meta(buf: &mut [u8], record: &CanvasMeta) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    let name_len = record.name.len();
    buf.insert_u8(
        name_len
            .try_into()
            .map_err(|_| Error::InvalidField(name_len.to_le_bytes().to_vec()))?,
    )?;
    buf.insert(record.name.as_bytes())?;

    let platform_name_len = record.platform.len();
    buf.insert_u8(
        platform_name_len
            .try_into()
            .map_err(|_| Error::InvalidField(platform_name_len.to_le_bytes().to_vec()))?,
    )?;

    buf.insert(record.platform.as_bytes())?;
    buf.insert_u64(record.time)?;
    buf.insert_u32(record.size.0)?;
    buf.insert_u32(record.size.1)?;

    Ok(len - buf.len())
}

fn des_canvas_meta(buf: &[u8]) -> Result<CanvasMeta, Error> {
    let mut buf = buf;

    let name_len = buf.extract_u8()? as usize;
    let name = str::from_utf8(buf.extract(name_len)?)?.to_string();
    let platform_len = buf.extract_u8()? as usize;
    let platform = str::from_utf8(buf.extract(platform_len)?)?.to_string();
    let time = buf.extract_u64()?;
    let size = (buf.extract_u32()?, buf.extract_u32()?);

    Ok(CanvasMeta {
        name,
        platform,
        time,
        size,
    })
}

fn ser_palette_insert(buf: &mut [u8], record: &PaletteInsert) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u32(record.offset)?;
    for color in &record.colors {
        buf.insert(color)?;
    }

    Ok(len - buf.len())
}

fn des_palette_insert(buf: &[u8]) -> Result<PaletteInsert, Error> {
    let mut buf = buf;

    let offset = buf.extract_u32()?;
    let mut colors = Vec::with_capacity(buf.len() / 4);
    while !buf.is_empty() {
        // Safety: extract(4) always returns a slice of len == 4
        colors.push(buf.extract(4)?.try_into().unwrap());
    }

    Ok(PaletteInsert { offset, colors })
}

fn ser_palette_remove(buf: &mut [u8], record: &PaletteRemove) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u32(record.offset)?;
    if record.length.get() > 1 {
        buf.insert_u32(record.length.get())?;
    }

    Ok(len - buf.len())
}

fn des_palette_remove(buf: &[u8]) -> Result<PaletteRemove, Error> {
    let mut buf = buf;

    let offset = buf.extract_u32()?;
    let length = buf.extract_u32().unwrap_or(1);
    let length =
        NonZeroU32::new(length).ok_or(Error::InvalidField(length.to_le_bytes().to_vec()))?;

    Ok(PaletteRemove { offset, length })
}

fn ser_placement_insert(buf: &mut [u8], record: &PlacementInsert) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u64(record.time)?;
    buf.insert_u64(record.pos)?;
    buf.insert_u32(record.col)?;

    Ok(len - buf.len())
}

fn des_placement_insert(buf: &[u8]) -> Result<PlacementInsert, Error> {
    let mut buf = buf;

    let time = buf.extract_u64()?;
    let pos = buf.extract_u64()?;
    let col = buf.extract_u32()?;

    Ok(PlacementInsert { time, pos, col })
}

fn ser_placement_insert_fill(buf: &mut [u8], record: &PlacementInsertFill) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u64(record.time)?;
    buf.insert_u64(record.pos.0)?;
    buf.insert_u64(record.pos.1)?;
    buf.insert_u32(record.col)?;

    Ok(len - buf.len())
}

fn des_placement_insert_fill(buf: &[u8]) -> Result<PlacementInsertFill, Error> {
    let mut buf = buf;

    let time = buf.extract_u64()?;
    let pos = (buf.extract_u64()?, buf.extract_u64()?);
    let col = buf.extract_u32()?;

    Ok(PlacementInsertFill { time, pos, col })
}

fn ser_placement_remove(buf: &mut [u8], record: &PlacementRemove) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u64(record.time)?;
    buf.insert_u64(record.pos)?;

    Ok(len - buf.len())
}

fn des_placement_remove(buf: &[u8]) -> Result<PlacementRemove, Error> {
    let mut buf = buf;

    let time = buf.extract_u64()?;
    let pos = buf.extract_u64()?;

    Ok(PlacementRemove { time, pos })
}

fn ser_placement_remove_fill(buf: &mut [u8], record: &PlacementRemoveFill) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;

    buf.insert_u64(record.time)?;
    buf.insert_u64(record.pos.0)?;
    buf.insert_u64(record.pos.1)?;

    Ok(len - buf.len())
}

fn des_placement_remove_fill(buf: &[u8]) -> Result<PlacementRemoveFill, Error> {
    let mut buf = buf;

    let time = buf.extract_u64()?;
    let pos = (buf.extract_u64()?, buf.extract_u64()?);

    Ok(PlacementRemoveFill { time, pos })
}

fn ser_identify_numeric(buf: &mut [u8], record: u64) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;
    buf.insert_u64(record)?;

    Ok(len - buf.len())
}

fn des_identify_numeric(buf: &[u8]) -> Result<u64, Error> {
    let mut buf = buf;
    buf.extract_u64().map_err(Error::from)
}

fn ser_identify_string(buf: &mut [u8], record: &str) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;
    buf.insert(record.as_bytes())?;

    Ok(len - buf.len())
}

// TODO: Borrow?
fn des_identify_string(buf: &[u8]) -> Result<String, Error> {
    let mut buf = buf;
    Ok(str::from_utf8(buf.extract(buf.len())?)?.to_string())
}

fn ser_identify_secret(buf: &mut [u8], record: &[u8]) -> Result<usize, Error> {
    let len = buf.len();
    let mut buf = buf;
    buf.insert(record)?;

    Ok(len - buf.len())
}

// TODO: Borrow?
fn des_identify_secret(buf: &[u8]) -> Result<Vec<u8>, Error> {
    let mut buf = buf;
    Ok(buf.extract(buf.len())?.to_vec())
}

#[cfg(test)]
mod test {
    use crate::{PALETTE_REMOVE_TYPE_ID};

    use super::*;

    fn serdes_harness(sample: CanvasRecord, raw: &[u8]) {
        let serialiser = Serialiser;
        let record = serialiser
            .deserialise_record(sample.raw_id(), raw)
            .expect("failed deserialise");

        assert_eq!(record, sample);

        let mut buf = vec![0; raw.len()];
        let written = serialiser
            .serialise_record(buf.as_mut_slice(), &record)
            .expect("failed serialise");

        assert_eq!(written, buf.len());
        assert_eq!(buf, raw);
    }

    fn ser_harness_err(expected_len: usize, sample: CanvasRecord, err: Error) {
        let serialiser = Serialiser;
        let mut buf = vec![0; expected_len];
        let des_err = serialiser
            .serialise_record(buf.as_mut_slice(), &sample)
            .expect_err("succeeded serialise unexpectedly");

        assert_eq!(des_err, err);
    }

    fn des_harness_err(id: u16, raw: &[u8], err: Error) {
        let serialiser = Serialiser;
        let ser_err = serialiser
            .deserialise_record(id, raw)
            .expect_err("succeeded deserialise unexpectedly");

        assert_eq!(ser_err, err);
    }

    #[test]
    fn codec_canvas_meta() {
        // Typical
        serdes_harness(
            CanvasRecord::CanvasMeta(CanvasMeta {
                name: "test".to_string(),
                platform: "pxls.space".to_string(),
                time: 1234,
                size: (512, 256),
            }),
            constcat::concat_bytes!(
                &[4u8],                   // Name Len
                b"test".as_slice(),       // Name
                &[10u8],                  // Platform Name Len
                b"pxls.space".as_slice(), // Platform Name
                &1234u64.to_le_bytes(),   // Time
                &512u32.to_le_bytes(),    // Size.0
                &256u32.to_le_bytes(),    // Size.1
            ),
        );
        // Illegal name lengths
        ser_harness_err(
            288,
            CanvasRecord::CanvasMeta(CanvasMeta {
                name: String::from_utf8(vec![0; 256]).unwrap(),
                platform: "pxls.space".to_string(),
                time: 1234,
                size: (512, 256),
            }),
            Error::InvalidField(256usize.to_le_bytes().to_vec()),
        );
        ser_harness_err(
            289,
            CanvasRecord::CanvasMeta(CanvasMeta {
                name: "test".to_string(),
                platform: String::from_utf8(vec![0; 257]).unwrap(),
                time: 1234,
                size: (512, 256),
            }),
            Error::InvalidField(257usize.to_le_bytes().to_vec()),
        );
    }

    #[test]
    fn codec_palette_insert() {
        const COLORS: &[[u8; 4]] = &[
            [0x00, 0x00, 0x00, 0x00], // colors[0]
            [0xFF, 0xFF, 0xFF, 0xFF], // colors[1]
            [0x7F, 0x7F, 0x7F, 0xFF], // colors[2]
            [0x00, 0x00, 0x00, 0xFF], // colors[3]
        ];
        serdes_harness(
            CanvasRecord::PaletteInsert(PaletteInsert {
                offset: 16,
                colors: COLORS.to_vec(),
            }),
            constcat::concat_bytes!(
                &16u32.to_le_bytes(), // Offset
                COLORS.as_flattened()
            ),
        );
    }

    #[test]
    fn codec_palette_remove() {
        // Long form
        serdes_harness(
            CanvasRecord::PaletteRemove(PaletteRemove {
                offset: 16,
                length: NonZeroU32::new(32).unwrap(),
            }),
            constcat::concat_bytes!(
                &16u32.to_le_bytes(), // Offset
                &32u32.to_le_bytes(), // Length
            ),
        );
        // Short form
        serdes_harness(
            CanvasRecord::PaletteRemove(PaletteRemove {
                offset: 16,
                length: NonZeroU32::new(1).unwrap(),
            }),
            constcat::concat_bytes!(
                &16u32.to_le_bytes(), // Offset
            ),
        );
        // Invalid deserialise
        des_harness_err(
            PALETTE_REMOVE_TYPE_ID,
            constcat::concat_bytes!(
                &16u32.to_le_bytes(), // Offset
                &0u32.to_le_bytes(),  // Length (0 is invalid)
            ),
            Error::InvalidField(0u32.to_le_bytes().to_vec()),
        );
    }

    #[test]
    fn codec_placement_insert() {
        let inner = PlacementInsert {
            time: 1234,
            pos: 21,
            col: 5,
        };
        let raw = constcat::concat_bytes!(
            &1234u64.to_le_bytes(), // Time
            &21u64.to_le_bytes(),   // Position
            &5u32.to_le_bytes(),    // Color
        );

        serdes_harness(CanvasRecord::PlacementInsert(inner.clone()), raw);
        serdes_harness(CanvasRecord::PlacementInsertQuiet(inner), raw);
    }

    #[test]
    fn codec_placement_insert_fill() {
        let inner = PlacementInsertFill {
            time: 1234,
            pos: (21, 42),
            col: 5,
        };
        let raw = constcat::concat_bytes!(
            &1234u64.to_le_bytes(), // Time
            &21u64.to_le_bytes(),   // Position 1
            &42u64.to_le_bytes(),   // Position 2
            &5u32.to_le_bytes(),    // Color
        );

        serdes_harness(CanvasRecord::PlacementInsertFill(inner.clone()), raw);
        serdes_harness(CanvasRecord::PlacementInsertFillQuiet(inner), raw);
    }

    #[test]
    fn codec_placement_remove() {
        let inner = PlacementRemove {
            time: 1234,
            pos: 21,
        };
        let raw = constcat::concat_bytes!(
            &1234u64.to_le_bytes(), // Time
            &21u64.to_le_bytes(),   // Position
        );

        serdes_harness(CanvasRecord::PlacementRemove(inner.clone()), raw);
        serdes_harness(CanvasRecord::PlacementRemoveQuiet(inner), raw);
    }

    #[test]
    fn codec_placement_remove_fill() {
        let inner = PlacementRemoveFill {
            time: 1234,
            pos: (21, 42),
        };
        let raw = constcat::concat_bytes!(
            &1234u64.to_le_bytes(), // Time
            &21u64.to_le_bytes(),   // Position 1
            &42u64.to_le_bytes(),   // Position 2
        );

        serdes_harness(CanvasRecord::PlacementRemoveFill(inner.clone()), raw);
        serdes_harness(CanvasRecord::PlacementRemoveFillQuiet(inner), raw);
    }

    #[test]
    fn codec_identifier() {
        serdes_harness(
            CanvasRecord::IdentifierNumeric(1234u64),
            constcat::concat_bytes!(&1234u64.to_le_bytes()),
        );
        serdes_harness(
            CanvasRecord::IdentifierString("Etos2".to_string()),
            constcat::concat_bytes!(b"Etos2".as_slice()),
        );
        let secret = vec![42u8; 256];
        serdes_harness(
            CanvasRecord::IdentifierSecret(secret.clone()),
            secret.as_slice(),
        );
    }
}
