//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/sdio.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2020 MediaTek Inc.
//
// Author: Sean Wang <sean.wang@mediatek.com>
//
pub const MT_PSE_PAGE_SZ: c_int = 128;
pub const MCR_WCIR: c_uint = 0x0000;
pub const MCR_WHLPCR: c_uint = 0x0004;

pub const MCR_WSDIOCSR: c_uint = 0x0008;
pub const MCR_WHCR: c_uint = 0x000C;

pub const MCR_WHISR: c_uint = 0x0010;
pub const MCR_WHIER: c_uint = 0x0014;

pub const MCR_WASR: c_uint = 0x0020;
pub const MCR_WSICR: c_uint = 0x0024;
pub const MCR_WTSR0: c_uint = 0x0028;

pub const MCR_WTSR1: c_uint = 0x002c;

pub const MCR_WTDR1: c_uint = 0x0034;
pub const MCR_WRDR0: c_uint = 0x0050;
pub const MCR_WRDR1: c_uint = 0x0054;

pub const MCR_H2DSM0R: c_uint = 0x0070;

pub const MCR_H2DSM1R: c_uint = 0x0074;
pub const MCR_D2HRM0R: c_uint = 0x0078;
pub const MCR_D2HRM1R: c_uint = 0x007c;
pub const MCR_D2HRM2R: c_uint = 0x0080;
pub const MCR_WRPLR: c_uint = 0x0090;

pub const MCR_WTMDR: c_uint = 0x00b0;
pub const MCR_WTMCR: c_uint = 0x00b4;
pub const MCR_WTMDPCR0: c_uint = 0x00b8;
pub const MCR_WTMDPCR1: c_uint = 0x00bc;
pub const MCR_WPLRCR: c_uint = 0x00d4;
pub const MCR_WSR: c_uint = 0x00D8;
pub const MCR_CLKIOCR: c_uint = 0x0100;
pub const MCR_CMDIOCR: c_uint = 0x0104;
pub const MCR_DAT0IOCR: c_uint = 0x0108;
pub const MCR_DAT1IOCR: c_uint = 0x010C;
pub const MCR_DAT2IOCR: c_uint = 0x0110;
pub const MCR_DAT3IOCR: c_uint = 0x0114;
pub const MCR_CLKDLYCR: c_uint = 0x0118;
pub const MCR_CMDDLYCR: c_uint = 0x011C;
pub const MCR_ODATDLYCR: c_uint = 0x0120;
pub const MCR_IDATDLYCR1: c_uint = 0x0124;
pub const MCR_IDATDLYCR2: c_uint = 0x0128;
pub const MCR_ILCHCR: c_uint = 0x012C;
pub const MCR_WTQCR0: c_uint = 0x0130;
pub const MCR_WTQCR1: c_uint = 0x0134;
pub const MCR_WTQCR2: c_uint = 0x0138;
pub const MCR_WTQCR3: c_uint = 0x013C;
pub const MCR_WTQCR4: c_uint = 0x0140;
pub const MCR_WTQCR5: c_uint = 0x0144;
pub const MCR_WTQCR6: c_uint = 0x0148;
pub const MCR_WTQCR7: c_uint = 0x014C;

pub const MCR_SWPCDBGR: c_uint = 0x0154;
pub const MCR_H2DSM2R: c_uint = 0x0160 /* supported in CONNAC2 */;
pub const MCR_H2DSM3R: c_uint = 0x0164 /* supported in CONNAC2 */;
pub const MCR_D2HRM3R: c_uint = 0x0174 /* supported in CONNAC2 */;

pub const MCR_WTQCR8: c_uint = 0x0190 /* supported in CONNAC2 */;
pub const MCR_WTQCR9: c_uint = 0x0194 /* supported in CONNAC2 */;
pub const MCR_WTQCR10: c_uint = 0x0198 /* supported in CONNAC2 */;
pub const MCR_WTQCR11: c_uint = 0x019C /* supported in CONNAC2 */;
pub const MCR_WTQCR12: c_uint = 0x01A0 /* supported in CONNAC2 */;
pub const MCR_WTQCR13: c_uint = 0x01A4 /* supported in CONNAC2 */;
pub const MCR_WTQCR14: c_uint = 0x01A8 /* supported in CONNAC2 */;
pub const MCR_WTQCR15: c_uint = 0x01AC /* supported in CONNAC2 */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_connac_sdio_ver {
    MT76_CONNAC_SDIO,
    MT76_CONNAC2_SDIO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76s_intr {
    pub isr: u32,
    pub rec_mb: *mut u32,
    pub wtqcr: *mut u32,
    pub tx: },
    pub len: [*mut u16; 2],
    pub num: *mut u16,
    pub rx: },
}
