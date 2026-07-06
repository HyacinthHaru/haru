use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::state::Mood;

/// Localized UI strings. Templates use `{placeholder}` tokens filled by render.
/// Sounds (meow/purr) are intentionally universal and live in `sounds.rs`.
#[derive(Debug, Deserialize)]
pub struct Messages {
    pub tip_line: String,
    pub unit_one: String,
    pub unit_other: String,
    pub patted: String,
    pub fed: String,
    pub status: String,
    pub never: String,
    pub mood_waiting: String,
    pub mood_happy: String,
    pub mood_purring: String,
    pub mood_spoiled: String,
}

impl Messages {
    pub fn mood(&self, mood: Mood) -> &str {
        match mood {
            Mood::Waiting => &self.mood_waiting,
            Mood::Happy => &self.mood_happy,
            Mood::Purring => &self.mood_purring,
            Mood::Spoiled => &self.mood_spoiled,
        }
    }

    pub fn unit(&self, n: u64) -> &str {
        if n == 1 {
            &self.unit_one
        } else {
            &self.unit_other
        }
    }
}

/// One embedded language, baked in at compile time. To add a language, drop
/// `assets/lang/<code>/{tips.txt, pat.txt, feed.txt, messages.json}` and add one
/// `Pack { ... }` entry below.
struct Pack {
    code: &'static str,
    tips: &'static str,
    pat: &'static str,
    feed: &'static str,
    messages: &'static str,
}

const PACKS: &[Pack] = &[
    Pack {
        code: "en",
        tips: include_str!("../assets/lang/en/tips.txt"),
        pat: include_str!("../assets/lang/en/pat.txt"),
        feed: include_str!("../assets/lang/en/feed.txt"),
        messages: include_str!("../assets/lang/en/messages.json"),
    },
    Pack {
        code: "zh",
        tips: include_str!("../assets/lang/zh/tips.txt"),
        pat: include_str!("../assets/lang/zh/pat.txt"),
        feed: include_str!("../assets/lang/zh/feed.txt"),
        messages: include_str!("../assets/lang/zh/messages.json"),
    },
    Pack {
        code: "ja",
        tips: include_str!("../assets/lang/ja/tips.txt"),
        pat: include_str!("../assets/lang/ja/pat.txt"),
        feed: include_str!("../assets/lang/ja/feed.txt"),
        messages: include_str!("../assets/lang/ja/messages.json"),
    },
];

const DEFAULT_CODE: &str = "en";

/// The resolved active language: parsed line pools and messages.
pub struct Lang {
    pub tips: Vec<&'static str>,
    pub pat_flavors: Vec<&'static str>,
    pub feed_flavors: Vec<&'static str>,
    pub messages: Messages,
}

impl Lang {
    pub fn random_tip(&self) -> &'static str {
        pick(&self.tips)
    }

    pub fn random_pat_flavor(&self) -> &'static str {
        pick(&self.pat_flavors)
    }

    pub fn random_feed_flavor(&self) -> &'static str {
        pick(&self.feed_flavors)
    }
}

fn pick(items: &[&'static str]) -> &'static str {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng).copied().unwrap_or("meow.")
}

/// Non-empty, non-comment lines from an embedded text pool.
fn parse_lines(raw: &'static str) -> Vec<&'static str> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect()
}

fn pack_for(code: &str) -> &'static Pack {
    PACKS
        .iter()
        .find(|p| p.code == code)
        .unwrap_or_else(|| PACKS.iter().find(|p| p.code == DEFAULT_CODE).unwrap())
}

/// Reduce a locale tag ("zh-CN", "zh_CN.UTF-8", "EN") to its lowercase primary
/// subtag ("zh", "zh", "en").
fn primary_subtag(locale: &str) -> String {
    locale
        .split(['-', '_', '.'])
        .next()
        .unwrap_or("")
        .to_lowercase()
}

/// Resolve the language code. Manual override wins; otherwise the system locale
/// (via sys-locale) is the source of truth; otherwise the default.
fn resolve_code(cli_lang: Option<&str>) -> String {
    if let Some(code) = cli_lang {
        return primary_subtag(code);
    }
    if let Ok(code) = std::env::var("HARU_LANG")
        && !code.trim().is_empty()
    {
        return primary_subtag(&code);
    }
    if let Some(locale) = sys_locale::get_locale() {
        return primary_subtag(&locale);
    }
    DEFAULT_CODE.to_string()
}

/// Resolve the active language, loading its embedded pack. An unknown code
/// falls back to the default language.
pub fn resolve(cli_lang: Option<&str>) -> Lang {
    let code = resolve_code(cli_lang);
    let pack = pack_for(&code);
    let messages: Messages =
        serde_json::from_str(pack.messages).expect("embedded messages.json must be valid");
    Lang {
        tips: parse_lines(pack.tips),
        pat_flavors: parse_lines(pack.pat),
        feed_flavors: parse_lines(pack.feed),
        messages,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_packs_parse_and_have_content() {
        for p in PACKS {
            serde_json::from_str::<Messages>(p.messages)
                .unwrap_or_else(|e| panic!("{} messages.json invalid: {e}", p.code));
            assert!(!parse_lines(p.tips).is_empty(), "{} has no tips", p.code);
            assert!(!parse_lines(p.pat).is_empty(), "{} has no pat flavors", p.code);
            assert!(!parse_lines(p.feed).is_empty(), "{} has no feed flavors", p.code);
        }
    }

    #[test]
    fn messages_have_required_placeholders() {
        for p in PACKS {
            let m: Messages = serde_json::from_str(p.messages).unwrap();
            assert!(m.tip_line.contains("{tip}"), "{}: tip_line", p.code);
            assert!(m.patted.contains("{n}") && m.patted.contains("{unit}"), "{}: patted", p.code);
            assert!(m.fed.contains("{n}") && m.fed.contains("{unit}"), "{}: fed", p.code);
            for token in ["{mood}", "{pats}", "{feeds}", "{total_pats}", "{total_feeds}", "{last_seen}"] {
                assert!(m.status.contains(token), "{}: status missing {token}", p.code);
            }
        }
    }

    #[test]
    fn primary_subtag_normalizes() {
        assert_eq!(primary_subtag("zh-CN"), "zh");
        assert_eq!(primary_subtag("zh_CN.UTF-8"), "zh");
        assert_eq!(primary_subtag("EN"), "en");
    }

    #[test]
    fn explicit_override_selects_language() {
        assert_eq!(resolve(Some("zh")).messages.mood_happy, "开心");
        assert_eq!(resolve(Some("en")).messages.mood_happy, "happy");
        assert_eq!(resolve(Some("ja")).messages.mood_happy, "ごきげん");
    }

    #[test]
    fn unknown_code_falls_back_to_default() {
        // An unknown code resolves to the default (English) pack.
        assert_eq!(resolve(Some("xx")).messages.mood_happy, "happy");
    }
}
