//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/sh_mmcif.h
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
// platform data for eMMC driver
//
// Copyright (C) 2010 Renesas Solutions Corp.
//

//
// MMCIF : CE_CLK_CTRL [19:16]
// 1000 : Peripheral clock / 512
// 0111 : Peripheral clock / 256
// 0110 : Peripheral clock / 128
// 0101 : Peripheral clock / 64
// 0100 : Peripheral clock / 32
// 0011 : Peripheral clock / 16
// 0010 : Peripheral clock / 8
// 0001 : Peripheral clock / 4
// 0000 : Peripheral clock / 2
// 1111 : Peripheral clock (sup_pclk set '1')
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mmcif_plat_data {
    pub /: *mut *mut unsigned int slave_id_tx; / embedded slave_id_[tr]x,
    pub slave_id_rx: c_uint,
    pub /: *mut *mut u8 sup_pclk; / 1 :SH7757, 0: SH7724/SH7372,
    pub caps: c_ulong,
    pub ocr: u32,
}

pub const MMCIF_CE_CMD_SET: c_uint = 0x00000000;
pub const MMCIF_CE_ARG: c_uint = 0x00000008;
pub const MMCIF_CE_ARG_CMD12: c_uint = 0x0000000C;
pub const MMCIF_CE_CMD_CTRL: c_uint = 0x00000010;
pub const MMCIF_CE_BLOCK_SET: c_uint = 0x00000014;
pub const MMCIF_CE_CLK_CTRL: c_uint = 0x00000018;
pub const MMCIF_CE_BUF_ACC: c_uint = 0x0000001C;
pub const MMCIF_CE_RESP3: c_uint = 0x00000020;
pub const MMCIF_CE_RESP2: c_uint = 0x00000024;
pub const MMCIF_CE_RESP1: c_uint = 0x00000028;
pub const MMCIF_CE_RESP0: c_uint = 0x0000002C;
pub const MMCIF_CE_RESP_CMD12: c_uint = 0x00000030;
pub const MMCIF_CE_DATA: c_uint = 0x00000034;
pub const MMCIF_CE_INT: c_uint = 0x00000040;
pub const MMCIF_CE_INT_MASK: c_uint = 0x00000044;
pub const MMCIF_CE_HOST_STS1: c_uint = 0x00000048;
pub const MMCIF_CE_HOST_STS2: c_uint = 0x0000004C;
pub const MMCIF_CE_CLK_CTRL2: c_uint = 0x00000070;
pub const MMCIF_CE_VERSION: c_uint = 0x0000007C;
// CE_BUF_ACC

// CE_CLK_CTRL

// n: bus clock/(2^(n+1))

// CE_VERSION

pub const SOFT_RST_OFF: c_int = 0;
extern "C" {
    pub fn __raw_readl(reg: addr +) -> return;
}

extern "C" {
    pub fn sh_mmcif_boot_cmd_poll(_arg: base, _arg: 0x00010000) -> return;
}
// CMD13 - Status
// CMD17 - Read
// In data transfer mode: Set clock to Bus clock/4 (about 20Mhz)
// CMD9 - Get CSD
// CMD7 - Select the card
// CMD16 - Set the block size
// reset
// byte swap
// Set block size in MMCIF hardware
// Enable the clock, set it to Bus clock/256 (about 325Khz).
// CMD0
// CMD1 - Get OCR
// CMD2 - Get CID
// CMD3 - Set card relative address
