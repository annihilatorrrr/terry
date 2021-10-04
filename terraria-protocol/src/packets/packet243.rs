use crate::serde::{PacketBody, Result, SliceCursor};
use crate::utils::HexVec;

/// No description known yet.
///
/// Direction: ???.
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Packet243 {
    pub buf: HexVec,
}

impl PacketBody for Packet243 {
    const TAG: u8 = 243;

    fn write_body(&self, cursor: &mut SliceCursor) -> Result<()> {
        for b in self.buf.0.iter() {
            cursor.write(b)?;
        }
        Ok(())
    }

    fn from_body(cursor: &mut SliceCursor) -> Result<Self> {
        Ok(Self {
            buf: HexVec(cursor.read_to_end().to_vec()),
        })
    }
}
