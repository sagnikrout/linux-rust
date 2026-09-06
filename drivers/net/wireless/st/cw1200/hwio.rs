//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/hwio.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level API for mac80211 ST-Ericsson CW1200 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//
// Based on:
// ST-Ericsson UMAC CW1200 driver which is
// Copyright (c) 2010, ST-Ericsson
// Author: Ajitpal Singh <ajitpal.singh@stericsson.com>
//

// Macro flag: #define CW1200_HWIO_H_INCLUDED
// extern */ struct cw1200_common;

// Download control area
// boot loader start address in SRAM

// 32K, 0x4000 to 0xDFFF

// 32K

// 128 bytes, 0xFF80 to 0xFFFF

#[repr(C)]
#[derive(Copy, Clone)]
pub struct download_cntl_t {
// size of whole firmware file (including Cheksum), host init
    pub image_size: u32,
// downloading flags
    pub flags: u32,
// No. of bytes put into the download, init & updated by host
    pub put: u32,
// last traced program counter, last ARM reg_pc
    pub trace_pc: u32,
// No. of bytes read from the download, host init, device updates
    pub get: u32,
// r0, boot losader status, host init to pending, device updates
    pub status: u32,
// Extra debug info, r1 to r14 if status=r0=DOWNLOAD_EXCEPTION
    pub debug_data: [u32; DOWNLOAD_CTRL_DATA_DWORDS],
}

// For boot loader detection

// Download error code

// Device register definitions
// WBF - SPI Register Addresses

// 16/32 bits

// 16/32 bits

// 16 bits, Q mode W/R

// 32 bits, AHB bus R/W

// 16/32 bits

// 32 bits, APB bus R/W

// 32 bits, t_settle/general

// 16 bits, Q mode read, no length

// WBF - Control register bit set
// next o/p length, bit 11 to 0

// SPI Config register bit set

// TBD: Sure???

// QueueM

// AHB bus

// APB bus

// cpu reset

// For CW1200 the IRQ Enable and Ready Bits are in CONFIG register

// val = le32_to_cpu(tmp) & 0xfffff;
extern "C" {
    pub fn cw1200_reg_write(_arg: priv, _arg: addr, _arg: &tmp, _arg: sizeof(tmp)) -> return;
}
// val = le32_to_cpu(tmp);
extern "C" {
    pub fn cw1200_reg_write(_arg: priv, _arg: addr, _arg: &tmp, _arg: sizeof(val)) -> return;
}
// val = le32_to_cpu(tmp);
extern "C" {
    pub fn cw1200_apb_write(_arg: priv, _arg: addr, _arg: &tmp, _arg: sizeof(val)) -> return;
}
// val = le32_to_cpu(tmp);
