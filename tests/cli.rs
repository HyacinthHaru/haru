//! End-to-end tests: run the real binary in an isolated HOME and verify that
//! state persists and that language selection works.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn tmp_home(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn haru(home: &Path, envs: &[(&str, &str)], args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_haru"));
    cmd.args(args)
        .env("HOME", home)
        .env("XDG_STATE_HOME", home.join("state"));
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.output().expect("failed to run haru")
}

fn stdout(out: Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn pat_persists_and_status_reflects_it() {
    let home = tmp_home("haru-it-en");
    let en = &[("HARU_LANG", "en")];

    let o1 = haru(&home, en, &["--pat"]);
    assert!(o1.status.success());
    assert!(stdout(o1).contains("patted Haru 1 time today"));
    assert!(stdout(haru(&home, en, &["--pat"])).contains("patted Haru 2 times today"));
    assert!(stdout(haru(&home, en, &["--status"])).contains("pats today: 2"));

    let _ = std::fs::remove_dir_all(&home);
}

#[test]
fn chinese_output_via_lang_flag_overrides_env() {
    let home = tmp_home("haru-it-zh");
    // --lang must win even over HARU_LANG=en.
    let out = stdout(haru(
        &home,
        &[("HARU_LANG", "en")],
        &["--lang", "zh", "--status"],
    ));
    assert!(out.contains("Haru 状态"), "got: {out}");
    assert!(out.contains("心情"), "got: {out}");

    let _ = std::fs::remove_dir_all(&home);
}
