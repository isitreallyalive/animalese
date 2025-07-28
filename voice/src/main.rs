use bincode::{config, encode_to_vec};
use std::{env, fs, path::PathBuf};
use voice::{HEADER, Metadata};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("voices");

    for game_entry in fs::read_dir(&root)? {
        let game_file = game_entry?;
        if !game_file.file_type()?.is_dir() {
            continue;
        }
        let game = game_file
            .file_name()
            .to_string_lossy()
            .parse()
            .unwrap_or_default();

        for lang_entry in fs::read_dir(game_file.path())? {
            let lang_file = lang_entry?;
            if !lang_file.file_type()?.is_dir() {
                continue;
            }
            let language = lang_file
                .file_name()
                .to_string_lossy()
                .parse()
                .unwrap_or_default();

            for voice_entry in fs::read_dir(lang_file.path())? {
                let voice_file = voice_entry?;
                if !voice_file.file_type()?.is_dir() {
                    continue;
                }
                let voice = voice_file.file_name().to_string_lossy().to_string();

                let mut buf = Vec::from(HEADER);
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
                    &buf,
                )?;

                println!("{:?}", voice::decode(&buf));
            }
        }
    }

    Ok(())
}
