//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/brcmnand/brcmnand.h
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
// Copyright © 2015 Broadcom Corporation
//

// Special register offset constant to intercept a non-MMIO access
// to the flash cache register space. This is intentionally large
// not to overlap with an existing offset.
//
pub const BRCMNAND_NON_MMIO_FC_ADDR: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmnand_soc {
    pub soc): *mut *mut bool (ctlrdy_ack)(struct brcmnand_soc,
    pub en): *mut *mut *mut void (ctlrdy_set_enabled)(struct brcmnand_soc soc, bool,
    pub is_param): bool,
    pub fc_words): *mut *mut u32 buffer, int,
    pub ops: *const brcmnand_io_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmnand_io_ops {
    pub offset): *mut *mut *mut u32 (read_reg)(struct brcmnand_soc soc, u32,
    pub offset): *mut *mut *mut void (write_reg)(struct brcmnand_soc soc, u32 val, u32,
}

//
// MIPS endianness is configured by boot strap, which also reverses all
// bus endianness (i.e., big-endian CPU + big endian bus ==> native
// endian I/O).
//
// Other architectures (e.g., ARM) either do not support big endian, or
// else leave I/O in little endian mode.
//
extern "C" {
    pub fn __raw_readl(_arg: addr) -> return;
}
extern "C" {
    pub fn readl_relaxed(_arg: addr) -> return;
}
// See brcmnand_readl() comments
extern "C" {
    pub fn brcmnand_probe(pdev: *mut platform_device, soc: *mut brcmnand_soc) -> c_int;
}
extern "C" {
    pub fn brcmnand_remove(pdev: *mut platform_device);
}
