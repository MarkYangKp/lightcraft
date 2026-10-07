//! Compile translated format strings so Rust checks every language's placeholders.
use std::{collections::BTreeMap, env, fs, path::PathBuf};

/// One entry per non-English language: the `Language` enum variant and its formats catalog.
const CATALOGS: &[(&str, &str)] = &[("Ja", "locales/ja-formats.json"), ("Zh", "locales/zh-formats.json")];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut loaded: Vec<(&str, BTreeMap<String, String>)> = Vec::new();
    for (variant, file) in CATALOGS {
        println!("cargo:rerun-if-changed={file}");
        loaded.push((variant, serde_json::from_str(&fs::read_to_string(file)?)?));
    }
    // Every catalog translates the same English formats, so no language silently falls back.
    for (variant, messages) in &loaded[1..] {
        if messages.len() != loaded[0].1.len() || messages.keys().ne(loaded[0].1.keys()) {
            let missing: Vec<_> = loaded[0].1.keys().filter(|k| !messages.contains_key(*k)).take(3).collect();
            return Err(format!("{variant}: formats must translate the same English strings (missing e.g. {missing:?})").into());
        }
    }
    let mut source = String::from("macro_rules! tr_format {\n");
    for english in loaded[0].1.keys() {
        let en = serde_json::to_string(english)?;
        let arms = CATALOGS
            .iter()
            .zip(&loaded)
            .map(|((variant, _), (_, messages))| {
                let translated = serde_json::to_string(messages.get(english).ok_or("verified above: key present")?)?;
                Ok::<_, Box<dyn std::error::Error>>(format!(" $crate::i18n::Language::{variant} => format!({translated} $(, $($args)*)?),"))
            })
            .collect::<Result<String, _>>()?;
        source.push_str(&format!(
            "({en} $(, $($args:tt)*)?) => {{ match $crate::i18n::language() {{ {arms} $crate::i18n::Language::En => format!({en} $(, $($args)*)?) }} }};\n"
        ));
    }
    source.push_str("}\npub(crate) use tr_format;\n");
    fs::write(PathBuf::from(env::var("OUT_DIR")?).join("tr-formats.rs"), source)?;
    Ok(())
}
