use rand::seq::SliceRandom;

/// Which pose to draw. The mood shown in `--status` does not change the cat
/// in v0.1 — poses map to actions, not feelings.
pub enum Pose {
    Neutral,
    Pat,
    Feed,
}

const NEUTRAL: &[&str] = &[
    " /\\_/\\\n( o.o )\n > ^ <",
    " /\\_/\\\n( -.- )\n > ^ <",
    " /\\_/\\\n( o.o )\n z z z",
    " /\\_/\\\n( ^.^ )\n > ^ <",
    " /\\_/\\\n( o_o )\n > . <",
    " /\\_/\\\n( u.u )\n > ^ <",
    " /\\_/\\\n( o.- )\n > ^ <",
];

const PAT: &[&str] = &[
    " /\\_/\\\n( =^.^= )\n /  > <3",
    " /\\_/\\\n( ^w^ )\n /  >  ",
    " /\\_/\\\n( >w< )\n /  > <3",
    " /\\_/\\\n( ^-^ )\n (  > <3",
    " /\\_/\\\n( =^w^= )\n /  >  ",
];

const FEED: &[&str] = &[
    " /\\_/\\\n( o.o )\n /  > *",
    " /\\_/\\\n( =w= )\n /  > ~",
    " /\\_/\\\n( ^.^ )\n /  > @",
    " /\\_/\\\n( -.- )\n /  > =",
    " /\\_/\\\n( o.o )\n (  > o",
];

/// Pick a random cat for the given pose.
pub fn random_cat(pose: Pose) -> &'static str {
    let set = match pose {
        Pose::Neutral => NEUTRAL,
        Pose::Pat => PAT,
        Pose::Feed => FEED,
    };
    let mut rng = rand::thread_rng();
    set.choose(&mut rng)
        .copied()
        .unwrap_or(" /\\_/\\\n( o.o )\n > ^ <")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pose_has_cats() {
        assert!(!NEUTRAL.is_empty());
        assert!(!PAT.is_empty());
        assert!(!FEED.is_empty());
    }
}
