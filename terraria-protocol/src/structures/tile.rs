use crate::{Deserializable, Serializable, SliceCursor};

/// Tiles for which the frame is considered "important".
const TILE_FRAME_IMPORTANT: [bool; 623] = [
    false, false, false, true, true, true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, false, true, true, true, true, false, true, false, true, true, true, true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, true, false, false, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, false, true, true, false, false, false, false, false, false, false, false, false, false, true, true, false, true, true, false, false, true, true, true, true, true, true, true, true, false, true, true, true, true, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, true, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, true, true, false, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, true, false, false, true, false, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, false, false, true, true, false, false, false, false, false, false, false, false, false, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, false, false, false, false, false, false, true, true, true, true, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, false, false, false, true, true, false, false, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true, true, true, true, true, false, false, false, true, true, false, false, false, true, false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, false, true, false, false, false, false, false, true, true, false, false, true, true, true, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true, true, false, false, false, false, true, false, false, false, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, true
];

#[derive(Debug, Clone)]
pub enum Liquid {
    Water,
    Honey,
    Lava,
}

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub wire: [bool; 4],
    pub tile_color: u8,
    pub wall_color: u8,
    pub ty: u16,
    pub frame: (i16, i16),
    pub wall: u16,
    pub liquid: Option<Liquid>,
    pub lava: bool,
    pub honey: bool,
    pub half_brick: bool,
    pub slope: u8,
    pub actuator: bool,
    pub inactive: bool,
}

impl Tile {
    pub fn is_important(&self) -> bool {
        TILE_FRAME_IMPORTANT[self.ty as usize]
    }
}

impl Serializable for Tile {
    fn serialize(&self, _cursor: &mut SliceCursor) {
        todo!()
    }
}

impl Deserializable for Tile {
    fn deserialize(cursor: &mut SliceCursor) -> Self {
        todo!()
        /*
        let flags: [u8; 2] = [cursor.read(), cursor.read()];
        let wires: [bool; 4] = [
            flags[0] & 0x10 != 0,
            flags[1] & 0x01 != 0,
            flags[1] & 0x02 != 0,
            false,
        ];
        let slopes: [bool; 3] = [
            flags[1] & 0x10 != 0,
            flags[1] & 0x20 != 0,
            flags[1] & 0x40 != 0,
        ];
        let tile_color = if flags[1] & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        }; // u8
        let wall_color = if flags[1] & 0x08 != 0 {
            Some(cursor.read())
        } else {
            None
        }; // u8
        let ty = if flags[0] & 0x01 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let frame = if let Some(ty) = ty {
            if TILE_FRAME_IMPORTANT[ty as usize - 1] {
                Some((cursor.read(), cursor.read()))
            } else {
                None
            }
        } else {
            None
        };

        let wall = if flags[0] & 0x04 != 0 {
            Some(cursor.read())
        } else {
            None
        };
        let liquid = if flags[0] & 0x08 != 0 {
            Some((cursor.read(), cursor.read()))
        } else {
            None
        };

        let is_half = flags[0] & 0x20 != 0;
        let is_actuator = flags[0] & 0x40 != 0;
        let inactive = flags[0] & 0x80 != 0;

        dbg!(Self {
            wires,
            slopes,
            tile_color,
            wall_color,
            ty,
            frame,
            wall,
            liquid,
            is_half,
            is_actuator,
            inactive,
        })
        */
    }
}
