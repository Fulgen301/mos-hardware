//! C64 Plasma Example (80 x 25 mode for mega65)
//!
//! - (w)2001 by groepaz; sourced from the CC65 /samples/cbm directory
//! - Cleanup and porting to CC65 by Ullrich von Bassewitz.
//! - Porting to Rust by Mikael Lund aka Wombat (2022)

#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use itertools::{iproduct};
use ufmt_stdio::*;
use mos_hardware::*;

/// Generate stochastic character set
unsafe fn make_charset(charset_ptr: *mut u8) {
    const BITS: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];
    (*mega65::SID0).start_random_generator();

    repeat_element(SINUSTABLE.iter().copied(), 8)
        .enumerate()
        .for_each(|(cnt, sine)| {
            let mut char_pattern = 0b00000000u8;
            BITS.iter().filter(|_| rand8!(mega65::SID0) > sine).for_each(|bit| {
                char_pattern |= bit;
            });
            poke!(charset_ptr.offset(cnt as isize), char_pattern);
            if cnt % 64 == 0 {
                print!(".");
            }
        });
}

/// Render entire screen
unsafe fn render_plasma(screen_ptr: *mut u8) {
    static mut C1A: u8 = 0;
    static mut C1B: u8 = 0;
    static mut C2A: u8 = 0;
    static mut C2B: u8 = 0;
    static mut XBUF: [u8; 80] = [0; 80];
    static mut YBUF: [u8; 25] = [0; 25];

    let mut c1a = C1A;
    let mut c1b = C1B;
    YBUF.iter_mut().for_each(|y| {
        *y = add!(SINUSTABLE[c1a as usize], SINUSTABLE[c1b as usize]);
        c1a = add!(c1a, 4);
        c1b = add!(c1b, 9);
    });
    C1A = add!(C1A, 3);
    C1B = sub!(C1B, 5);

    let mut c2a = C2A;
    let mut c2b = C2B;
    XBUF.iter_mut().for_each(|x| {
        *x = add!(SINUSTABLE[c2a as usize], SINUSTABLE[c2b as usize]);
        c2a = add!(c2a, 3);
        c2b = add!(c2b, 7);
    });
    C2A = add!(C2A, 2);
    C2B = sub!(C2B, 3);

    iproduct!(YBUF.iter().copied(), XBUF.iter().copied())
        .enumerate()
        .for_each(|(cnt, (y, x))| {
            poke!(screen_ptr.offset(cnt as isize), add!(y, x));
        })
}

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    mega65::speed_mode3();       // Set CPU speed to 3.5 Mhz
    const CHARSET: u16 = 0x3000; // Custom charset
    const SCREEN1: u16 = 0x0800; // Set up two character screens...
    const SCREEN2: u16 = 0x2800; // ...for double buffering
    const PAGE1: u8 = vic2::ScreenBank::from_address(SCREEN1).bits() | vic2::CharsetBank::from(CHARSET).bits();
    const PAGE2: u8 = vic2::ScreenBank::from_address(SCREEN2).bits() | vic2::CharsetBank::from(CHARSET).bits();
    unsafe {
        make_charset(CHARSET as *mut u8);
        loop {
            render_plasma(SCREEN1 as *mut u8);
            (*mega65::VICII).screen_and_charset_bank.write(PAGE1);
            render_plasma(SCREEN2 as *mut u8);
            (*mega65::VICII).screen_and_charset_bank.write(PAGE2);
        }
    }
}

const SINUSTABLE: [u8; 256] = [
    0x80, 0x7d, 0x7a, 0x77, 0x74, 0x70, 0x6d, 0x6a, 0x67, 0x64, 0x61, 0x5e, 0x5b, 0x58, 0x55, 0x52,
    0x4f, 0x4d, 0x4a, 0x47, 0x44, 0x41, 0x3f, 0x3c, 0x39, 0x37, 0x34, 0x32, 0x2f, 0x2d, 0x2b, 0x28,
    0x26, 0x24, 0x22, 0x20, 0x1e, 0x1c, 0x1a, 0x18, 0x16, 0x15, 0x13, 0x11, 0x10, 0x0f, 0x0d, 0x0c,
    0x0b, 0x0a, 0x08, 0x07, 0x06, 0x06, 0x05, 0x04, 0x03, 0x03, 0x02, 0x02, 0x02, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x02, 0x02, 0x02, 0x03, 0x03, 0x04, 0x05, 0x06, 0x06, 0x07, 0x08, 0x0a,
    0x0b, 0x0c, 0x0d, 0x0f, 0x10, 0x11, 0x13, 0x15, 0x16, 0x18, 0x1a, 0x1c, 0x1e, 0x20, 0x22, 0x24,
    0x26, 0x28, 0x2b, 0x2d, 0x2f, 0x32, 0x34, 0x37, 0x39, 0x3c, 0x3f, 0x41, 0x44, 0x47, 0x4a, 0x4d,
    0x4f, 0x52, 0x55, 0x58, 0x5b, 0x5e, 0x61, 0x64, 0x67, 0x6a, 0x6d, 0x70, 0x74, 0x77, 0x7a, 0x7d,
    0x80, 0x83, 0x86, 0x89, 0x8c, 0x90, 0x93, 0x96, 0x99, 0x9c, 0x9f, 0xa2, 0xa5, 0xa8, 0xab, 0xae,
    0xb1, 0xb3, 0xb6, 0xb9, 0xbc, 0xbf, 0xc1, 0xc4, 0xc7, 0xc9, 0xcc, 0xce, 0xd1, 0xd3, 0xd5, 0xd8,
    0xda, 0xdc, 0xde, 0xe0, 0xe2, 0xe4, 0xe6, 0xe8, 0xea, 0xeb, 0xed, 0xef, 0xf0, 0xf1, 0xf3, 0xf4,
    0xf5, 0xf6, 0xf8, 0xf9, 0xfa, 0xfa, 0xfb, 0xfc, 0xfd, 0xfd, 0xfe, 0xfe, 0xfe, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfc, 0xfb, 0xfa, 0xfa, 0xf9, 0xf8, 0xf6,
    0xf5, 0xf4, 0xf3, 0xf1, 0xf0, 0xef, 0xed, 0xeb, 0xea, 0xe8, 0xe6, 0xe4, 0xe2, 0xe0, 0xde, 0xdc,
    0xda, 0xd8, 0xd5, 0xd3, 0xd1, 0xce, 0xcc, 0xc9, 0xc7, 0xc4, 0xc1, 0xbf, 0xbc, 0xb9, 0xb6, 0xb3,
    0xb1, 0xae, 0xab, 0xa8, 0xa5, 0xa2, 0x9f, 0x9c, 0x99, 0x96, 0x93, 0x90, 0x8c, 0x89, 0x86, 0x83,
];

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}

