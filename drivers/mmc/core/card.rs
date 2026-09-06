//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/card.h
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
// Private header for the mmc subsystem
//
// Copyright (C) 2016 Linaro Ltd
//
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

// Card states

//
// The world is not perfect and supplies us with broken mmc/sdio devices.
// For at least some of these bugs we need a work-around.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_fixup {
// CID-specific fields.
    pub name: *const c_char,
// Valid revision range
    pub rev_end: u64 rev_start,,
    pub manfid: c_uint,
    pub oemid: c_ushort,
// Manufacturing date
    pub year: c_ushort,
    pub month: c_uchar,
// SDIO-specific fields. You can use SDIO_ANY_ID here of course
    pub cis_device: u16 cis_vendor,,
// for MMC cards
    pub ext_csd_rev: c_uint,
// Match against functions declared in device tree
    pub of_compatible: *const c_char,
    pub data): *mut *mut *mut void (vendor_fixup)(struct mmc_card card, int,
    pub data: c_int,
}

pub const CID_MANFID_SANDISK: c_uint = 0x2;
pub const CID_MANFID_SANDISK_SD: c_uint = 0x3;
pub const CID_MANFID_ATP: c_uint = 0x9;
pub const CID_MANFID_TOSHIBA: c_uint = 0x11;
pub const CID_MANFID_GIGASTONE: c_uint = 0x12;
pub const CID_MANFID_MICRON: c_uint = 0x13;
pub const CID_MANFID_SAMSUNG: c_uint = 0x15;
pub const CID_MANFID_APACER: c_uint = 0x27;
pub const CID_MANFID_SANDISK_MMC: c_uint = 0x45;
pub const CID_MANFID_SWISSBIT: c_uint = 0x5D;
pub const CID_MANFID_KINGSTON: c_uint = 0x70;
pub const CID_MANFID_HYNIX: c_uint = 0x90;
pub const CID_MANFID_KINGSTON_SD: c_uint = 0x9F;
pub const CID_MANFID_NUMONYX: c_uint = 0xFE;

//
// Unconditionally quirk add/remove.
//
// We have TI wl1251 attached to this mmc. Pass this
// information to the SDIO core because it can't be
// probed by normal methods.
//
// Quirk add/remove for MMC products.
//
// Quirk add/remove for SD products.
//
