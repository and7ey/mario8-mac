// Mush8 - set.rs
// Constantin Clerc (c22dev) - 0.1
use crate::ipc::{Request, Response, Service, Stub, write_buffer};

const LANGUAGES: [&str; 18] =
    ["ja", "en-US", "fr", "de", "it", "es", "zh-CN", "ko", "nl", "pt", "ru", "zh-TW", "en-GB", "fr-CA", "es-419", "zh-Hans", "zh-Hant", "pt-BR"];

pub fn code(lang: &str) -> u64 {
    let mut b = [0u8; 8];
    b[..lang.len()].copy_from_slice(lang.as_bytes());
    u64::from_le_bytes(b)
}

pub fn language() -> &'static str {
    static LANG: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    LANG.get_or_init(|| {
        #[cfg(not(target_os = "android"))]
        let tag = std::env::var("NX_LANGUAGE").ok().or_else(|| objc2_foundation::NSLocale::preferredLanguages().firstObject().map(|s| s.to_string())).unwrap_or_default();
        #[cfg(target_os = "android")]
        let tag = std::env::var("NX_LANGUAGE").unwrap_or_else(|_| "en-US".into());
        let lang = switch_language(&tag);
        eprintln!("[set] language {lang} (device: {tag})");
        lang
    })
}

fn switch_language(tag: &str) -> &'static str {
    let tag = tag.replace('_', "-");
    let (lang, rest) = tag.split_once('-').unwrap_or((&tag, ""));
    let region = rest.rsplit('-').next().unwrap_or("");
    match lang {
        "ja" => "ja",
        "en" if matches!(region, "US" | "CA" | "") => "en-US",
        "en" => "en-GB",
        "fr" if region == "CA" => "fr-CA",
        "fr" => "fr",
        "de" => "de",
        "it" => "it",
        "es" if matches!(region, "ES" | "") => "es",
        "es" => "es-419",
        "nl" => "nl",
        "pt" if region == "PT" => "pt",
        "pt" => "pt-BR",
        "ru" => "ru",
        "ko" => "ko",
        "zh" if rest.contains("Hant") || matches!(region, "TW" | "HK" | "MO") => "zh-Hant",
        "zh" => "zh-Hans",
        _ => "en-US",
    }
}

fn region() -> u32 {
    match language() {
        "ja" => 0,
        "en-US" | "fr-CA" | "es-419" | "pt-BR" => 1,
        "ko" | "zh-Hant" => 4,
        "zh-Hans" => 5,
        _ => 2,
    }
}

pub struct Settings;

impl Service for Settings {
    fn name(&self) -> String {
        "set".into()
    }
    fn call(&self, req: &Request, rsp: &mut Response) -> u32 {
        match req.cmd {
            0 => rsp.push_u64(code(language())),
            1 | 5 => {
                let n = if req.cmd == 1 { 15 } else { LANGUAGES.len() };
                let bytes: Vec<u8> = LANGUAGES[..n].iter().flat_map(|l| code(l).to_le_bytes()).collect();
                let written = req.out_buffer().map_or(0, |b| write_buffer(&b, &bytes) / 8);
                rsp.push_u32(written as u32);
            }
            2 => rsp.push_u64(LANGUAGES.get(req.u32(0) as usize).map_or(0, |l| code(l))),
            3 => rsp.push_u32(15),
            4 => rsp.push_u32(region()),
            6 => rsp.push_u32(LANGUAGES.len() as u32),
            _ => return Stub(self.name()).call(req, rsp),
        }
        0
    }
}
