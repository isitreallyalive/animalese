#[repr(C)]
struct Header {
    magic: [u8; 4], // AVF
    version: u8,
}

#[repr(C)]
enum Game {
    AnimalCrossing,
}

#[repr(C)]
enum Language {
    Japanese,
}

#[repr(C)]
struct Metadata {
    game: Game,
    language: Language,
}

// #[repr(C)]
// enum SfxId {
//     Exclamation
// }

#[repr(C)]
enum VoiceEntryKind {
    Mora {
        /// Consonant sound, if any. For example, 'k' in 'kyu'
        consonant: Option<u8>,
        /// Glide sound, if any. For example, 'y' in 'kyu'
        glide: Option<u8>,
        /// Vowel sound, for example 'u' in 'kyu'
        vowel: u8,
    },
    // Sfx(SfxId)
}

#[repr(C)]
struct Entry {
    data: VoiceEntryKind,
    /// Byte offset where this entry's audio data starts
    offset: u64,
    /// Length in bytes of this entry's audio data
    length: u32,
}
