use argon2::{hash_raw, Config, ThreadMode, Variant, Version};
use copypasta_ext::{prelude::ClipboardProvider, x11_fork::ClipboardContext};
use rpassword::prompt_password;

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

	let mut res: String = String::new();

	let characters = String::new() + LOWERCASE + UPPERCASE + NUMERIC + SPECIAL;

	for e in hash.get(0..16).unwrap() {
		res += &(characters.as_bytes()[*e as usize % characters.len()] as char).to_string();
	}

	let mut pos: usize = 0;

	for (i, e) in hash.get(16..46).unwrap().iter().enumerate() {
		if i % 2 == 0 {
			pos = *e as usize % res.len();
			continue;
		}

		let charset = match i {
			0..=15 => LOWERCASE,
			16..=23 => UPPERCASE,
			24..=27 => NUMERIC,
			28..=29 => SPECIAL,
			_ => "",
		};

		res.replace_range(
			pos..pos + 1,
			&(charset.as_bytes()[*e as usize % charset.len()] as char).to_string(),
		);
	}

	println!("{}", res);

	ClipboardContext::new().unwrap().set_contents(res).unwrap();
}
