use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcoinError {
    #[error("Invalid transaction format")]
    InvalidTransaction,
    #[error("Invalid script format")]
    InvalidScript,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Parse error: {0}")]
    ParseError(String),
}

// pub fn read_compact_size<R: Read>(reader: &mut R) -> Result<u64, BitcoinError> {
//     let mut first = [0u8; 1];
//     reader
//         .read_exact(&mut first)
//         .map_err(|_| BitcoinError::InvalidTransaction)?;

//     match first[0] {
//         0x00..=0xFC => Ok(first[0] as u64),
//         0xFD => {
//             let mut buf = [0u8; 2];
//             reader
//                 .read_exact(&mut buf)
//                 .map_err(|_| BitcoinError::InvalidTransaction)?;
//             Ok(u16::from_le_bytes(buf) as u64)
//         }
//         0xFE => {
//             let mut buf = [0u8; 4];
//             reader
//                 .read_exact(&mut buf)
//                 .map_err(|_| BitcoinError::InvalidTransaction)?;
//             Ok(u32::from_le_bytes(buf) as u64)
//         }
//         0xFF => {
//             let mut buf = [0u8; 8];
//             reader
//                 .read_exact(&mut buf)
//                 .map_err(|_| BitcoinError::InvalidTransaction)?;
//             Ok(u64::from_le_bytes(buf))
//         }
//     }
// }

// pub fn write_compact_size(value: u64) -> Vec<u8> {
//     let mut result = Vec::new();
//     if value < 0xFD {
//         result.push(value as u8);
//     } else if value <= 0xFFFF {
//         result.push(0xFD);
//         result.extend_from_slice(&(value as u16).to_le_bytes());
//     } else if value <= 0xFFFFFFFF {
//         result.push(0xFE);
//         result.extend_from_slice(&(value as u32).to_le_bytes());
//     } else {
//         result.push(0xFF);
//         result.extend_from_slice(&value.to_le_bytes());
//     }
//     result
// }

pub fn read_array<const N: usize, R: Read>(reader: &mut R) -> Result<[u8; N], BitcoinError> {
    let mut buf = [0u8; N];
    reader
        .read_exact(&mut buf)
        .map(|_| buf)
        .map_err(|_| BitcoinError::InvalidTransaction)
}
