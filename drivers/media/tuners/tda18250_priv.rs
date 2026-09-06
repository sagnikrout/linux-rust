//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18250_priv.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// NXP TDA18250BHN silicon tuner driver
//
// Copyright (C) 2017 Olli Salonen <olli.salonen@iki.fi>
//

pub const R00_ID1: c_uint = 0x00	/* ID byte 1 */;
pub const R01_ID2: c_uint = 0x01	/* ID byte 2 */;
pub const R02_ID3: c_uint = 0x02	/* ID byte 3 */;
pub const R03_THERMO1: c_uint = 0x03	/* Thermo byte 1 */;
pub const R04_THERMO2: c_uint = 0x04	/* Thermo byte 2 */;
pub const R05_POWER1: c_uint = 0x05	/* Power byte 1 */;
pub const R06_POWER2: c_uint = 0x06	/* Power byte 2 */;
pub const R07_GPIO: c_uint = 0x07	/* GPIO */;
pub const R08_IRQ1: c_uint = 0x08	/* IRQ */;
pub const R09_IRQ2: c_uint = 0x09	/* IRQ */;
pub const R0A_IRQ3: c_uint = 0x0a	/* IRQ */;
pub const R0B_IRQ4: c_uint = 0x0b	/* IRQ */;
pub const R0C_AGC11: c_uint = 0x0c	/* AGC1 byte 1 */;
pub const R0D_AGC12: c_uint = 0x0d	/* AGC1 byte 2 */;
pub const R0E_AGC13: c_uint = 0x0e	/* AGC1 byte 3 */;
pub const R0F_AGC14: c_uint = 0x0f	/* AGC1 byte 4 */;
pub const R10_LT1: c_uint = 0x10	/* LT byte 1 */;
pub const R11_LT2: c_uint = 0x11	/* LT byte 2 */;
pub const R12_AGC21: c_uint = 0x12	/* AGC2 byte 1 */;
pub const R13_AGC22: c_uint = 0x13	/* AGC2 byte 2 */;
pub const R14_AGC23: c_uint = 0x14	/* AGC2 byte 3 */;
pub const R15_AGC24: c_uint = 0x15	/* AGC2 byte 4 */;
pub const R16_AGC25: c_uint = 0x16	/* AGC2 byte 5 */;
pub const R17_AGC31: c_uint = 0x17	/* AGC3 byte 1 */;
pub const R18_AGC32: c_uint = 0x18	/* AGC3 byte 2 */;
pub const R19_AGC33: c_uint = 0x19	/* AGC3 byte 3 */;
pub const R1A_AGCK: c_uint = 0x1a;
pub const R1B_GAIN1: c_uint = 0x1b;
pub const R1C_GAIN2: c_uint = 0x1c;
pub const R1D_GAIN3: c_uint = 0x1d;
pub const R1E_WI_FI: c_uint = 0x1e	/* Wireless Filter */;
pub const R1F_RF_BPF: c_uint = 0x1f	/* RF Band Pass Filter */;
pub const R20_IR_MIX: c_uint = 0x20	/* IR Mixer */;
pub const R21_IF_AGC: c_uint = 0x21;
pub const R22_IF1: c_uint = 0x22	/* IF byte 1 */;
pub const R23_IF2: c_uint = 0x23	/* IF byte 2 */;
pub const R24_IF3: c_uint = 0x24	/* IF byte 3 */;
pub const R25_REF: c_uint = 0x25	/* reference byte */;
pub const R26_IF: c_uint = 0x26	/* IF frequency */;
pub const R27_RF1: c_uint = 0x27	/* RF frequency byte 1 */;
pub const R28_RF2: c_uint = 0x28	/* RF frequency byte 2 */;
pub const R29_RF3: c_uint = 0x29	/* RF frequency byte 3 */;
pub const R2A_MSM1: c_uint = 0x2a;
pub const R2B_MSM2: c_uint = 0x2b;
pub const R2C_PS1: c_uint = 0x2c	/* power saving mode byte 1 */;
pub const R2D_PS2: c_uint = 0x2d	/* power saving mode byte 2 */;
pub const R2E_PS3: c_uint = 0x2e	/* power saving mode byte 3 */;
pub const R2F_RSSI1: c_uint = 0x2f;
pub const R30_RSSI2: c_uint = 0x30;
pub const R31_IRQ_CTRL: c_uint = 0x31;
pub const R32_DUMMY: c_uint = 0x32;
pub const R33_TEST: c_uint = 0x33;
pub const R34_MD1: c_uint = 0x34;
pub const R35_SD1: c_uint = 0x35;
pub const R36_SD2: c_uint = 0x36;
pub const R37_SD3: c_uint = 0x37;
pub const R38_SD4: c_uint = 0x38;
pub const R39_SD5: c_uint = 0x39;
pub const R3A_SD_TEST: c_uint = 0x3a;
pub const R3B_REGU: c_uint = 0x3b;
pub const R3C_RCCAL1: c_uint = 0x3c;
pub const R3D_RCCAL2: c_uint = 0x3d;
pub const R3E_IRCAL1: c_uint = 0x3e;
pub const R3F_IRCAL2: c_uint = 0x3f;
pub const R40_IRCAL3: c_uint = 0x40;
pub const R41_IRCAL4: c_uint = 0x41;
pub const R42_IRCAL5: c_uint = 0x42;
pub const R43_PD1: c_uint = 0x43	/* power down byte 1 */;
pub const R44_PD2: c_uint = 0x44	/* power down byte 2 */;
pub const R45_PD: c_uint = 0x45	/* power down */;
pub const R46_CPUMP: c_uint = 0x46	/* charge pump */;
pub const R47_LNAPOL: c_uint = 0x47	/* LNA polar casc */;
pub const R48_SMOOTH1: c_uint = 0x48	/* smooth test byte 1 */;
pub const R49_SMOOTH2: c_uint = 0x49	/* smooth test byte 2 */;
pub const R4A_SMOOTH3: c_uint = 0x4a	/* smooth test byte 3 */;
pub const R4B_XTALOSC1: c_uint = 0x4b;
pub const R4C_XTALOSC2: c_uint = 0x4c;
pub const R4D_XTALFLX1: c_uint = 0x4d;
pub const R4E_XTALFLX2: c_uint = 0x4e;
pub const R4F_XTALFLX3: c_uint = 0x4f;
pub const R50_XTALFLX4: c_uint = 0x50;
pub const R51_XTALFLX5: c_uint = 0x51;
pub const R52_IRLOOP0: c_uint = 0x52;
pub const R53_IRLOOP1: c_uint = 0x53;
pub const R54_IRLOOP2: c_uint = 0x54;
pub const R55_IRLOOP3: c_uint = 0x55;
pub const R56_IRLOOP4: c_uint = 0x56;
pub const R57_PLL_LOG: c_uint = 0x57;
pub const R58_AGC2_UP1: c_uint = 0x58;
pub const R59_AGC2_UP2: c_uint = 0x59;
pub const R5A_H3H5: c_uint = 0x5a;
pub const R5B_AGC_AUTO: c_uint = 0x5b;
pub const R5C_AGC_DEBUG: c_uint = 0x5c;
pub const TDA18250_NUM_REGS: c_int = 93;
pub const TDA18250_POWER_STANDBY: c_int = 0;
pub const TDA18250_POWER_NORMAL: c_int = 1;
pub const TDA18250_IRQ_CAL: c_uint = 0x81;
pub const TDA18250_IRQ_HW_INIT: c_uint = 0x82;
pub const TDA18250_IRQ_TUNE: c_uint = 0x88;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18250_dev {
    pub i2c_mutex: mutex,
    pub fe: *mut dvb_frontend,
    pub i2c: *mut i2c_adapter,
    pub regmap: *mut regmap,
    pub xtal_freq: u8,
// IF in kHz
    pub if_dvbt_6: u16,
    pub if_dvbt_7: u16,
    pub if_dvbt_8: u16,
    pub if_dvbc_6: u16,
    pub if_dvbc_8: u16,
    pub if_atsc: u16,
    pub if_frequency: u16,
    pub slave: bool,
    pub loopthrough: bool,
    pub warm: bool,
    pub regs: [u8; TDA18250_NUM_REGS],
}
