pub mod args;
pub mod bytes;
pub mod config;
pub mod datalink;
pub mod error;
pub mod hamming;
pub mod modem;
pub mod physical;

pub mod devices;
pub mod speaker;

pub mod utils;

pub const DIAL_TONE: f32 = 800.0;
pub const ANSWER_TONE: f32 = 200.0;
pub const PREAMBLE: [u8; 8] = [1, 0, 1, 0, 1, 0, 1, 0];
pub const USFD: [u8; 8] = [0, 0, 0, 1, 0, 1, 1, 0];
pub const ISFD: [i8; 8] = [0, 0, 0, 1, 0, 1, 1, 0];

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
    LISTENING,
    RECEIVING,
    RESET,
    ANSWER,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ModulationMethod {
    BFSK,
    QFSK,
}
