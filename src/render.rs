use crate::cats::{random_cat, Pose};
use crate::lang::Lang;
use crate::sounds::{random_sound, Sound};
use crate::state::State;

/// `haru` — summon the cat.
pub fn summon(lang: &Lang) -> String {
    let tip = lang.messages.tip_line.replace("{tip}", lang.random_tip());
    format!(
        "{}\n\n{}\n\n{}",
        random_sound(Sound::Summon),
        random_cat(Pose::Neutral),
        tip
    )
}

/// `haru --pat` — expects the state *after* the pat was applied.
/// Layout: sound, cat, a random flavor line, then the count line.
pub fn pat(lang: &Lang, state: &State) -> String {
    let count = lang
        .messages
        .patted
        .replace("{n}", &state.pats_today.to_string())
        .replace("{unit}", lang.messages.unit(state.pats_today));
    format!(
        "{}\n\n{}\n\n{}\n{}",
        random_sound(Sound::Pat),
        random_cat(Pose::Pat),
        lang.random_pat_flavor(),
        count
    )
}

/// `haru --feed` — expects the state *after* the feed was applied.
/// Layout: sound, cat, a random flavor line, then the count line.
pub fn feed(lang: &Lang, state: &State) -> String {
    let count = lang
        .messages
        .fed
        .replace("{n}", &state.feeds_today.to_string())
        .replace("{unit}", lang.messages.unit(state.feeds_today));
    format!(
        "{}\n\n{}\n\n{}\n{}",
        random_sound(Sound::Feed),
        random_cat(Pose::Feed),
        lang.random_feed_flavor(),
        count
    )
}

/// `haru --status` — a plain, scannable dashboard. No cat.
pub fn status(lang: &Lang, state: &State) -> String {
    let m = &lang.messages;
    let last_seen = if state.last_seen.is_empty() {
        m.never.as_str()
    } else {
        state.last_seen.as_str()
    };
    m.status
        .replace("{mood}", m.mood(state.mood()))
        .replace("{pats}", &state.pats_today.to_string())
        .replace("{feeds}", &state.feeds_today.to_string())
        .replace("{total_pats}", &state.total_pats.to_string())
        .replace("{total_feeds}", &state.total_feeds.to_string())
        .replace("{last_seen}", last_seen)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pat_line_pluralizes_in_english() {
        let lang = crate::lang::resolve(Some("en"));
        let one = State {
            pats_today: 1,
            ..State::default()
        };
        assert!(pat(&lang, &one).contains("1 time today"));
        let two = State {
            pats_today: 2,
            ..State::default()
        };
        assert!(pat(&lang, &two).contains("2 times today"));
    }

    #[test]
    fn status_fills_english_template() {
        let lang = crate::lang::resolve(Some("en"));
        let s = State {
            last_seen: "2026-07-06".into(),
            pats_today: 5,
            feeds_today: 1,
            total_pats: 42,
            total_feeds: 9,
        };
        let out = status(&lang, &s);
        assert!(out.contains("mood: purring"));
        assert!(out.contains("pats today: 5"));
        assert!(out.contains("last seen: 2026-07-06"));
    }

    #[test]
    fn status_fills_chinese_template() {
        let lang = crate::lang::resolve(Some("zh"));
        let s = State {
            last_seen: "2026-07-06".into(),
            pats_today: 5,
            feeds_today: 1,
            total_pats: 42,
            total_feeds: 9,
        };
        let out = status(&lang, &s);
        assert!(out.contains("Haru 状态"));
        assert!(out.contains("心情：呼噜噜"));
        assert!(!out.contains('{'), "unfilled placeholder left in: {out}");
    }
}
