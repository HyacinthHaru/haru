use rand::seq::SliceRandom;

/// The little noise Haru makes, chosen to fit the action.
pub enum Sound {
    Summon,
    Pat,
    Feed,
}

const SUMMON: &[&str] = &[
    "meow.",
    "mrrp.",
    "mew.",
    "meow?",
    "mrow.",
    "nya.",
    "prrp?",
    "meow!",
    "mrrl...",
    "mew mew.",
];

const PAT: &[&str] = &[
    "purr...",
    "prrr.",
    "*purr*",
    "purrrr.",
    "chirp.",
    "mrrp!",
    "*happy purr*",
    "brrrp.",
    "*leans in*",
    "trrrl.",
];

const FEED: &[&str] = &[
    "munch munch.",
    "nom nom.",
    "crunch crunch.",
    "om nom.",
    "*chomp*",
    "munch.",
    "nom.",
    "*happy chewing*",
    "gnaw gnaw.",
    "mlem.",
];

/// Pick a random sound for the given action.
pub fn random_sound(sound: Sound) -> &'static str {
    let set = match sound {
        Sound::Summon => SUMMON,
        Sound::Pat => PAT,
        Sound::Feed => FEED,
    };
    let mut rng = rand::thread_rng();
    set.choose(&mut rng).copied().unwrap_or("meow.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_sound_set_has_entries() {
        assert!(!SUMMON.is_empty());
        assert!(!PAT.is_empty());
        assert!(!FEED.is_empty());
    }
}
