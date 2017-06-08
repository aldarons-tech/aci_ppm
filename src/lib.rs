// lib.rs
// Aldaron's Codec Interface - PPM
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE

fn skip_line(ppm: &'static [u8], index: &mut usize) {
	loop {
		if ppm[*index] == '\n' as u8 {
			break;
		}
		*index += 1;
	}
	*index += 1;
}

fn utf8_to_u32(ppm: &'static [u8], index: &mut usize, until: char) -> u32 {
	let mut number = 0;
	while ppm[*index] != until as u8 {
		number *= 10;
		if ppm[*index] != '0' as u8 {
			number += ppm[*index] as u32 - 48;
		}
		*index += 1;
	}
	*index += 1;
	number
}

/// Decode PPM data.
/// 
/// Returns `(width, height, pixels)`
pub fn decode(ppm: &'static [u8]) -> (u32, u32, &'static [u8]) {
	let mut index = 3;

	// Header
	if ppm[0] != 'P' as u8 || ppm[1] != '6' as u8 {
		panic!("Not a PPM file.");
	}

	// Optional Comment
	if ppm[index] == '#' as u8 {
		skip_line(ppm, &mut index);
	}

	// Width & Height
	let width = utf8_to_u32(ppm, &mut index, ' ');
	let height = utf8_to_u32(ppm, &mut index, '\n');

	// We don't care about this.  In ppm format 255 is normally here
	skip_line(ppm, &mut index);

	(width, height, &ppm[index..])
}
