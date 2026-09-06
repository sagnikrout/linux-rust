//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/em28xx/em28xx-reg.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
//
// em28xx-reg.h - Register definitions for em28xx driver
//

// em28xx endpoints
// 0x82:   (always ?) analog
pub const EM28XX_EP_AUDIO: c_uint = 0x83;
// 0x84:   digital or analog
// em2800 registers
pub const EM2800_R08_AUDIOSRC: c_uint = 0x08;
// em28xx registers
pub const EM28XX_R00_CHIPCFG: c_uint = 0x00;
// em28xx Chip Configuration 0x00
pub const EM2860_CHIPCFG_VENDOR_AUDIO: c_uint = 0x80;
pub const EM2860_CHIPCFG_I2S_VOLUME_CAPABLE: c_uint = 0x40;
pub const EM2820_CHIPCFG_I2S_3_SAMPRATES: c_uint = 0x30;
pub const EM2860_CHIPCFG_I2S_5_SAMPRATES: c_uint = 0x30;
pub const EM2820_CHIPCFG_I2S_1_SAMPRATE: c_uint = 0x20;
pub const EM2860_CHIPCFG_I2S_3_SAMPRATES: c_uint = 0x20;
pub const EM28XX_CHIPCFG_AC97: c_uint = 0x10;
pub const EM28XX_CHIPCFG_AUDIOMASK: c_uint = 0x30;
pub const EM28XX_R01_CHIPCFG2: c_uint = 0x01;
// em28xx Chip Configuration 2 0x01
pub const EM28XX_CHIPCFG2_TS_PRESENT: c_uint = 0x10;
pub const EM28XX_CHIPCFG2_TS_REQ_INTERVAL_MASK: c_uint = 0x0c /* bits 3-2 */;
pub const EM28XX_CHIPCFG2_TS_REQ_INTERVAL_1MF: c_uint = 0x00;
pub const EM28XX_CHIPCFG2_TS_REQ_INTERVAL_2MF: c_uint = 0x04;
pub const EM28XX_CHIPCFG2_TS_REQ_INTERVAL_4MF: c_uint = 0x08;
pub const EM28XX_CHIPCFG2_TS_REQ_INTERVAL_8MF: c_uint = 0x0c;
pub const EM28XX_CHIPCFG2_TS_PACKETSIZE_MASK: c_uint = 0x03 /* bits 0-1 */;
pub const EM28XX_CHIPCFG2_TS_PACKETSIZE_188: c_uint = 0x00;
pub const EM28XX_CHIPCFG2_TS_PACKETSIZE_376: c_uint = 0x01;
pub const EM28XX_CHIPCFG2_TS_PACKETSIZE_564: c_uint = 0x02;
pub const EM28XX_CHIPCFG2_TS_PACKETSIZE_752: c_uint = 0x03;
// GPIO/GPO registers
pub const EM2880_R04_GPO: c_uint = 0x04    /* em2880-em2883 only */;
pub const EM2820_R08_GPIO_CTRL: c_uint = 0x08	/* em2820-em2873/83 only */;
pub const EM2820_R09_GPIO_STATE: c_uint = 0x09	/* em2820-em2873/83 only */;
pub const EM28XX_R06_I2C_CLK: c_uint = 0x06;
// em28xx I2C Clock Register (0x06)
pub const EM28XX_I2C_CLK_ACK_LAST_READ: c_uint = 0x80;
pub const EM28XX_I2C_CLK_WAIT_ENABLE: c_uint = 0x40;
pub const EM28XX_I2C_EEPROM_ON_BOARD: c_uint = 0x08;
pub const EM28XX_I2C_EEPROM_KEY_VALID: c_uint = 0x04;
pub const EM2874_I2C_SECONDARY_BUS_SELECT: c_uint = 0x04 /* em2874 has two i2c buses */;
pub const EM28XX_I2C_FREQ_1_5_MHZ: c_uint = 0x03 /* bus frequency (bits [1-0]) */;
pub const EM28XX_I2C_FREQ_25_KHZ: c_uint = 0x02;
pub const EM28XX_I2C_FREQ_400_KHZ: c_uint = 0x01;
pub const EM28XX_I2C_FREQ_100_KHZ: c_uint = 0x00;
pub const EM28XX_R0A_CHIPID: c_uint = 0x0a;
pub const EM28XX_R0C_USBSUSP: c_uint = 0x0c;
pub const EM28XX_R0C_USBSUSP_SNAPSHOT: c_uint = 0x20 /* 1=button pressed, needs reset */;
pub const EM28XX_R0E_AUDIOSRC: c_uint = 0x0e;
pub const EM28XX_R0F_XCLK: c_uint = 0x0f;
// em28xx XCLK Register (0x0f)
pub const EM28XX_XCLK_AUDIO_UNMUTE: c_uint = 0x80 /* otherwise audio muted */;
pub const EM28XX_XCLK_I2S_MSB_TIMING: c_uint = 0x40 /* otherwise standard timing */;
pub const EM28XX_XCLK_IR_RC5_MODE: c_uint = 0x20 /* otherwise NEC mode */;
pub const EM28XX_XCLK_IR_NEC_CHK_PARITY: c_uint = 0x10;
pub const EM28XX_XCLK_FREQUENCY_30MHZ: c_uint = 0x00 /* Freq. select (bits [3-0]) */;
pub const EM28XX_XCLK_FREQUENCY_15MHZ: c_uint = 0x01;
pub const EM28XX_XCLK_FREQUENCY_10MHZ: c_uint = 0x02;
pub const EM28XX_XCLK_FREQUENCY_7_5MHZ: c_uint = 0x03;
pub const EM28XX_XCLK_FREQUENCY_6MHZ: c_uint = 0x04;
pub const EM28XX_XCLK_FREQUENCY_5MHZ: c_uint = 0x05;
pub const EM28XX_XCLK_FREQUENCY_4_3MHZ: c_uint = 0x06;
pub const EM28XX_XCLK_FREQUENCY_12MHZ: c_uint = 0x07;
pub const EM28XX_XCLK_FREQUENCY_20MHZ: c_uint = 0x08;
pub const EM28XX_XCLK_FREQUENCY_20MHZ_2: c_uint = 0x09;
pub const EM28XX_XCLK_FREQUENCY_48MHZ: c_uint = 0x0a;
pub const EM28XX_XCLK_FREQUENCY_24MHZ: c_uint = 0x0b;
pub const EM28XX_R10_VINMODE: c_uint = 0x10;
// used by all non-camera devices:
pub const EM28XX_VINMODE_YUV422_CbYCrY: c_uint = 0x10;
// used by camera devices:
pub const EM28XX_VINMODE_YUV422_YUYV: c_uint = 0x08;
pub const EM28XX_VINMODE_YUV422_YVYU: c_uint = 0x09;
pub const EM28XX_VINMODE_YUV422_UYVY: c_uint = 0x0a;
pub const EM28XX_VINMODE_YUV422_VYUY: c_uint = 0x0b;
pub const EM28XX_VINMODE_RGB8_BGGR: c_uint = 0x0c;
pub const EM28XX_VINMODE_RGB8_GRBG: c_uint = 0x0d;
pub const EM28XX_VINMODE_RGB8_GBRG: c_uint = 0x0e;
pub const EM28XX_VINMODE_RGB8_RGGB: c_uint = 0x0f;
//
// apparently:
// bit 0: swap component 1+2 with 3+4
// => e.g.: YUYV => YVYU, BGGR => GRBG
// bit 1: swap component 1 with 2 and 3 with 4
// => e.g.: YUYV => UYVY, BGGR => GBRG
//
pub const EM28XX_R11_VINCTRL: c_uint = 0x11;
// em28xx Video Input Control Register 0x11
pub const EM28XX_VINCTRL_VBI_SLICED: c_uint = 0x80;
pub const EM28XX_VINCTRL_VBI_RAW: c_uint = 0x40;
pub const EM28XX_VINCTRL_VOUT_MODE_IN: c_uint = 0x20 /* HREF,VREF,VACT in output */;
pub const EM28XX_VINCTRL_CCIR656_ENABLE: c_uint = 0x10;
pub const EM28XX_VINCTRL_VBI_16BIT_RAW: c_uint = 0x08 /* otherwise 8-bit raw */;
pub const EM28XX_VINCTRL_FID_ON_HREF: c_uint = 0x04;
pub const EM28XX_VINCTRL_DUAL_EDGE_STROBE: c_uint = 0x02;
pub const EM28XX_VINCTRL_INTERLACED: c_uint = 0x01;
pub const EM28XX_R12_VINENABLE: c_uint = 0x12	/* */;
pub const EM28XX_R14_GAMMA: c_uint = 0x14;
pub const EM28XX_R15_RGAIN: c_uint = 0x15;
pub const EM28XX_R16_GGAIN: c_uint = 0x16;
pub const EM28XX_R17_BGAIN: c_uint = 0x17;
pub const EM28XX_R18_ROFFSET: c_uint = 0x18;
pub const EM28XX_R19_GOFFSET: c_uint = 0x19;
pub const EM28XX_R1A_BOFFSET: c_uint = 0x1a;
pub const EM28XX_R1B_OFLOW: c_uint = 0x1b;
pub const EM28XX_R1C_HSTART: c_uint = 0x1c;
pub const EM28XX_R1D_VSTART: c_uint = 0x1d;
pub const EM28XX_R1E_CWIDTH: c_uint = 0x1e;
pub const EM28XX_R1F_CHEIGHT: c_uint = 0x1f;
pub const EM28XX_R20_YGAIN: c_uint = 0x20 /* contrast [0:4]   */;
pub const CONTRAST_DEFAULT: c_uint = 0x10;
pub const EM28XX_R21_YOFFSET: c_uint = 0x21 /* brightness       */	/* signed */;
pub const BRIGHTNESS_DEFAULT: c_uint = 0x00;
pub const EM28XX_R22_UVGAIN: c_uint = 0x22 /* saturation [0:4] */;
pub const SATURATION_DEFAULT: c_uint = 0x10;
pub const EM28XX_R23_UOFFSET: c_uint = 0x23 /* blue balance     */	/* signed */;
pub const BLUE_BALANCE_DEFAULT: c_uint = 0x00;
pub const EM28XX_R24_VOFFSET: c_uint = 0x24 /* red balance      */	/* signed */;
pub const RED_BALANCE_DEFAULT: c_uint = 0x00;
pub const EM28XX_R25_SHARPNESS: c_uint = 0x25 /* sharpness [0:4]  */;
pub const SHARPNESS_DEFAULT: c_uint = 0x00;
pub const EM28XX_R26_COMPR: c_uint = 0x26;
pub const EM28XX_R27_OUTFMT: c_uint = 0x27;
// em28xx Output Format Register (0x27)
pub const EM28XX_OUTFMT_RGB_8_RGRG: c_uint = 0x00;
pub const EM28XX_OUTFMT_RGB_8_GRGR: c_uint = 0x01;
pub const EM28XX_OUTFMT_RGB_8_GBGB: c_uint = 0x02;
pub const EM28XX_OUTFMT_RGB_8_BGBG: c_uint = 0x03;
pub const EM28XX_OUTFMT_RGB_16_656: c_uint = 0x04;
pub const EM28XX_OUTFMT_RGB_8_BAYER: c_uint = 0x08 /* Pattern in Reg 0x10[1-0] */;
pub const EM28XX_OUTFMT_YUV211: c_uint = 0x10;
pub const EM28XX_OUTFMT_YUV422_Y0UY1V: c_uint = 0x14;
pub const EM28XX_OUTFMT_YUV422_Y1UY0V: c_uint = 0x15;
pub const EM28XX_OUTFMT_YUV411: c_uint = 0x18;
pub const EM28XX_R28_XMIN: c_uint = 0x28;
pub const EM28XX_R29_XMAX: c_uint = 0x29;
pub const EM28XX_R2A_YMIN: c_uint = 0x2a;
pub const EM28XX_R2B_YMAX: c_uint = 0x2b;
pub const EM28XX_R30_HSCALELOW: c_uint = 0x30;
pub const EM28XX_R31_HSCALEHIGH: c_uint = 0x31;
pub const EM28XX_R32_VSCALELOW: c_uint = 0x32;
pub const EM28XX_R33_VSCALEHIGH: c_uint = 0x33;
pub const EM28XX_HVSCALE_MAX: c_uint = 0x3fff /* => 20% */;
pub const EM28XX_R34_VBI_START_H: c_uint = 0x34;
pub const EM28XX_R35_VBI_START_V: c_uint = 0x35;
//
// NOTE: the EM276x (and EM25xx, EM277x/8x ?) (camera bridges) use these
// registers for a different unknown purpose.
// => register 0x34 is set to capture width / 16
// => register 0x35 is set to capture height / 16
//
pub const EM28XX_R36_VBI_WIDTH: c_uint = 0x36;
pub const EM28XX_R37_VBI_HEIGHT: c_uint = 0x37;
pub const EM28XX_R40_AC97LSB: c_uint = 0x40;
pub const EM28XX_R41_AC97MSB: c_uint = 0x41;
pub const EM28XX_R42_AC97ADDR: c_uint = 0x42;
pub const EM28XX_R43_AC97BUSY: c_uint = 0x43;
pub const EM28XX_R45_IR: c_uint = 0x45;
//
// 0x45  bit 7    - parity bit
// bits 6-0 - count
// 0x46  IR brand
// 0x47  IR data
//
// em2874 registers
pub const EM2874_R50_IR_CONFIG: c_uint = 0x50;
pub const EM2874_R51_IR: c_uint = 0x51;
pub const EM2874_R5D_TS1_PKT_SIZE: c_uint = 0x5d;
pub const EM2874_R5E_TS2_PKT_SIZE: c_uint = 0x5e;
//
// For both TS1 and TS2, In isochronous mode:
// 0x01  188 bytes
// 0x02  376 bytes
// 0x03  564 bytes
// 0x04  752 bytes
// 0x05  940 bytes
// In bulk mode:
// 0x01..0xff  total packet count in 188-byte
//
pub const EM2874_R5F_TS_ENABLE: c_uint = 0x5f;
// em2874/174/84, em25xx, em276x/7x/8x GPIO registers
//
// NOTE: not all ports are bonded out;
// Some ports are multiplexed with special function I/O
//
pub const EM2874_R80_GPIO_P0_CTRL: c_uint = 0x80;
pub const EM2874_R81_GPIO_P1_CTRL: c_uint = 0x81;
pub const EM2874_R82_GPIO_P2_CTRL: c_uint = 0x82;
pub const EM2874_R83_GPIO_P3_CTRL: c_uint = 0x83;
pub const EM2874_R84_GPIO_P0_STATE: c_uint = 0x84;
pub const EM2874_R85_GPIO_P1_STATE: c_uint = 0x85;
pub const EM2874_R86_GPIO_P2_STATE: c_uint = 0x86;
pub const EM2874_R87_GPIO_P3_STATE: c_uint = 0x87;
// em2874 IR config register (0x50)
pub const EM2874_IR_NEC: c_uint = 0x00;
pub const EM2874_IR_NEC_NO_PARITY: c_uint = 0x01;
pub const EM2874_IR_RC5: c_uint = 0x04;
pub const EM2874_IR_RC6_MODE_0: c_uint = 0x08;
pub const EM2874_IR_RC6_MODE_6A: c_uint = 0x0b;
// em2874 Transport Stream Enable Register (0x5f)

// register settings
pub const EM2800_AUDIO_SRC_TUNER: c_uint = 0x0d;
pub const EM2800_AUDIO_SRC_LINE: c_uint = 0x0c;
pub const EM28XX_AUDIO_SRC_TUNER: c_uint = 0xc0;
pub const EM28XX_AUDIO_SRC_LINE: c_uint = 0x80;
// FIXME: Need to be populated with the other chip ID's
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_chip_id {
    CHIP_ID_EM2800 = 7,
    CHIP_ID_EM2710 = 17,
    CHIP_ID_EM2820 = 18,	/* Also used by some em2710 */
    CHIP_ID_EM2840 = 20,
    CHIP_ID_EM2750 = 33,
    CHIP_ID_EM2860 = 34,
    CHIP_ID_EM2870 = 35,
    CHIP_ID_EM2883 = 36,
    CHIP_ID_EM2765 = 54,
    CHIP_ID_EM2874 = 65,
    CHIP_ID_EM2884 = 68,
    CHIP_ID_EM28174 = 113,
    CHIP_ID_EM28178 = 114,
    CHIP_ID_EM28281 = 145,
    CHIP_ID_EM2828X = 148,
}

//
// Registers used by em202
//
// EMP202 vendor registers
pub const EM202_EXT_MODEM_CTRL: c_uint = 0x3e;
pub const EM202_GPIO_CONF: c_uint = 0x4c;
pub const EM202_GPIO_POLARITY: c_uint = 0x4e;
pub const EM202_GPIO_STICKY: c_uint = 0x50;
pub const EM202_GPIO_MASK: c_uint = 0x52;
pub const EM202_GPIO_STATUS: c_uint = 0x54;
pub const EM202_SPDIF_OUT_SEL: c_uint = 0x6a;
pub const EM202_ANTIPOP: c_uint = 0x72;
pub const EM202_EAPD_GPIO_ACCESS: c_uint = 0x74;
