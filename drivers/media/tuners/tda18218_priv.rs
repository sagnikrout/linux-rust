//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/tda18218_priv.h
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
// NXP TDA18218HN silicon tuner driver
//
// Copyright (C) 2010 Antti Palosaari <crope@iki.fi>
//

pub const R00_ID: c_uint = 0x00	/* ID byte */;
pub const R01_R1: c_uint = 0x01	/* Read byte 1 */;
pub const R02_R2: c_uint = 0x02	/* Read byte 2 */;
pub const R03_R3: c_uint = 0x03	/* Read byte 3 */;
pub const R04_R4: c_uint = 0x04	/* Read byte 4 */;
pub const R05_R5: c_uint = 0x05	/* Read byte 5 */;
pub const R06_R6: c_uint = 0x06	/* Read byte 6 */;
pub const R07_MD1: c_uint = 0x07	/* Main divider byte 1 */;
pub const R08_PSM1: c_uint = 0x08	/* PSM byte 1 */;
pub const R09_MD2: c_uint = 0x09	/* Main divider byte 2 */;
pub const R0A_MD3: c_uint = 0x0a	/* Main divider byte 1 */;
pub const R0B_MD4: c_uint = 0x0b	/* Main divider byte 4 */;
pub const R0C_MD5: c_uint = 0x0c	/* Main divider byte 5 */;
pub const R0D_MD6: c_uint = 0x0d	/* Main divider byte 6 */;
pub const R0E_MD7: c_uint = 0x0e	/* Main divider byte 7 */;
pub const R0F_MD8: c_uint = 0x0f	/* Main divider byte 8 */;
pub const R10_CD1: c_uint = 0x10	/* Call divider byte 1 */;
pub const R11_CD2: c_uint = 0x11	/* Call divider byte 2 */;
pub const R12_CD3: c_uint = 0x12	/* Call divider byte 3 */;
pub const R13_CD4: c_uint = 0x13	/* Call divider byte 4 */;
pub const R14_CD5: c_uint = 0x14	/* Call divider byte 5 */;
pub const R15_CD6: c_uint = 0x15	/* Call divider byte 6 */;
pub const R16_CD7: c_uint = 0x16	/* Call divider byte 7 */;
pub const R17_PD1: c_uint = 0x17	/* Power-down byte 1 */;
pub const R18_PD2: c_uint = 0x18	/* Power-down byte 2 */;
pub const R19_XTOUT: c_uint = 0x19	/* XTOUT byte */;
pub const R1A_IF1: c_uint = 0x1a	/* IF byte 1 */;
pub const R1B_IF2: c_uint = 0x1b	/* IF byte 2 */;
pub const R1C_AGC2B: c_uint = 0x1c	/* AGC2b byte */;
pub const R1D_PSM2: c_uint = 0x1d	/* PSM byte 2 */;
pub const R1E_PSM3: c_uint = 0x1e	/* PSM byte 3 */;
pub const R1F_PSM4: c_uint = 0x1f	/* PSM byte 4 */;
pub const R20_AGC11: c_uint = 0x20	/* AGC1 byte 1 */;
pub const R21_AGC12: c_uint = 0x21	/* AGC1 byte 2 */;
pub const R22_AGC13: c_uint = 0x22	/* AGC1 byte 3 */;
pub const R23_AGC21: c_uint = 0x23	/* AGC2 byte 1 */;
pub const R24_AGC22: c_uint = 0x24	/* AGC2 byte 2 */;
pub const R25_AAGC: c_uint = 0x25	/* Analog AGC byte */;
pub const R26_RC: c_uint = 0x26	/* RC byte */;
pub const R27_RSSI: c_uint = 0x27	/* RSSI byte */;
pub const R28_IRCAL1: c_uint = 0x28	/* IR CAL byte 1 */;
pub const R29_IRCAL2: c_uint = 0x29	/* IR CAL byte 2 */;
pub const R2A_IRCAL3: c_uint = 0x2a	/* IR CAL byte 3 */;
pub const R2B_IRCAL4: c_uint = 0x2b	/* IR CAL byte 4 */;
pub const R2C_RFCAL1: c_uint = 0x2c	/* RF CAL byte 1 */;
pub const R2D_RFCAL2: c_uint = 0x2d	/* RF CAL byte 2 */;
pub const R2E_RFCAL3: c_uint = 0x2e	/* RF CAL byte 3 */;
pub const R2F_RFCAL4: c_uint = 0x2f	/* RF CAL byte 4 */;
pub const R30_RFCAL5: c_uint = 0x30	/* RF CAL byte 5 */;
pub const R31_RFCAL6: c_uint = 0x31	/* RF CAL byte 6 */;
pub const R32_RFCAL7: c_uint = 0x32	/* RF CAL byte 7 */;
pub const R33_RFCAL8: c_uint = 0x33	/* RF CAL byte 8 */;
pub const R34_RFCAL9: c_uint = 0x34	/* RF CAL byte 9 */;
pub const R35_RFCAL10: c_uint = 0x35	/* RF CAL byte 10 */;
pub const R36_RFCALRAM1: c_uint = 0x36	/* RF CAL RAM byte 1 */;
pub const R37_RFCALRAM2: c_uint = 0x37	/* RF CAL RAM byte 2 */;
pub const R38_MARGIN: c_uint = 0x38	/* Margin byte */;
pub const R39_FMAX1: c_uint = 0x39	/* Fmax byte 1 */;
pub const R3A_FMAX2: c_uint = 0x3a	/* Fmax byte 2 */;
pub const TDA18218_NUM_REGS: c_int = 59;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tda18218_priv {
    pub cfg: *mut tda18218_config,
    pub i2c: *mut i2c_adapter,
    pub if_frequency: u32,
    pub regs: [u8; TDA18218_NUM_REGS],
}
