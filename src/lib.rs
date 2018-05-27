//! ### crc16
//!
//! #### Installation
//!
//! ```toml
//! [dependencies]
//! crc16 = "*"
//! ```
//!
//! #### Usage
//!
//! ```rust
//! use crc16::*;
//!
//! // In one pass
//! assert_eq!(State::<ARC>::calculate(b"123456789"), 0xBB3D);
//!
//! // Incrementally
//! let mut state = State::<ARC>::new();
//! state.update(b"12345");
//! state.update(b"6789");
//! assert_eq!(state.get(), 0xBB3D);
//! ```
#![allow(non_snake_case, non_camel_case_types)]
#![no_std]

use core::default::Default;
use core::hash::Hasher;

include!(concat!(env!("OUT_DIR"), "/tables.rs"));

const BIT_REVERSE_TABLE: [u16; 256] = [
    0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0,
    0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8,
    0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4,
    0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC,
    0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2,
    0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA,
    0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6,
    0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE,
    0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1,
    0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9,
    0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5,
    0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD,
    0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3,
    0x0B, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB,
    0x07, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
    0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF,
];

pub trait CrcType {
    fn init() -> u16;
    fn update(crc: u16, msg: &[u8]) -> u16;
    fn get(crc: u16) -> u16;
}

/// State of crc calculation.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct State<T> {
    state: u16,
    ty: core::marker::PhantomData<T>,
}

impl<T: CrcType> State<T> {
    /// Creates new state of given CRC type.
    ///
    /// ```
    /// use crc16::*;
    ///
    /// let state = State::<ARC>::new();
    /// ```
    pub fn new() -> State<T> {
        State {
            state: <T as CrcType>::init(),
            ty: core::marker::PhantomData,
        }
    }
    /// Updates state with new data.
    pub fn update(&mut self, msg: &[u8]) {
        self.state = <T as CrcType>::update(self.state, msg);
    }
    /// Returns CRC value of state.
    pub fn get(&self) -> u16 {
        <T as CrcType>::get(self.state)
    }
    /// Calculates CRC value of given type for given message.
    ///
    /// ```
    /// use crc16::*;
    ///
    /// assert_eq!(State::<ARC>::calculate(b"123456789"), 0xBB3D);
    pub fn calculate(msg: &[u8]) -> u16 {
        <T as CrcType>::get(<T as CrcType>::update(<T as CrcType>::init(), msg))
    }
}

impl<T: CrcType> Hasher for State<T> {
    #[inline]
    fn finish(&self) -> u64 {
        self.get() as u64
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl<T: CrcType> Default for State<T> {
    fn default() -> State<T> {
        State::new()
    }
}

macro_rules! define_crc_type {
    ($(#[$attr:meta])* poly=$poly:expr, init=$init:expr, refin=$refin:ident, refout=True, xorout=$xorout:expr,
     check=$check:expr, name=$name:ident, table=$table:ident, full_name=$full_name:expr,
     test_name=$test_name:ident) =>
    (
        $(#[$attr])* pub enum $name {}

        impl CrcType for $name {
            fn init() -> u16 {
                let mut crc = $init;
                for _ in 0..16 {
                    let bit = crc & 1 > 0;
                    if bit {
                        crc = crc ^ $poly;
                    }
                    crc = crc >> 1;
                    if bit {
                        crc = crc | 0x8000;
                    }
                }
                (BIT_REVERSE_TABLE[(crc & 0xFF) as usize] << 8) | BIT_REVERSE_TABLE[(crc >> 8) as usize]
            }
            fn update(mut crc: u16, msg: &[u8]) -> u16 {
                for i in 0..msg.len() {
                    crc = ((crc >> 8) | ((msg[i] as u16) << 8)) ^ $table[(crc & 0xff) as usize];
                }
                crc
            }
            fn get(mut crc: u16) -> u16 {
                for _ in 0..2 {
                    crc = (crc >> 8) ^ $table[(crc & 0xff) as usize];
                }
                crc ^ $xorout
            }
        }

        #[cfg(test)]
        mod $test_name {
            use super::$name;
            use super::State;
            #[test]
            fn test_crc() {
                assert_eq!(State::<$name>::calculate(b"123456789"), $check);
                let mut state = State::<$name>::new();
                state.update(b"12345");
                state.update(b"6789");
                assert_eq!(state.get(), $check);
            }
        }
    );
    ($(#[$attr:meta])* poly=$poly:expr, init=$init:expr, refin=$refin:ident, refout=False, xorout=$xorout:expr,
     check=$check:expr, name=$name:ident, table=$table:ident, full_name=$full_name:expr,
     test_name=$test_name:ident) =>
    (
        $(#[$attr])* pub enum $name {}

        impl CrcType for $name {
            fn init() -> u16 {
                let mut crc = $init;
                for _ in 0..16 {
                    let bit = crc & 1 > 0;
                    if bit {
                        crc = crc ^ $poly;
                    }
                    crc = crc >> 1;
                    if bit {
                        crc = crc | 0x8000;
                    }
                }
                crc
            }
            fn update(mut crc: u16, msg: &[u8]) -> u16 {
                for i in 0..msg.len() {
                    crc = ((crc << 8) | (msg[i] as u16)) ^ $table[((crc >> 8) & 0xFF) as usize];
                }
                crc
            }
            fn get(mut crc: u16) -> u16 {
                for _ in 0..2 {
                    crc = (crc << 8) ^ $table[((crc >> 8) & 0xFF) as usize];
                }
                crc ^ $xorout
            }
        }

        #[cfg(test)]
        mod $test_name {
            use super::$name;
            use super::State;
            #[test]
            fn test_it() {
                assert_eq!(State::<$name>::calculate(b"123456789"), $check);
                let mut state = State::<$name>::new();
                state.update(b"12345");
                state.update(b"6789");
                assert_eq!(state.get(), $check);
            }
        }
    );
}

define_crc_type! {
    #[doc = "ARC ```poly=0x8005``` ```check=0xbb3d```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0x0000, refin=True, refout=True, xorout=0x0000, check=0xbb3d,
    name=ARC, table=ARC_TABLE, full_name="ARC", test_name=ARC_TEST
}

define_crc_type! {
    #[doc = "CRC-16/AUG-CCITT ```poly=0x1021``` ```check=0xe5cc```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0x1d0f, refin=False, refout=False, xorout=0x0000, check=0xe5cc,
    name=AUG_CCITT, table=AUG_CCITT_TABLE, full_name="CRC-16/AUG-CCITT", test_name=AUG_CCITT_TEST
}

define_crc_type! {
    #[doc = "CRC-16/BUYPASS ```poly=0x8005``` ```check=0xfee8```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0x0000, refin=False, refout=False, xorout=0x0000, check=0xfee8,
    name=BUYPASS, table=BUYPASS_TABLE, full_name="CRC-16/BUYPASS", test_name=BUYPASS_TEST
}

define_crc_type! {
    #[doc = "CRC-16/CCITT-FALSE ```poly=0x1021``` ```check=0x29b1```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xffff, refin=False, refout=False, xorout=0x0000, check=0x29b1,
    name=CCITT_FALSE, table=CCITT_FALSE_TABLE, full_name="CRC-16/CCITT-FALSE", test_name=CCITT_FALSE_TEST
}

define_crc_type! {
    #[doc = "CRC-16/CDMA2000 ```poly=0xc867``` ```check=0x4c06```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0xc867, init=0xffff, refin=False, refout=False, xorout=0x0000, check=0x4c06,
    name=CDMA2000, table=CDMA2000_TABLE, full_name="CRC-16/CDMA2000", test_name=CDMA2000_TEST
}

define_crc_type! {
    #[doc = "CRC-16/DDS-110 ```poly=0x1021``` ```check=0x29b1```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0x800d, refin=False, refout=False, xorout=0x0000, check=0x9ecf,
    name=DDS_110, table=DDS_110_TABLE, full_name="CRC-16/DDS-110", test_name=DDS_110_TEST
}

define_crc_type! {
    #[doc = "CRC-16/DECT-R ```poly=0x0589``` ```check=0x007e```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x0589, init=0x0000, refin=False, refout=False, xorout=0x0001, check=0x007e,
    name=DECT_R, table=DECT_R_TABLE, full_name="CRC-16/DECT-R", test_name=DECT_R_TEST
}

define_crc_type! {
    #[doc = "CRC-16/DECT-X ```poly=0x0589``` ```check=0x007f```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x0589, init=0x0000, refin=False, refout=False, xorout=0x0000, check=0x007f,
    name=DECT_X, table=DECT_X_TABLE, full_name="CRC-16/DECT-X", test_name=DECT_X_TEST
}

define_crc_type! {
    #[doc = "CRC-16/DNP ```poly=0x3d65``` ```check=0xea82```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x3d65, init=0x0000, refin=True, refout=True, xorout=0xffff, check=0xea82,
    name=DNP, table=DNP_TABLE, full_name="CRC-16/DNP", test_name=DNP_TEST
}

define_crc_type! {
    #[doc = "CRC-16/EN-13757 ```poly=0x3d65``` ```check=0xc2b7```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x3d65, init=0x0000, refin=False, refout=False, xorout=0xffff, check=0xc2b7,
    name=EN_13757, table=EN_13757_TABLE, full_name="CRC-16/EN-13757", test_name=EN_13757_TEST
}

define_crc_type! {
    #[doc = "CRC-16/GENIBUS ```poly=0x1021``` ```check=0xd64e```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xffff, refin=False, refout=False, xorout=0xffff, check=0xd64e,
    name=GENIBUS, table=GENIBUS_TABLE, full_name="CRC-16/GENIBUS", test_name=GENIBUS_TEST
}

define_crc_type! {
    #[doc = "CRC-16/MAXIM ```poly=0x8005``` ```check=0x44c2```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0x0000, refin=True, refout=True, xorout=0xffff, check=0x44c2,
    name=MAXIM, table=MAXIM_TABLE, full_name="CRC-16/MAXIM", test_name=MAXIM_TEST
}

define_crc_type! {
    #[doc = "CRC-16/MCRF4XX ```poly=0x1021``` ```check=0x6f91```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xffff, refin=True, refout=True, xorout=0x0000, check=0x6f91,
    name=MCRF4XX, table=MCRF4XX_TABLE, full_name="CRC-16/MCRF4XX", test_name=MCRF4XX_TEST
}

define_crc_type! {
    #[doc = "CRC-16/RIELLO ```poly=0x1021``` ```check=0x63d0```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xb2aa, refin=True, refout=True, xorout=0x0000, check=0x63d0,
    name=RIELLO, table=RIELLO_TABLE, full_name="CRC-16/RIELLO", test_name=RIELLO_TEST
}

define_crc_type! {
    #[doc = "CRC-16/T10-DIF ```poly=0x8bb7``` ```check=0xd0db```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8bb7, init=0x0000, refin=False, refout=False, xorout=0x0000, check=0xd0db,
    name=T10_DIF, table=T10_DIF_TABLE, full_name="CRC-16/T10-DIF", test_name=T10_DIF_TEST
}

define_crc_type! {
    #[doc = "CRC-16/TELEDISK ```poly=0xa097``` ```check=0x0fb3```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0xa097, init=0x0000, refin=False, refout=False, xorout=0x0000, check=0x0fb3,
    name=TELEDISK, table=TELEDISK_TABLE, full_name="CRC-16/TELEDISK", test_name=TELEDISK_TEST
}

define_crc_type! {
    #[doc = "CRC-16/TMS37157 ```poly=0x1021``` ```check=0x26b1```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0x89ec, refin=True, refout=True, xorout=0x0000, check=0x26b1,
    name=TMS37157, table=TMS37157_TABLE, full_name="CRC-16/TMS37157", test_name=TMS37157_TEST
}

define_crc_type! {
    #[doc = "CRC-16/USB ```poly=0x8005``` ```check=0xb4c8```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0xffff, refin=True, refout=True, xorout=0xffff, check=0xb4c8,
    name=USB, table=USB_TABLE, full_name="CRC-16/USB", test_name=USB_TEST
}

define_crc_type! {
    #[doc = "CRC-A ```poly=0x1021``` ```check=0xbf05```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xc6c6, refin=True, refout=True, xorout=0x0000, check=0xbf05,
    name=CRC_A, table=CRC_A_TABLE, full_name="CRC-A", test_name=CRC_A_TEST
}

define_crc_type! {
    #[doc = "KERMIT ```poly=0x1021``` ```check=0x2189```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0x0000, refin=True, refout=True, xorout=0x0000, check=0x2189,
    name=KERMIT, table=KERMIT_TABLE, full_name="KERMIT", test_name=KERMIT_TEST
}

define_crc_type! {
    #[doc = "MODBUS ```poly=0x8005``` ```check=0x4b37```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x8005, init=0xffff, refin=True, refout=True, xorout=0x0000, check=0x4b37,
    name=MODBUS, table=MODBUS_TABLE, full_name="MODBUS", test_name=MODBUS_TEST
}

define_crc_type! {
    #[doc = "X-25 ```poly=0x1021``` ```check=0x906e```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0xffff, refin=True, refout=True, xorout=0xffff, check=0x906e,
    name=X_25, table=X_25_TABLE, full_name="X-25", test_name=X_25_TEST
}

define_crc_type! {
    #[doc = "XMODEM ```poly=0x1021``` ```check=0x31c3```"]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    poly=0x1021, init=0x0000, refin=False, refout=False, xorout=0x0000, check=0x31c3,
    name=XMODEM, table=XMODEM_TABLE, full_name="XMODEM", test_name=XMODEM_TEST
}
