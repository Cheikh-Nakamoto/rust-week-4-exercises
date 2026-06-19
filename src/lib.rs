mod utils;
pub use utils::func::BitcoinError;

use crate::utils::func::read_array;

#[derive(Debug, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        // TODO: Implement constructor for Point
        Self { x, y }
    }
}

pub trait BitcoinSerialize {
    fn serialize(&self) -> Vec<u8>;
}

// Legacy Bitcoin transaction
#[derive(Debug, Clone)]
pub struct LegacyTransaction {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl LegacyTransaction {
    pub fn builder() -> LegacyTransactionBuilder {
        // TODO: Return a new builder for constructing a transaction
        LegacyTransactionBuilder::new()
    }
}

// Transaction builder
pub struct LegacyTransactionBuilder {
    pub version: i32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Default for LegacyTransactionBuilder {
    fn default() -> Self {
        // TODO: Implement default values
        Self {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: 0,
        }
    }
}

impl LegacyTransactionBuilder {
    pub fn new() -> Self {
        // TODO: Initialize new builder by calling default
        Self::default()
    }

    pub fn version(mut self, version: i32) -> Self {
        // TODO: Set the transaction version
        self.version = version;
        self
    }

    pub fn add_input(mut self, input: TxInput) -> Self {
        // TODO: Add input to the transaction
        self.inputs.push(input);
        self
    }

    pub fn add_output(mut self, output: TxOutput) -> Self {
        // TODO: Add output to the transaction
        self.outputs.push(output);
        self
    }

    pub fn lock_time(mut self, lock_time: u32) -> Self {
        // TODO: Set lock_time for transaction
        self.lock_time = lock_time;
        self
    }

    pub fn build(self) -> LegacyTransaction {
        // TODO: Build and return the final LegacyTransaction
        LegacyTransaction {
            version: self.version,
            inputs: self.inputs,
            outputs: self.outputs,
            lock_time: self.lock_time,
        }
    }
}

// Transaction components
#[derive(Debug, Clone)]
pub struct TxInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

#[derive(Debug, Clone)]
pub struct TxOutput {
    pub value: u64, // in satoshis
    pub script_pubkey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct OutPoint {
    pub txid: [u8; 32],
    pub vout: u32,
}

// impl BitcoinSerialize for TxInput {
//     fn serialize(&self) -> Vec<u8> {
//         let mut vec = Vec::new();
//         vec.extend_from_slice(&self.previous_output.txid);
//         vec.extend_from_slice(&self.previous_output.vout.to_le_bytes());
//         vec.extend_from_slice(&write_compact_size(self.script_sig.len() as u64));
//         vec.extend_from_slice(&self.script_sig);
//         vec.extend_from_slice(&self.sequence.to_le_bytes());
//         vec
//     }
// }

// impl BitcoinSerialize for TxOutput {
//     fn serialize(&self) -> Vec<u8> {
//         let mut vec = Vec::new();
//         vec.extend_from_slice(&self.value.to_le_bytes());
//         vec.extend_from_slice(&write_compact_size(self.script_pubkey.len() as u64));
//         vec.extend_from_slice(&self.script_pubkey);
//         vec
//     }
// }

// Simple CLI argument parser
pub fn parse_cli_args(args: &[String]) -> Result<CliCommand, BitcoinError> {
    // TODO: Match args to "send" or "balance" commands and parse required arguments
    if args.is_empty() {
        return Err(BitcoinError::ParseError(
            "Failed to parse: empty Input".to_string(),
        ));
    }
    match args[0].as_str() {
        "balance" => Ok(CliCommand::Balance),
        "send" => {
            if args.len() != 3 {
                return Err(BitcoinError::ParseError(
                    "Failed to parse: Less than 3 inputs".to_string(),
                ));
            }
            let amount = args[1].parse().map_err(|_| {
                BitcoinError::ParseError("Failed to parse: amount must be a num".to_string())
            })?;
            Ok(CliCommand::Send {
                amount,
                address: args[2].clone(),
            })
        }
        _ => Err(BitcoinError::ParseError(
            "Failed to parse: the input is neither balance nor send".to_string(),
        )),
    }
}

pub enum CliCommand {
    Send { amount: u64, address: String },
    Balance,
}

// Decoding legacy transaction
impl TryFrom<&[u8]> for LegacyTransaction {
    type Error = BitcoinError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // TODO: Parse binary data into a LegacyTransaction
        // Minimum length is 10 bytes (4 version + 4 inputs count + 4 lock_time)
        // Minimum: 4 version + 1 in_count(=0) + 1 out_count(=0) + 4 lock_time = 10 bytes
        if data.len() < 10 {
            return Err(BitcoinError::InvalidTransaction);
        }

        // &[u8] implémente Read ; lire via `&mut reader` avance la slice.
        let mut reader = data;

        // Version: 4 octets little-endian
        let version = i32::from_le_bytes(read_array::<4, _>(&mut reader)?);

        // in_count: u32 little-endian (4 octets)
        let in_count = u32::from_le_bytes(read_array::<4, _>(&mut reader)?) as usize;

        // Parse inputs : on réserve seulement la capacité annoncée
        let inputs = Vec::with_capacity(in_count);

        // out_count: u32 little-endian (4 octets)
        let out_count = u32::from_le_bytes(read_array::<4, _>(&mut reader)?) as usize;

        // Parse outputs : on réserve seulement la capacité annoncée
        let outputs = Vec::with_capacity(out_count);

        // lock_time: 4 octets LE
        let lock_time = u32::from_le_bytes(read_array::<4, _>(&mut reader)?);

        // Vérifier qu'on a consommé exactement tous les octets
        if !reader.is_empty() {
            return Err(BitcoinError::InvalidTransaction);
        }

        Ok(Self {
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

// Custom serialization for transaction
impl BitcoinSerialize for LegacyTransaction {
    fn serialize(&self) -> Vec<u8> {
        // TODO: Serialize only version and lock_time (simplified)
        let mut vec = Vec::new();

        // Version: 4 octets LE
        vec.extend_from_slice(&self.version.to_le_bytes());

        // lock_time: 4 octets LE
        vec.extend_from_slice(&self.lock_time.to_le_bytes());

        vec
    }
}
