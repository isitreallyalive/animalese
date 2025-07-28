use bincode::{config, encode_to_vec};
use std::{env, fs, path::PathBuf};
use voice::{Game, HEADER, Language, Metadata};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("voices");
    let assets = fs::read_dir(&root)?;

    let mut buf = Vec::from(HEADER);
    let mut game: Option<Game> = None;
    let mut language: Option<Language> = None;
    let mut voice: Option<String> = None;

    for file in assets {
        let game_file = file?;
        game = game_file.file_name().to_string_lossy().parse().ok();
        let files = fs::read_dir(game_file.path())?;
        for file in files {
            let lang_file = file?;
            language = lang_file.file_name().to_string_lossy().parse().ok();
            let files = fs::read_dir(lang_file.path())?;
            for file in files {
                let voice_file = file?;
                voice = Some(voice_file.file_name().to_string_lossy().to_string());
                // let files = fs::read_dir(voice_file.path())?;
                // for file in files {}
            }
        }
    }

    // metadata
    let game = game.unwrap_or_default();
    let language = language.unwrap_or_default();
    let voice = voice.unwrap_or_default();
    buf.extend(encode_to_vec(
        Metadata {
            game,
            language,
            voice: voice.clone(),
        },
        config::standard(),
    )?);

    fs::write(
        root.join(game.as_ref())
            .join(language.as_ref())
            .join(format!("{}.bin", voice.to_lowercase())),
        buf,
    )?;

    Ok(())
}
