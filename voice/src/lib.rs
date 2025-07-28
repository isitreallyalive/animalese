use bincode::Encode;
use strum::{AsRefStr, EnumString};

pub const HEADER: [u8; 5] = [
    b'A', b'C', b'V', b'F', // magic
    0x01, // version
];

#[repr(C)]
#[derive(AsRefStr, Clone, Copy, Debug, Default, Encode, EnumString)]
pub enum Game {
    #[strum(serialize = "ac")]
    AnimalCrossing,
    #[default]
    #[strum(disabled)]
    Unknown,
}

#[repr(C)]
#[derive(AsRefStr, Clone, Copy, Debug, Default, Encode, EnumString)]
pub enum Language {
    #[strum(serialize = "jp")]
    Japanese,
    #[default]
    #[strum(disabled)]
    Unknown,
}

#[repr(C)]
#[derive(Debug, Encode)]
pub struct Metadata {
    pub game: Game,
    pub language: Language,
    pub voice: String,
}

#[repr(C)]
#[derive(Encode)]
pub enum VoiceEntryKind {
    Mora([u8; 3]), // unused slots can be '\0'
}

#[repr(C)]
#[derive(Encode)]
pub struct Entry {
    data: VoiceEntryKind,
    /// Byte offset where this entry's audio data starts
    offset: u64,
    /// Length in bytes of this entry's audio data
    length: u32,
}

struct VoiceFont {
    metadata: Metadata,
    entries: Vec<Entry>,
}
