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

//! Registers for the MOS 6566/6567 (VIC-II) Chip
//! found in e.g. the Commodore 64.

use bitflags::bitflags;
use volatile_register::{RO, RW};

pub const BLACK: u8 = 0;
pub const WHITE: u8 = 1;
pub const RED: u8 = 2;
pub const CYAN: u8 = 3;
pub const PURPLE: u8 = 4;
pub const GREEN: u8 = 5;
pub const BLUE: u8 = 6;
pub const YELLOW: u8 = 7;
pub const ORANGE: u8 = 8;
pub const BROWN: u8 = 9;
pub const LIGHT_RED: u8 = 10;
pub const GRAY1: u8 = 11;
pub const GRAY2: u8 = 12;
pub const LIGHT_GREEN: u8 = 13;
pub const LIGHT_BLUE: u8 = 14;
pub const GRAY3: u8 = 15;

bitflags! {
    /// Bitmask for sprites 0-7
    pub struct Sprites: u8 {
        const SPRITE0 = 0b00000001;
        const SPRITE1 = 0b00000010;
        const SPRITE2 = 0b00000100;
        const SPRITE3 = 0b00001000;
        const SPRITE4 = 0b00010000;
        const SPRITE5 = 0b00100000;
        const SPRITE6 = 0b01000000;
        const SPRITE7 = 0b10000000;
    }
}

bitflags! {
    /// Y-Scroll Register Mask (0xD011)
    pub struct ControlYFlags: u8 {
        /// Control smooth y scrolling
        const YSCROLL = 0b0000_0111;
        /// Switch between 25 (on) and 24 (off) row text mode
        const ROW_SELECT = 0b0000_1000;
        /// Blank screen to border color (0 = blank)
        const BLANK_SCREEN = 0b0001_0000;
        /// Enable bitmap mode (enable = 1)
        const BITMAP_MODE = 0b0010_0000;
        /// Extended color text mode (enable = 1)
        const EXTENDED_COLOR_MODE = 0b0100_0000;
        const RASTER_COMPARE = 0b1000_0000;
    }
}

bitflags! {
    /// X-Scroll Register Flags (0xD016)
    pub struct ControlXFlags: u8 {
        /// Control smooth x scrolling
        const XSCROLL = 0b0000_0111;
        /// Switch between 38 (off) or 40 (on) column text mode
        const COLUMN_SELECT = 0b0000_1000;
        /// Enable (on) multi color for text and bitmap modes
        const MULTICOLOR = 0b0001_0000;
        /// Should always be set to zero!
        const ALWAYS_ZERO = 0b0010_0000;
        /// Unused
        const UNUSED = 0b1100_0000;
    }
}

bitflags! {
    /// Interrupt Register Flags (0xD019).
    /// Bits are set to 1 when an IRQ is detected
    ///
    /// Example:
    /// ```
    /// 
    /// ```
    pub struct InterruptFlags: u8 {
        /// Set when raster counter equals stored raster count
        const RASTER_COMPARE_IRQ = 0b00000001;
        /// Set for first collision of sprite with background
        const SPRITE_BACKGROUND_COLLISION = 0b00000010;
        /// Set for first collision of sprite with another sprite
        const SPRITE_SPRITE_COLLISION = 0b00000100;
        const LIGHPEN_TRIGGERED = 0b00001000;
        const ANY_IRQ = 0b10000000;
    }
}

bitflags! {
    /**
     * All possible charset memory locations
     *
     * Example:
     * ```
     * let bank = vic2::ScreenBank::AT_2C00.bits() | vic2::CharsetBank::AT_2000.bits();
     * (*c64::VIC).screen_and_charset_bank.write(bank);
     * ```
     */
    pub struct CharsetBank: u8 {
        const AT_0000 = 0b0000_0000;
        const AT_0800 = 0b0000_0010;
        const AT_1000 = 0b0000_0100;
        const AT_1800 = 0b0000_0110;
        const AT_2000 = 0b0000_1000;
        const AT_2800 = 0b0000_1010;
        const AT_3000 = 0b0000_1100;
        const AT_3800 = 0b0000_1110;
        const DEFAULT = Self::AT_1000.bits;
    }
}

impl CharsetBank {
    /**
     * Generate bank from charset memory address. Will check if it is valid.
     *
     * Example:
     * ```
     * const SCREEN: u16 = 0x2800;
     * const CHARSET: u16 = 0x2000;
     * const BANK: u8 = vic2::ScreenBank::from(SCREEN).bits() | vic2::CharsetBank::from(CHARSET).bits();
     * ```
     */
    pub const fn from(charset: u16) -> CharsetBank {
        let bank = ((charset >> 10) & 0x0e) as u8;
        Self::from_bits(bank).unwrap()
    }
}

bitflags! {
    /**
     * All possible screen memory locations
     */
    pub struct ScreenBank: u8 {
        const AT_0000 = 0b0000_0000;
        const AT_0400 = 0b0001_0000;
        const AT_0800 = 0b0010_0000;
        const AT_0C00 = 0b0011_0000;
        const AT_1000 = 0b0100_0000;
        const AT_1400 = 0b0101_0000;
        const AT_1800 = 0b0110_0000;
        const AT_1C00 = 0b0111_0000;
        const AT_2000 = 0b1000_0000;
        const AT_2400 = 0b1001_0000;
        const AT_2800 = 0b1010_0000;
        const AT_2C00 = 0b1011_0000;
        const AT_3000 = 0b1100_0000;
        const AT_3400 = 0b1101_0000;
        const AT_3800 = 0b1110_0000;
        const AT_3C00 = 0b1111_0000;
        const DEFAULT = Self::AT_0800.bits;
    }
}

impl ScreenBank {
    /**
     * Generate bank from screen memory address. Will check if it is valid.
     *
     * Example:
     * ```
     * const SCREEN: u16 = 0x2800;
     * const CHARSET: u16 = 0x2000;
     * const BANK: u8 = vic2::ScreenBank::from(SCREEN).bits() | vic2::CharsetBank::from(CHARSET).bits();
     * ```
     */
    pub const fn from(screen: u16) -> ScreenBank {
        let bank = (screen >> 6) as u8;
        Self::from_bits(bank).unwrap()
    }
}

#[repr(C, packed)]
pub struct MOSVideoInterfaceControllerII {
    pub sprite0_xpos: RW<u8>,
    pub sprite0_ypos: RW<u8>,
    pub sprite1_xpos: RW<u8>,
    pub sprite1_ypos: RW<u8>,
    pub sprite2_xpos: RW<u8>,
    pub sprite2_ypos: RW<u8>,
    pub sprite3_xpos: RW<u8>,
    pub sprite3_ypos: RW<u8>,
    pub sprite4_xpos: RW<u8>,
    pub sprite4_ypos: RW<u8>,
    pub sprite5_xpos: RW<u8>,
    pub sprite5_ypos: RW<u8>,
    pub sprite6_xpos: RW<u8>,
    pub sprite6_ypos: RW<u8>,
    pub sprite7_xpos: RW<u8>,
    pub sprite7_ypos: RW<u8>,
    /// [0x10]
    pub sprite_positions_most_significant_bit_of_x: RW<Sprites>,
    /// [0x11]
    pub control_y: RW<ControlYFlags>,
    /// [0x12]
    pub raster_counter: RW<u8>,
    /// [0x13]
    pub lightpen_x: RW<u8>,
    /// [0x14]
    pub lightpen_y: RW<u8>,
    /// [0x15]
    pub sprite_enable: RW<Sprites>,
    /// [0x16]
    pub control_x: RW<ControlXFlags>,
    /// [0x17]
    pub sprite_expand_y: RW<Sprites>,
    /// [0x18]
    pub screen_and_charset_bank: RW<u8>,
    /// [0x19]
    pub irq_status: RW<u8>,
    /// Enable interrupt requests (enable = 1) [0x1a]
    pub irq_enable: RW<u8>,
    /// Place non-transparent sprite data behind (0)
    /// character/bitmap data, or in front (1) [0x1b]
    pub sprite_background_priority: RW<Sprites>,
    /// [0x1c]
    pub sprite_multicolor_mode: RW<Sprites>,
    /// [0x1d]
    pub sprite_expand_x: RW<Sprites>,
    /// [0x1e]
    pub sprite_sprite_collision: RO<Sprites>,
    /// [0x1f]
    pub sprite_background_collision: RO<Sprites>,
    /// [0x20]
    pub border_color: RW<u8>,
    /// [0x21]
    pub background_color0: RW<u8>,
    /// [0x22]
    pub background_color1: RW<u8>,
    /// [0x23]
    pub background_color2: RW<u8>,
    /// [0x24]
    pub background_color3: RW<u8>,
    /// [0x25]
    pub sprite_multicolor0: RW<u8>,
    /// [0x26]
    pub sprite_multicolor1: RW<u8>,
    /// [0x27, 0x2e]
    pub sprite_colors: [RW<u8>; 8],
}

/// Calculate sprite pointer from pattern address
///
/// To make a given sprite show the pattern that's stored in RAM at `address`
/// (which must be divisible with 64), set the contents of the corresponding
/// sprite pointer address to `address` divided by 64. For instance, if the sprite pattern
/// begins at address 704, the pointer value will be 704 / 64 = 11.
pub fn to_sprite_pointer(address : u16) -> u8 {
    debug_assert!(address % 64 == 0);
    debug_assert!(address / 64 < 256);
    return (address / 64) as u8;
}