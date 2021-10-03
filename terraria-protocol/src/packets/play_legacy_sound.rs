use crate::packets::packet_struct;
use crate::structures::{serializable_enum, Vec2};

serializable_enum! {
    pub enum SoundMode: u8 {
        Style = 1,
        VolumeScale = 2,
        PitchOffset = 3,
    }
}

packet_struct! {
    /// Play a legacy sound.
    ///
    /// Direction: Server -> Client.
    pub struct PlayLegacySound {
        const TAG = 132;

        pub pos: Vec2,
        pub sound_id: u16,
        pub mode: SoundMode,
    }
}
