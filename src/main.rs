use argon2::{hash_raw, Config, ThreadMode, Variant, Version};
use rpassword::prompt_password;
use std::process::Command;

const CONFIG: Config = Config {
    variant: Variant::Argon2id,
    version: Version::Version13,
    mem_cost: 65536,
    time_cost: 4,
    lanes: 8,
    thread_mode: ThreadMode::Parallel,
    secret: &[],
    ad: &[],
    hash_length: 46,
};
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC: &str = "0123456789";
const SPECIAL: &str = "!@#$%^&*";

fn main() {
    let seed = prompt_password("Enter seed: ").unwrap();
    let key = prompt_password("Enter key: ").unwrap();

    let hash = hash_raw(key.to_lowercase().as_bytes(), seed.as_bytes(), &CONFIG).unwrap();

    let characters = String::new() + LOWERCASE + UPPERCASE + NUMERIC + SPECIAL;

    let mut res = hash
        .get(0..16)
        .unwrap()
        .iter()
        .map(|e| characters.as_bytes()[*e as usize % characters.chars().count()] as char)
        .collect::<String>();

    let mut pos: usize = 0;

    hash.get(16..46)
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, e)| {
            if i % 2 == 0 {
                pos = *e as usize % res.chars().count();
                return;
            }

            let charset = match i {
                0..=15 => LOWERCASE,
                16..=23 => UPPERCASE,
                24..=27 => NUMERIC,
                28..=29 => SPECIAL,
                _ => unreachable!(),
            };

            res.replace_range(
                pos..pos + 1,
                &(charset.as_bytes()[*e as usize % charset.len()] as char).to_string(),
            );
        });

    println!("{}", res);

    Command::new("wl-copy")
        .arg(res)
        .status()
        .expect("Failed to copy to clipboard");
}
