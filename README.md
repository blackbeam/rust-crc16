# rust-crc16 [![Build Status](https://travis-ci.org/blackbeam/rust-crc16.svg?branch=master)](https://travis-ci.org/blackbeam/rust-crc16)

## Installation

Use [cargo package](https://crates.io/crates/crc16).

## Documentation

Hosted on [docs.rs](https://docs.rs/crc16).

## Supported variants
* ARC
  * ```poly=0x8005 init=0x0000 refin=true refout=true xorout=0x0000 check=0xbb3d```
* CRC-16/AUG-CCITT
  * ```poly=0x1021 init=0x1d0f refin=false refout=false xorout=0x0000 check=0xe5cc```
* CRC-16/BUYPASS
  * ```poly=0x8005 init=0x0000 refin=false refout=false xorout=0x0000 check=0xfee8```
* CRC-16/CCITT-FALSE
  * ```poly=0x1021 init=0xffff refin=false refout=false xorout=0x0000 check=0x29b1```
* CRC-16/CDMA2000
  * ```poly=0xc867 init=0xffff refin=false refout=false xorout=0x0000 check=0x4c06```
* CRC-16/DDS-110
  * ```poly=0x8005 init=0x800d refin=false refout=false xorout=0x0000 check=0x9ecf```
* CRC-16/DECT-R
  * ```poly=0x0589 init=0x0000 refin=false refout=false xorout=0x0001 check=0x007e```
* CRC-16/DECT-X
  * ```poly=0x0589 init=0x0000 refin=false refout=false xorout=0x0000 check=0x007f```
* CRC-16/DNP
  * ```poly=0x3d65 init=0x0000 refin=true refout=true xorout=0xffff check=0xea82```
* CRC-16/EN-13757
  * ```poly=0x3d65 init=0x0000 refin=false refout=false xorout=0xffff check=0xc2b7```
* CRC-16/GENIBUS
  * ```poly=0x1021 init=0xffff refin=false refout=false xorout=0xffff check=0xd64e```
* CRC-16/MAXIM
  * ```poly=0x8005 init=0x0000 refin=true refout=true xorout=0xffff check=0x44c2```
* CRC-16/MCRF4XX
  * ```poly=0x1021 init=0xffff refin=true refout=true xorout=0x0000 check=0x6f91```
* CRC-16/RIELLO
  * ```poly=0x1021 init=0xb2aa refin=true refout=true xorout=0x0000 check=0x63d0```
* CRC-16/T10-DIF
  * ```poly=0x8bb7 init=0x0000 refin=false refout=false xorout=0x0000 check=0xd0db```
* CRC-16/TELEDISK
  * ```poly=0xa097 init=0x0000 refin=false refout=false xorout=0x0000 check=0x0fb3```
* CRC-16/TMS37157
  * ```poly=0x1021 init=0x89ec refin=true refout=true xorout=0x0000 check=0x26b1```
* CRC-16/USB
  * ```poly=0x8005 init=0xffff refin=true refout=true xorout=0xffff check=0xb4c8```
* CRC-A
  * ```poly=0x1021 init=0xc6c6 refin=true refout=true xorout=0x0000 check=0xbf05```
* KERMIT
  * ```poly=0x1021 init=0x0000 refin=true refout=true xorout=0x0000 check=0x2189```
* MODBUS
  * ```poly=0x8005 init=0xffff refin=true refout=true xorout=0x0000 check=0x4b37```
* X-25
  * ```poly=0x1021 init=0xffff refin=true refout=true xorout=0xffff check=0x906e```
* XMODEM
  * ```poly=0x1021 init=0x0000 refin=false refout=false xorout=0x0000 check=0x31c3```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

