use base64::{decode, encode};
use std::error::Error;
use std::io::{Cursor, Read, Write};

pub fn decode_radarview(encoded_str: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let raw_bytes = decode(encoded_str)?;
    let mut cursor = Cursor::new(&raw_bytes);

    let mut h_bytes = [0u8; 3];
    cursor.read_exact(&mut h_bytes)?;
    let h_passages = u32::from_le_bytes([h_bytes[0], h_bytes[1], h_bytes[2], 0]);
    let h_bits: Vec<u8> = (0..12)
        .map(|i| ((h_passages >> (2 * i)) & 0b11) as u8)
        .collect();

    let mut v_bytes = [0u8; 3];
    cursor.read_exact(&mut v_bytes)?;
    let v_passages = u32::from_le_bytes([v_bytes[0], v_bytes[1], v_bytes[2], 0]);
    let v_bits: Vec<u8> = (0..12)
        .map(|i| ((v_passages >> (2 * i)) & 0b11) as u8)
        .collect();

    let mut cell_bytes = [0u8; 5];
    cursor.read_exact(&mut cell_bytes)?;
    let cells: Vec<u8> = (0..9)
        .map(|i| ((cell_bytes[i / 2] >> ((i % 2) * 4)) & 0x0F) as u8)
        .collect();

    Ok((h_bits, v_bits, cells))
}

pub fn encode_radarview(
    h_passages: &[u8],
    v_passages: &[u8],
    cells: &[u8],
) -> Result<String, Box<dyn Error>> {
    let mut buffer = Vec::new();

    let h_value: u32 = h_passages
        .iter()
        .enumerate()
        .map(|(i, &val)| (val as u32) << (2 * i))
        .sum();
    buffer.write_all(&h_value.to_le_bytes()[..3])?;

    let v_value: u32 = v_passages
        .iter()
        .enumerate()
        .map(|(i, &val)| (val as u32) << (2 * i))
        .sum();
    buffer.write_all(&v_value.to_le_bytes()[..3])?;

    let mut cell_bytes = [0u8; 5];
    for (i, &cell) in cells.iter().enumerate() {
        cell_bytes[i / 2] |= (cell & 0x0F) << ((i % 2) * 4);
    }
    buffer.write_all(&cell_bytes)?;

    Ok(encode(&buffer))
}
