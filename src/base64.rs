// "QUJD" should be "ABC"

pub fn decode(encoded_string: &str) -> String {
	let mut vals: Vec<u8> = vec![];
	let mut chars: Vec<char> = vec![];
	for c in encoded_string.chars() {
		vals.push(index_from_char(c).unwrap());
	}
	for mut i in 0..vals.len() {
		let mut new: u8 = 0b0000_0000;
		match i%4 {
			0 => {
				new |= vals[i].clone();
				new <<= 2;
				let mut next_val: u8 = vals[i+1].clone();
				next_val >>= 4;
				next_val &= 0b0000_0011;
				new |= next_val;
			}
			1 => {
				let mut val = vals[i].clone();
				val <<= 4;
				new |= val;
				let mut next_val = vals[i+1].clone();
				next_val >>= 2;
				new |= next_val;
			}
			2 => {
				let mut val = vals[i].clone();
				val <<= 6;
				let next_val = vals[i+1];
				val |= next_val;
				new |= val;
			}
			_ => {continue}
		}
		chars.push(new as char);
	}
	chars.into_iter().collect()
}

pub fn encode(string: &str) -> String {
	"e".to_owned()
}

fn index_from_char(ascii_char: char) -> Option<u8> {
	if ascii_char == '=' {
		return Some(0 as u8)
	}
	for (i, c) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().enumerate() {
		if ascii_char == c {
			return Some(i as u8)
		}
	}
	None
}

fn char_from_index(index: u8) {
	
}