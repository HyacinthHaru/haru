use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

/// What the user asked Haru to do this run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Summon,
    Pat,
    Feed,
    Status,
}

/// Haru's mood, derived from today's attention. Localized for display by the
/// `lang` module; the enum itself is language-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Waiting,
    Happy,
    Purring,
    Spoiled,
}

/// Haru's save file. Dates are stored as "YYYY-MM-DD".
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct State {
    #[serde(default)]
    pub last_seen: String,
    #[serde(default)]
    pub pats_today: u64,
    #[serde(default)]
    pub feeds_today: u64,
    #[serde(default)]
    pub total_pats: u64,
    #[serde(default)]
    pub total_feeds: u64,
}

impl State {
    /// Pure transition: roll the day over if `today` differs from `last_seen`
    /// (zeroing today's counts but keeping totals), stamp `last_seen`, then
    /// apply the action. No I/O — this is the testable core.
    pub fn advance(&self, today: &str, action: Action) -> State {
        let mut s = self.clone();
        if s.last_seen != today {
            s.pats_today = 0;
            s.feeds_today = 0;
        }
        s.last_seen = today.to_string();
        match action {
            Action::Pat => {
                s.pats_today += 1;
                s.total_pats += 1;
            }
            Action::Feed => {
                s.feeds_today += 1;
                s.total_feeds += 1;
            }
            Action::Summon | Action::Status => {}
        }
        s
    }

    /// A gentle mood derived purely from today's attention. Never stored,
    /// never decays, never punishes.
    pub fn mood(&self) -> Mood {
        match self.pats_today + self.feeds_today {
            0 => Mood::Waiting,
            1..=3 => Mood::Happy,
            4..=9 => Mood::Purring,
            _ => Mood::Spoiled,
        }
    }

    /// Load the save file, or a fresh cat if it's missing or unreadable.
    pub fn load(path: &Path) -> State {
        match std::fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
            Err(_) => State::default(),
        }
    }

    /// Write the save file atomically (temp file + rename).
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = path.with_extension("json.tmp");
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(&tmp, text)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }
}

/// Where the save file lives. `None` means we couldn't find a home directory,
/// in which case Haru runs but doesn't remember anything (ephemeral mode).
pub fn state_file_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("cat", "haru", "haru")?;
    let dir = dirs
        .state_dir()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| dirs.data_dir().to_path_buf());
    Some(dir.join("state.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_day_keeps_today_counts() {
        let s = State {
            last_seen: "2026-07-06".into(),
            pats_today: 2,
            feeds_today: 1,
            total_pats: 5,
            total_feeds: 3,
        };
        let n = s.advance("2026-07-06", Action::Pat);
        assert_eq!(n.pats_today, 3);
        assert_eq!(n.total_pats, 6);
        assert_eq!(n.feeds_today, 1);
    }

    #[test]
    fn new_day_resets_today_but_keeps_totals() {
        let s = State {
            last_seen: "2026-07-05".into(),
            pats_today: 9,
            feeds_today: 4,
            total_pats: 20,
            total_feeds: 10,
        };
        let n = s.advance("2026-07-06", Action::Pat);
        assert_eq!(n.pats_today, 1);
        assert_eq!(n.feeds_today, 0);
        assert_eq!(n.total_pats, 21);
        assert_eq!(n.total_feeds, 10);
        assert_eq!(n.last_seen, "2026-07-06");
    }

    #[test]
    fn summon_and_status_do_not_increment() {
        let s = State {
            last_seen: "2026-07-06".into(),
            pats_today: 2,
            feeds_today: 1,
            total_pats: 5,
            total_feeds: 3,
        };
        assert_eq!(s.advance("2026-07-06", Action::Summon), s);
        assert_eq!(s.advance("2026-07-06", Action::Status), s);
    }

    #[test]
    fn mood_thresholds() {
        let m = |pats: u64, feeds: u64| State {
            pats_today: pats,
            feeds_today: feeds,
            ..State::default()
        }
        .mood();
        assert_eq!(m(0, 0), Mood::Waiting);
        assert_eq!(m(2, 0), Mood::Happy);
        assert_eq!(m(3, 3), Mood::Purring);
        assert_eq!(m(6, 6), Mood::Spoiled);
    }

    #[test]
    fn serde_round_trip() {
        let s = State {
            last_seen: "2026-07-06".into(),
            pats_today: 5,
            feeds_today: 1,
            total_pats: 42,
            total_feeds: 9,
        };
        let text = serde_json::to_string(&s).unwrap();
        let back: State = serde_json::from_str(&text).unwrap();
        assert_eq!(s, back);
    }
}
