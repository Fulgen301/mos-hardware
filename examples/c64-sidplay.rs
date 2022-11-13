// copyright 2022 mikael lund aka wombat
//
// licensed under the apache license, version 2.0 (the "license");
// you may not use this file except in compliance with the license.
// you may obtain a copy of the license at
//
//     http://www.apache.org/licenses/license-2.0
//
// unless required by applicable law or agreed to in writing, software
// distributed under the license is distributed on an "as is" basis,
// without warranties or conditions of any kind, either express or implied.
// see the license for the specific language governing permissions and
// limitations under the license.

//! Incomplete C64 SID play example

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mos_hardware::{c64, poke, vic2};
use ufmt_stdio::*;
use vic2::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

/// See https://gist.github.com/cbmeeks/2b107f0a8d36fc461ebb056e94b2f4d6
const SID_DATA_OFFSET: usize = usize::from_be_bytes([SID_MUSIC[0x06], SID_MUSIC[0x07]]);
const SID_INIT_ADDRESS: u16 = u16::from_be_bytes([SID_MUSIC[0x0a], SID_MUSIC[0x0b]]);
const SID_PLAY_ADDRESS: u16 = u16::from_be_bytes([SID_MUSIC[0x0c], SID_MUSIC[0x0d]]);
const SID_NUM_SONGS: u16 = u16::from_be_bytes([SID_MUSIC[0x0e], SID_MUSIC[0x0f]]);
const SID_SIZE: usize = SID_MUSIC.len() - SID_DATA_OFFSET;

const SID_LOAD_ADDRESS: u16 = match u16::from_be_bytes([SID_MUSIC[0x08], SID_MUSIC[0x09]]) {
    0 => u16::from_le_bytes([SID_MUSIC[SID_DATA_OFFSET], SID_MUSIC[SID_DATA_OFFSET + 1]]),
    _ => u16::from_be_bytes([SID_MUSIC[0x08], SID_MUSIC[0x09]]),
};

const SID_MUSIC: [u8; 2658] = [
    0x50, 0x53, 0x49, 0x44, 0x00, 0x02, 0x00, 0x7c, 0x00, 0x00, 0xc0, 0x00, 0xc0, 0x06, 0x00, 0x01,
    0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x54, 0x68, 0x65, 0x20, 0x4c, 0x61, 0x73, 0x74, 0x20, 0x48,
    0x65, 0x72, 0x6f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4b, 0x69, 0x6d, 0x20, 0x43, 0x68, 0x72, 0x69, 0x73, 0x74,
    0x65, 0x6e, 0x73, 0x65, 0x6e, 0x20, 0x28, 0x46, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x46, 0x72,
    0x65, 0x61, 0x6b, 0x29, 0x00, 0x00, 0x31, 0x39, 0x38, 0x38, 0x20, 0x44, 0x65, 0x78, 0x69, 0x6f,
    0x6e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x4c, 0x22,
    0xc2, 0x4c, 0x54, 0xc2, 0x4c, 0xcc, 0xc2, 0xa9, 0x01, 0x9d, 0x5c, 0xc3, 0xbd, 0x33, 0xc3, 0x85,
    0xfb, 0xbd, 0x34, 0xc3, 0x85, 0xfc, 0xbd, 0x32, 0xc3, 0x0a, 0xa8, 0xc8, 0xb1, 0xfb, 0xc9, 0x00,
    0xd0, 0x0c, 0xa9, 0x00, 0x9d, 0x32, 0xc3, 0xa0, 0x01, 0xb1, 0xfb, 0xb8, 0x50, 0x21, 0xc9, 0x01,
    0xd0, 0x1d, 0xa9, 0x00, 0x8d, 0x15, 0xc3, 0x60, 0x98, 0x9d, 0x31, 0xc3, 0xb9, 0x83, 0xc3, 0x9d,
    0x01, 0xd4, 0x9d, 0x48, 0xc3, 0xb9, 0xe3, 0xc3, 0x9d, 0x00, 0xd4, 0x9d, 0x47, 0xc3, 0x60, 0x48,
    0x88, 0xb1, 0xfb, 0x85, 0xfb, 0x68, 0x85, 0xfc, 0xbd, 0x44, 0xc3, 0xa8, 0x18, 0x69, 0x02, 0x9d,
    0x44, 0xc3, 0xb1, 0xfb, 0xc9, 0x00, 0xf0, 0x07, 0xbd, 0x44, 0xc3, 0xc9, 0x22, 0xd0, 0x27, 0xa9,
    0x00, 0x9d, 0x44, 0xc3, 0xfe, 0x32, 0xc3, 0xbd, 0x32, 0xc3, 0xcd, 0xbb, 0xc4, 0xd0, 0x14, 0xa9,
    0x00, 0x8d, 0x32, 0xc3, 0x8d, 0x39, 0xc3, 0x8d, 0x40, 0xc3, 0x8d, 0x44, 0xc3, 0x8d, 0x4b, 0xc3,
    0x8d, 0x52, 0xc3, 0x4c, 0x0e, 0xc0, 0xb1, 0xfb, 0x9d, 0x31, 0xc3, 0xc8, 0xb1, 0xfb, 0x48, 0x29,
    0x1f, 0x9d, 0x19, 0xc3, 0x68, 0x29, 0xe0, 0x4a, 0x4a, 0xa8, 0xbd, 0x6e, 0xc3, 0x85, 0xfb, 0xad,
    0x6f, 0xc3, 0x85, 0xfc, 0xa9, 0x00, 0x9d, 0x04, 0xd4, 0x9d, 0x02, 0xd4, 0x9d, 0x49, 0xc3, 0xb1,
    0xfb, 0xc8, 0x9d, 0x03, 0xd4, 0x9d, 0x4a, 0xc3, 0x29, 0xf0, 0x9d, 0x18, 0xc3, 0xb1, 0xfb, 0xc8,
    0x9d, 0x1a, 0xc3, 0xb1, 0xfb, 0xc8, 0x9d, 0x05, 0xd4, 0xb1, 0xfb, 0xc8, 0x9d, 0x06, 0xd4, 0xb1,
    0xfb, 0xc8, 0x9d, 0x2f, 0xc3, 0xb1, 0xfb, 0xc8, 0x9d, 0x1c, 0xc3, 0xb1, 0xfb, 0xc8, 0x9d, 0x5b,
    0xc3, 0xb1, 0xfb, 0xc8, 0x9d, 0x1d, 0xc3, 0xbc, 0x31, 0xc3, 0x20, 0x3a, 0xc0, 0xbd, 0x48, 0xc3,
    0x9d, 0x5e, 0xc3, 0xbd, 0x47, 0xc3, 0x9d, 0x5d, 0xc3, 0xbd, 0x1a, 0xc3, 0x9d, 0x04, 0xd4, 0xa9,
    0x00, 0x9d, 0x1e, 0xc3, 0x60, 0xfe, 0x1b, 0xc3, 0xbd, 0x1b, 0xc3, 0x18, 0x7d, 0x5b, 0xc3, 0xa8,
    0xbd, 0x31, 0xc3, 0x18, 0x79, 0x73, 0xc4, 0xa8, 0x20, 0x3e, 0xc0, 0xbd, 0x1b, 0xc3, 0x18, 0x7d,
    0x1c, 0xc3, 0xa8, 0xb9, 0x43, 0xc4, 0x18, 0x7d, 0x47, 0xc3, 0x9d, 0x47, 0xc3, 0x90, 0x03, 0xfe,
    0x48, 0xc3, 0xbd, 0x1b, 0xc3, 0xc9, 0x0c, 0xd0, 0x05, 0xa9, 0x00, 0x9d, 0x1b, 0xc3, 0xbd, 0x1d,
    0xc3, 0x29, 0x20, 0xf0, 0x17, 0xbd, 0x5c, 0xc3, 0xf0, 0x0c, 0xde, 0x5c, 0xc3, 0xa9, 0x81, 0x9d,
    0x04, 0xd4, 0x9d, 0x01, 0xd4, 0x60, 0xbd, 0x1a, 0xc3, 0x9d, 0x04, 0xd4, 0xbd, 0x49, 0xc3, 0x18,
    0x7d, 0x2f, 0xc3, 0x9d, 0x49, 0xc3, 0x9d, 0x02, 0xd4, 0xbd, 0x4a, 0xc3, 0x69, 0x00, 0x9d, 0x4a,
    0xc3, 0x9d, 0x03, 0xd4, 0xbd, 0x1d, 0xc3, 0x4a, 0x48, 0x90, 0x21, 0xbd, 0x1e, 0xc3, 0x49, 0x80,
    0x9d, 0x1e, 0xc3, 0xd0, 0x0d, 0xbd, 0x31, 0xc3, 0x38, 0xe9, 0x0c, 0xa8, 0x20, 0x3a, 0xc0, 0xb8,
    0x50, 0x0a, 0xbd, 0x31, 0xc3, 0x18, 0x69, 0x0c, 0xa8, 0x20, 0x3a, 0xc0, 0x68, 0x4a, 0x48, 0x90,
    0x07, 0xbc, 0x31, 0xc3, 0x88, 0x20, 0x3a, 0xc0, 0x68, 0x4a, 0x48, 0x90, 0x0b, 0xbd, 0x46, 0xc3,
    0x49, 0xc0, 0x9d, 0x46, 0xc3, 0x9d, 0x04, 0xd4, 0x68, 0x4a, 0x90, 0x0b, 0xbd, 0x45, 0xc3, 0x49,
    0x60, 0x9d, 0x45, 0xc3, 0x9d, 0x04, 0xd4, 0xbd, 0x18, 0xc3, 0xf0, 0x37, 0x8d, 0x5a, 0xc3, 0xbd,
    0x1d, 0xc3, 0x29, 0x10, 0xf0, 0x12, 0xbd, 0x5d, 0xc3, 0x18, 0x6d, 0x5a, 0xc3, 0x9d, 0x5d, 0xc3,
    0x90, 0x03, 0xfe, 0x5e, 0xc3, 0xb8, 0x50, 0x0f, 0xbd, 0x5d, 0xc3, 0x38, 0xee, 0x5a, 0xc3, 0x9d,
    0x5d, 0xc3, 0xb0, 0x03, 0xde, 0x5e, 0xc3, 0xbd, 0x5d, 0xc3, 0x9d, 0x47, 0xc3, 0xbd, 0x5e, 0xc3,
    0x9d, 0x48, 0xc3, 0xbd, 0x47, 0xc3, 0x9d, 0x00, 0xd4, 0xbd, 0x48, 0xc3, 0x9d, 0x01, 0xd4, 0x60,
    0x20, 0x03, 0xc0, 0xa9, 0x03, 0x8d, 0x16, 0xc3, 0xa9, 0x00, 0xa9, 0x3f, 0x8d, 0x17, 0xc3, 0xa9,
    0x00, 0x8d, 0x59, 0xc3, 0xa2, 0x17, 0xa9, 0x00, 0x9d, 0x00, 0xd4, 0xca, 0x10, 0xfa, 0xad, 0x17,
    0xc3, 0x8d, 0x18, 0xd4, 0xa9, 0x00, 0x8d, 0x17, 0xd4, 0xa9, 0x41, 0x8d, 0x46, 0xc3, 0x8d, 0x4d,
    0xc3, 0x60, 0xa9, 0x00, 0x8d, 0x6e, 0xc3, 0xa9, 0xc5, 0x8d, 0x6f, 0xc3, 0xa9, 0x40, 0x8d, 0x75,
    0xc3, 0xa9, 0xc5, 0x8d, 0x76, 0xc3, 0xa9, 0x80, 0x8d, 0x7c, 0xc3, 0xa9, 0xc5, 0x8d, 0x7d, 0xc3,
    0xad, 0xbc, 0xc4, 0x8d, 0x33, 0xc3, 0xad, 0xbd, 0xc4, 0x8d, 0x34, 0xc3, 0xad, 0xbe, 0xc4, 0x8d,
    0x3a, 0xc3, 0xad, 0xbf, 0xc4, 0x8d, 0x3b, 0xc3, 0xad, 0xc0, 0xc4, 0x8d, 0x41, 0xc3, 0xad, 0xc1,
    0xc4, 0x8d, 0x42, 0xc3, 0xa9, 0xbc, 0x85, 0xfb, 0xa9, 0xc4, 0x85, 0xfc, 0xa9, 0x07, 0x8d, 0x15,
    0xc3, 0xa9, 0x80, 0x8d, 0x1e, 0xc3, 0xa9, 0x80, 0x8d, 0x25, 0xc3, 0x8d, 0x2c, 0xc3, 0xa9, 0x01,
    0x8d, 0x1b, 0xc3, 0xa2, 0x00, 0xa9, 0x00, 0x9d, 0x32, 0xc3, 0x9d, 0x19, 0xc3, 0x9d, 0x44, 0xc3,
    0x8a, 0x18, 0x69, 0x07, 0xaa, 0xe0, 0x15, 0xd0, 0xec, 0x60, 0xa5, 0xfb, 0x48, 0xa5, 0xfc, 0x48,
    0xad, 0x2a, 0xc3, 0x8d, 0x16, 0xd4, 0xee, 0x59, 0xc3, 0xa2, 0x00, 0xad, 0x15, 0xc3, 0x3d, 0x35,
    0xc3, 0xf0, 0x13, 0x20, 0x17, 0xc1, 0xad, 0x59, 0xc3, 0xcd, 0x16, 0xc3, 0xd0, 0x08, 0xde, 0x19,
    0xc3, 0x10, 0x03, 0x20, 0x09, 0xc0, 0x8a, 0x18, 0x69, 0x07, 0xaa, 0xe0, 0x15, 0xd0, 0xdc, 0xad,
    0x59, 0xc3, 0xcd, 0x16, 0xc3, 0xd0, 0x05, 0xa9, 0x00, 0x8d, 0x59, 0xc3, 0x68, 0x85, 0xfc, 0x68,
    0x85, 0xfb, 0x60, 0x07, 0x03, 0x3f, 0x00, 0x01, 0x41, 0x00, 0x6b, 0x00, 0x00, 0x00, 0x01, 0x41,
    0x01, 0x00, 0x2d, 0x00, 0x00, 0x0d, 0x41, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x70, 0x00, 0x1a,
    0x04, 0xa1, 0xc6, 0x01, 0x60, 0x00, 0x32, 0x02, 0x4f, 0xc8, 0x02, 0x70, 0x00, 0x32, 0x04, 0xb8,
    0xc9, 0x04, 0x0c, 0x21, 0x41, 0xbb, 0x04, 0x30, 0x02, 0x10, 0x41, 0x81, 0xd1, 0x12, 0x80, 0x0d,
    0x08, 0x21, 0x81, 0xd1, 0x12, 0x30, 0x02, 0x02, 0x10, 0x00, 0x01, 0xb4, 0x04, 0x00, 0x00, 0x00,
    0x00, 0x00, 0xd1, 0x12, 0x00, 0x00, 0x00, 0x00, 0x01, 0xd1, 0x12, 0x00, 0x00, 0xc5, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x40, 0xc5, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xc5, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02,
    0x02, 0x02, 0x02, 0x02, 0x03, 0x03, 0x03, 0x03, 0x03, 0x04, 0x04, 0x04, 0x04, 0x05, 0x05, 0x05,
    0x06, 0x06, 0x07, 0x07, 0x07, 0x08, 0x08, 0x09, 0x09, 0x0a, 0x0b, 0x0b, 0x0c, 0x0d, 0x0e, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x15, 0x16, 0x17, 0x19, 0x1a, 0x1c, 0x1d, 0x1f, 0x21, 0x23, 0x25,
    0x27, 0x2a, 0x2c, 0x2f, 0x32, 0x35, 0x38, 0x3b, 0x3f, 0x43, 0x47, 0x4b, 0x4f, 0x54, 0x59, 0x5e,
    0x64, 0x6a, 0x70, 0x77, 0x7e, 0x86, 0x8e, 0x96, 0x9f, 0xa8, 0xb3, 0xbd, 0xc8, 0xd4, 0xe1, 0xee,
    0xfd, 0x0c, 0x1c, 0x2d, 0x3e, 0x51, 0x66, 0x7b, 0x91, 0xa9, 0xc3, 0xdd, 0xfa, 0x18, 0x38, 0x5a,
    0x7d, 0xa3, 0xcc, 0xf6, 0x23, 0x53, 0x86, 0xbb, 0xf4, 0x30, 0x7a, 0xb4, 0xfb, 0x47, 0x98, 0xed,
    0x47, 0xa7, 0x0c, 0x77, 0xe9, 0x61, 0xe1, 0x68, 0xf7, 0x8f, 0x30, 0xda, 0x8f, 0x4e, 0x18, 0xef,
    0xd2, 0xc3, 0xc3, 0xd1, 0xef, 0x1f, 0x60, 0xb5, 0x1e, 0x9c, 0x31, 0xdf, 0xa5, 0x87, 0x86, 0xa2,
    0xdf, 0x3e, 0xc1, 0x6b, 0x3c, 0x39, 0x63, 0xbe, 0x4b, 0x0f, 0x0c, 0x45, 0xbf, 0x7d, 0x83, 0xd6,
    0x79, 0x73, 0xc7, 0x7c, 0x97, 0x1e, 0x18, 0x8b, 0x7e, 0xfa, 0x06, 0xac, 0xf3, 0xe6, 0x8f, 0xf8,
    0x2e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x64, 0xc8,
    0x64, 0x00, 0x64, 0xc8, 0x64, 0x00, 0x64, 0xc8, 0x64, 0x00, 0x32, 0x64, 0x96, 0xc8, 0xfa, 0xc8,
    0x96, 0x64, 0x32, 0x00, 0x00, 0x00, 0x28, 0x50, 0x78, 0x50, 0x28, 0x00, 0x28, 0x50, 0x78, 0x50,
    0x28, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x07,
    0x00, 0x03, 0x07, 0x00, 0x03, 0x07, 0x00, 0x03, 0x07, 0x00, 0x05, 0x09, 0x00, 0x05, 0x09, 0x00,
    0x05, 0x09, 0x00, 0x05, 0x09, 0x00, 0x04, 0x07, 0x00, 0x04, 0x07, 0x00, 0x04, 0x07, 0x00, 0x04,
    0x07, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x07, 0x00,
    0x07, 0x00, 0x07, 0x00, 0x07, 0x00, 0x07, 0x00, 0x07, 0x26, 0xa1, 0xc6, 0x4f, 0xc8, 0xb8, 0xc9,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41,
    0x09, 0x90, 0x70, 0x6b, 0x00, 0x00, 0x06, 0x41, 0x09, 0x50, 0x37, 0x40, 0x24, 0xc5, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x41,
    0x06, 0x60, 0x60, 0x00, 0x00, 0x2d, 0x08, 0x41, 0x09, 0x00, 0x70, 0x00, 0x0c, 0x20, 0x08, 0x41,
    0x09, 0x00, 0x70, 0x00, 0x24, 0x20, 0x02, 0x41, 0x07, 0x60, 0x40, 0x00, 0x00, 0x20, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x41,
    0x07, 0x70, 0x60, 0x00, 0x00, 0x20, 0x00, 0x41, 0x0a, 0xaf, 0x70, 0x00, 0x00, 0x00, 0x19, 0x41,
    0x00, 0xaf, 0x10, 0x01, 0x00, 0x10, 0x09, 0x41, 0x00, 0xaf, 0x10, 0x01, 0x00, 0x00, 0x00, 0x41,
    0x00, 0x80, 0x90, 0x00, 0x00, 0x00, 0x0f, 0x41, 0x0a, 0xaf, 0xa0, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x05,
    0x1a, 0x02, 0x1a, 0x02, 0x2d, 0x25, 0x1a, 0x02, 0x1a, 0x02, 0x1a, 0x05, 0x1a, 0x02, 0x1a, 0x02,
    0x2d, 0x25, 0x1a, 0x02, 0x1a, 0x02, 0x00, 0x22, 0x05, 0x22, 0x02, 0x22, 0x02, 0x2d, 0x25, 0x22,
    0x02, 0x22, 0x02, 0x22, 0x05, 0x22, 0x02, 0x22, 0x02, 0x2d, 0x25, 0x22, 0x02, 0x22, 0x02, 0x00,
    0x1d, 0x05, 0x1d, 0x02, 0x1d, 0x02, 0x2d, 0x25, 0x1d, 0x02, 0x1d, 0x02, 0x1c, 0x05, 0x1c, 0x02,
    0x1c, 0x02, 0x2d, 0x25, 0x1c, 0x02, 0x1c, 0x02, 0x00, 0x1d, 0x05, 0x1d, 0x02, 0x1d, 0x02, 0x2d,
    0x25, 0x1d, 0x02, 0x1d, 0x02, 0x1d, 0x05, 0x1d, 0x02, 0x1d, 0x02, 0x2d, 0x25, 0x1d, 0x02, 0x1d,
    0x02, 0x00, 0x1f, 0x05, 0x1f, 0x02, 0x1f, 0x02, 0x2d, 0x25, 0x1f, 0x02, 0x1f, 0x02, 0x1f, 0x05,
    0x1f, 0x02, 0x1f, 0x02, 0x2d, 0x25, 0x1f, 0x02, 0x1f, 0x02, 0x00, 0x21, 0x05, 0x21, 0x02, 0x21,
    0x02, 0x2d, 0x25, 0x21, 0x02, 0x21, 0x02, 0x21, 0x05, 0x21, 0x02, 0x21, 0x02, 0x2d, 0x25, 0x21,
    0x02, 0x21, 0x02, 0x00, 0x21, 0x05, 0x21, 0x02, 0x21, 0x02, 0x2d, 0x25, 0x21, 0x02, 0x21, 0x02,
    0x21, 0x05, 0x21, 0x02, 0x21, 0x02, 0x2d, 0x25, 0x21, 0x02, 0x21, 0x02, 0x00, 0x1d, 0x05, 0x1d,
    0x02, 0x1d, 0x02, 0x2d, 0x25, 0x1d, 0x02, 0x1d, 0x02, 0x1c, 0x05, 0x1c, 0x02, 0x1c, 0x02, 0x2d,
    0x25, 0x1c, 0x02, 0x1c, 0x02, 0x00, 0x22, 0x05, 0x22, 0x02, 0x22, 0x02, 0x2d, 0x25, 0x22, 0x02,
    0x22, 0x02, 0x21, 0x05, 0x21, 0x02, 0x21, 0x02, 0x2d, 0x25, 0x21, 0x02, 0x21, 0x02, 0x00, 0xc0,
    0xc5, 0xc0, 0xc5, 0xd9, 0xc5, 0xf2, 0xc5, 0xc0, 0xc5, 0xc0, 0xc5, 0xd9, 0xc5, 0xf2, 0xc5, 0xc0,
    0xc5, 0xc0, 0xc5, 0xd9, 0xc5, 0xf2, 0xc5, 0xc0, 0xc5, 0xc0, 0xc5, 0x0b, 0xc6, 0x24, 0xc6, 0x3d,
    0xc6, 0xc0, 0xc5, 0x0b, 0xc6, 0x24, 0xc6, 0x3d, 0xc6, 0xc0, 0xc5, 0xc0, 0xc5, 0xc0, 0xc5, 0xd9,
    0xc5, 0xf2, 0xc5, 0xc0, 0xc5, 0xc0, 0xc5, 0xd9, 0xc5, 0xf2, 0xc5, 0x56, 0xc6, 0x6f, 0xc6, 0x56,
    0xc6, 0x6f, 0xc6, 0xc0, 0xc5, 0x88, 0xc6, 0xc0, 0xc5, 0x88, 0xc6, 0xa1, 0x00, 0x26, 0x65, 0x32,
    0x22, 0x26, 0x65, 0x32, 0x28, 0x26, 0x65, 0x32, 0x22, 0x26, 0x65, 0x32, 0x28, 0x26, 0x65, 0x32,
    0x22, 0x26, 0x65, 0x32, 0x28, 0x26, 0x65, 0x32, 0x22, 0x26, 0x65, 0x32, 0x28, 0x22, 0x65, 0x3a,
    0x42, 0x22, 0x65, 0x3a, 0x48, 0x22, 0x65, 0x3a, 0x42, 0x22, 0x65, 0x3a, 0x48, 0x29, 0x65, 0x35,
    0x42, 0x29, 0x65, 0x35, 0x48, 0x28, 0x65, 0x3c, 0x42, 0x28, 0x65, 0x3c, 0x48, 0x39, 0x02, 0x32,
    0x02, 0x32, 0x22, 0x32, 0x02, 0x39, 0x02, 0x32, 0x22, 0x35, 0x02, 0x32, 0x02, 0x39, 0x02, 0x32,
    0x02, 0x32, 0x22, 0x32, 0x02, 0x39, 0x02, 0x32, 0x22, 0x35, 0x02, 0x32, 0x02, 0x3a, 0x02, 0x32,
    0x02, 0x3a, 0x42, 0x32, 0x02, 0x3a, 0x02, 0x3a, 0x42, 0x35, 0x02, 0x32, 0x02, 0x3a, 0x02, 0x32,
    0x02, 0x3a, 0x42, 0x32, 0x02, 0x3a, 0x02, 0x3a, 0x42, 0x35, 0x02, 0x32, 0x02, 0x39, 0x02, 0x30,
    0x02, 0x35, 0x42, 0x30, 0x02, 0x39, 0x02, 0x35, 0x42, 0x35, 0x02, 0x30, 0x02, 0x37, 0x02, 0x30,
    0x02, 0x3c, 0x42, 0x30, 0x02, 0x37, 0x02, 0x3c, 0x42, 0x34, 0x02, 0x30, 0x02, 0x34, 0x02, 0x2d,
    0x02, 0x39, 0x22, 0x2d, 0x02, 0x34, 0x02, 0x39, 0x22, 0x30, 0x02, 0x2d, 0x02, 0x34, 0x02, 0x2d,
    0x02, 0x39, 0x22, 0x2d, 0x02, 0x34, 0x02, 0x39, 0x22, 0x30, 0x02, 0x2d, 0x02, 0x35, 0x02, 0x2e,
    0x02, 0x3a, 0x42, 0x2e, 0x02, 0x35, 0x02, 0x3a, 0x42, 0x32, 0x02, 0x2e, 0x02, 0x35, 0x02, 0x2e,
    0x02, 0x3a, 0x42, 0x2e, 0x02, 0x35, 0x02, 0x3a, 0x42, 0x32, 0x02, 0x2e, 0x02, 0x37, 0x02, 0x30,
    0x02, 0x3c, 0x42, 0x30, 0x02, 0x37, 0x02, 0x3c, 0x42, 0x34, 0x02, 0x30, 0x02, 0x37, 0x02, 0x30,
    0x02, 0x3c, 0x42, 0x30, 0x02, 0x37, 0x02, 0x3c, 0x42, 0x34, 0x02, 0x30, 0x02, 0x39, 0x02, 0x32,
    0x02, 0x32, 0x42, 0x32, 0x02, 0x39, 0x02, 0x32, 0x42, 0x36, 0x02, 0x32, 0x02, 0x39, 0x02, 0x32,
    0x02, 0x32, 0x42, 0x32, 0x02, 0x39, 0x02, 0x36, 0x02, 0x32, 0x02, 0x2d, 0x02, 0x34, 0x02, 0x2d,
    0x02, 0x39, 0x22, 0x2d, 0x02, 0x34, 0x02, 0x39, 0x22, 0x30, 0x02, 0x2d, 0x02, 0x32, 0x02, 0x2b,
    0x02, 0x37, 0x42, 0x2b, 0x02, 0x32, 0x02, 0x37, 0x42, 0x2f, 0x02, 0x2b, 0x02, 0x39, 0x02, 0x32,
    0x02, 0x32, 0x22, 0x32, 0x02, 0x39, 0x02, 0x32, 0x22, 0x35, 0x02, 0x32, 0x02, 0x37, 0x02, 0x30,
    0x02, 0x30, 0x42, 0x30, 0x02, 0x37, 0x02, 0x30, 0x42, 0x34, 0x02, 0x30, 0x02, 0xef, 0xc6, 0x0f,
    0xc7, 0x2f, 0xc7, 0x2f, 0xc7, 0x4f, 0xc7, 0x6f, 0xc7, 0x2f, 0xc7, 0x2f, 0xc7, 0x4f, 0xc7, 0x6f,
    0xc7, 0x2f, 0xc7, 0x2f, 0xc7, 0x8f, 0xc7, 0xaf, 0xc7, 0xcf, 0xc7, 0xef, 0xc7, 0x8f, 0xc7, 0xaf,
    0xc7, 0xcf, 0xc7, 0xef, 0xc7, 0x2f, 0xc7, 0x2f, 0xc7, 0x4f, 0xc7, 0x6f, 0xc7, 0x2f, 0xc7, 0x2f,
    0xc7, 0x4f, 0xc7, 0x6f, 0xc7, 0x8f, 0xc7, 0x0f, 0xc8, 0x8f, 0xc7, 0x0f, 0xc8, 0x2f, 0xc7, 0x2f,
    0xc8, 0x2f, 0xc7, 0x2f, 0xc8, 0x4f, 0x00, 0x39, 0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x39,
    0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x39, 0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x39,
    0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x3a, 0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x3a,
    0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x3a, 0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x3a,
    0x02, 0x32, 0x02, 0x35, 0x02, 0x32, 0x02, 0x39, 0x02, 0x30, 0x02, 0x35, 0x02, 0x30, 0x02, 0x39,
    0x02, 0x30, 0x02, 0x35, 0x02, 0x30, 0x02, 0x37, 0x02, 0x30, 0x02, 0x34, 0x02, 0x30, 0x02, 0x37,
    0x02, 0x30, 0x02, 0x34, 0x02, 0x30, 0x02, 0x39, 0x2b, 0x37, 0x25, 0x39, 0x22, 0x32, 0x2e, 0x35,
    0x25, 0x37, 0x25, 0x39, 0x2b, 0x37, 0x25, 0x3c, 0x22, 0x39, 0x2e, 0x35, 0x25, 0x37, 0x25, 0x39,
    0x2b, 0x37, 0x25, 0x39, 0x22, 0x30, 0x2e, 0x35, 0x25, 0x37, 0x25, 0x39, 0x28, 0x3a, 0x28, 0x3c,
    0x25, 0x39, 0x28, 0x37, 0x28, 0x35, 0x25, 0x00, 0x32, 0x4b, 0x32, 0x6b, 0x30, 0x65, 0x32, 0x65,
    0x34, 0x65, 0x33, 0x4b, 0x35, 0x65, 0x34, 0x65, 0x32, 0x6b, 0x34, 0x65, 0x35, 0x65, 0x39, 0x71,
    0x37, 0x6b, 0x35, 0x65, 0x30, 0x6b, 0x32, 0x6b, 0x39, 0x85, 0x36, 0x85, 0x32, 0x85, 0x39, 0x85,
    0x36, 0x85, 0x32, 0x85, 0x2d, 0x65, 0x00, 0x35, 0x25, 0x37, 0x25, 0x39, 0x28, 0x37, 0x28, 0x35,
    0x25, 0x37, 0x28, 0x35, 0x28, 0x34, 0x25, 0x00, 0x37, 0x51, 0x39, 0x71, 0x39, 0xa5, 0x3b, 0xa5,
    0x3c, 0xa8, 0x3e, 0xa8, 0x3c, 0xa5, 0x3b, 0xa8, 0x37, 0xa8, 0x34, 0xbd, 0x34, 0x6b, 0x39, 0xa5,
    0x3b, 0xa5, 0x3c, 0xa8, 0x3e, 0xa8, 0x40, 0xa5, 0x3e, 0xa8, 0x3b, 0xa8, 0x37, 0xa5, 0x00, 0x2f,
    0x51, 0x32, 0x71, 0x32, 0xa5, 0x34, 0xa5, 0x35, 0xa8, 0x37, 0xa8, 0x35, 0xa5, 0x34, 0xa8, 0x30,
    0xa8, 0x2d, 0xbd, 0x2d, 0x6b, 0x32, 0xa5, 0x34, 0xa5, 0x35, 0xa8, 0x37, 0xa8, 0x39, 0xa5, 0x37,
    0xa8, 0x34, 0xa8, 0x30, 0xa5, 0x00, 0x99, 0xc8, 0x99, 0xc8, 0xb9, 0xc8, 0xd9, 0xc8, 0xf9, 0xc8,
    0x19, 0xc9, 0xf9, 0xc8, 0x19, 0xc9, 0x99, 0xc8, 0x99, 0xc8, 0x2a, 0xc9, 0x4a, 0xc9, 0x2a, 0xc9,
    0x4a, 0xc9, 0xf9, 0xc8, 0x59, 0xc9, 0xf9, 0xc8, 0x19, 0xc9, 0x6a, 0xc9, 0x8a, 0xc9, 0x91, 0xc9,
    0xb1, 0xc9,
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("panic!");
    loop {}
}