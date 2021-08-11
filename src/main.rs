use cipher_crypt::{Rot13};
use base64::{decode};
use std::fs::File;
use std::io::prelude::*;
use std::str;

fn main() -> std::io::Result<()> {
	let mut f = File::open("encrypted.txt")?;
	let mut contents = String::new();

	f.read_to_string(&mut contents)?;
	let rot13_d1 = Rot13::decrypt(&contents);
	let b64_d = decode(rot13_d1).unwrap();
	let b_t_s = str::from_utf8(&b64_d).unwrap();
	let rot13_d2 = Rot13::decrypt(b_t_s);
	println!("{}", rot13_d2);
	Ok(())
}
