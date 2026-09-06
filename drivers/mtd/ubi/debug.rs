//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/ubi/debug.h
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
// Copyright (c) International Business Machines Corp., 2006
//
// Author: Artem Bityutskiy (Битюцкий Артём)
//
extern "C" {
    pub fn ubi_dump_flash(ubi: *mut ubi_device, pnum: c_int, offset: c_int, len: c_int);
}
extern "C" {
    pub fn ubi_dump_ec_hdr(ec_hdr: *const ubi_ec_hdr);
}
extern "C" {
    pub fn ubi_dump_vid_hdr(vid_hdr: *const ubi_vid_hdr);
}

// General debugging messages

// Messages from the eraseblock association sub-system

// Messages from the wear-leveling sub-system

// Messages from the input/output sub-system

// Initialization and build messages

extern "C" {
    pub fn ubi_dump_vol_info(vol: *const ubi_volume);
}
extern "C" {
    pub fn ubi_dump_vtbl_record(r: *const ubi_vtbl_record, idx: c_int);
}
extern "C" {
    pub fn ubi_dump_av(av: *const ubi_ainf_volume);
}
extern "C" {
    pub fn ubi_dump_aeb(aeb: *const ubi_ainf_peb, type: c_int);
}
extern "C" {
    pub fn ubi_dump_mkvol_req(req: *const ubi_mkvol_req);
}
extern "C" {
    pub fn ubi_debugfs_init() -> c_int;
}
extern "C" {
    pub fn ubi_debugfs_exit();
}
extern "C" {
    pub fn ubi_debugfs_init_dev(ubi: *mut ubi_device) -> c_int;
}
extern "C" {
    pub fn ubi_debugfs_exit_dev(ubi: *mut ubi_device);
}
//
// The following function is a legacy implementation of UBI fault-injection
// hook. When using more powerful fault injection capabilities, the legacy
// fault injection interface should be retained.
//
extern "C" {
    pub fn ubi_dbg_power_cut(ubi: *mut ubi_device, caller: c_int) -> c_int;
}
//
// MASK_XXX: Mask for emulate_failures in ubi_debug_info.The mask is used to
// precisely control the type and process of fault injection.
//
// Emulate a power cut when writing EC/VID header

// Emulate a power cut when writing data

// Emulate bit-flips

// Emulate ecc error

// Emulates -EIO during data read

// Emulates -EIO during data write

// Emulates -EIO during erase a PEB

// Return UBI_IO_FF when reading EC/VID header

// Return UBI_IO_FF_BITFLIPS when reading EC/VID header

// Return UBI_IO_BAD_HDR when reading EC/VID header

// Return UBI_IO_BAD_HDR_EBADMSG when reading EC/VID header

extern "C" {
    pub fn should_fail_eccerr() -> bool;
}
extern "C" {
    pub fn should_fail_bitflips() -> bool;
}
extern "C" {
    pub fn should_fail_read_failure() -> bool;
}
extern "C" {
    pub fn should_fail_write_failure() -> bool;
}
extern "C" {
    pub fn should_fail_erase_failure() -> bool;
}
extern "C" {
    pub fn should_fail_power_cut() -> bool;
}
extern "C" {
    pub fn should_fail_io_ff() -> bool;
}
extern "C" {
    pub fn should_fail_io_ff_bitflips() -> bool;
}
extern "C" {
    pub fn should_fail_bad_hdr() -> bool;
}
extern "C" {
    pub fn should_fail_bad_hdr_ebadmsg() -> bool;
}
extern "C" {
    pub fn should_fail_bitflips() -> return;
}
extern "C" {
    pub fn should_fail_write_failure() -> return;
}
extern "C" {
    pub fn should_fail_erase_failure() -> return;
}
extern "C" {
    pub fn should_fail_power_cut() -> return;
}
extern "C" {
    pub fn should_fail_read_failure() -> return;
}
extern "C" {
    pub fn should_fail_eccerr() -> return;
}
extern "C" {
    pub fn should_fail_io_ff() -> return;
}
extern "C" {
    pub fn should_fail_io_ff_bitflips() -> return;
}
extern "C" {
    pub fn should_fail_bad_hdr() -> return;
}
extern "C" {
    pub fn should_fail_bad_hdr_ebadmsg() -> return;
}

//
// ubi_dbg_is_power_cut - if it is time to emulate power cut.
// @ubi: UBI device description object
//
// Returns true if power cut should be emulated, otherwise returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_power_cut(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_bitflip - if it is time to emulate a bit-flip.
// @ubi: UBI device description object
//
// Returns true if a bit-flip should be emulated, otherwise returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_bitflip(_arg: ubi) -> return;
}
//
// ubi_dbg_is_write_failure - if it is time to emulate a write failure.
// @ubi: UBI device description object
//
// Returns true if a write failure should be emulated, otherwise returns
// false.
//
extern "C" {
    pub fn ubi_dbg_fail_write(_arg: ubi) -> return;
}
//
// ubi_dbg_is_erase_failure - if its time to emulate an erase failure.
// @ubi: UBI device description object
//
// Returns true if an erase failure should be emulated, otherwise returns
// false.
//
extern "C" {
    pub fn ubi_dbg_fail_erase(_arg: ubi) -> return;
}
//
// ubi_dbg_is_eccerr - if it is time to emulate ECC error.
// @ubi: UBI device description object
//
// Returns true if a ECC error should be emulated, otherwise returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_eccerr(_arg: ubi) -> return;
}
//
// ubi_dbg_is_read_failure - if it is time to emulate a read failure.
// @ubi: UBI device description object
//
// Returns true if a read failure should be emulated, otherwise returns
// false.
//
extern "C" {
    pub fn ubi_dbg_fail_read(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_ff - if it is time to emulate that read region is only 0xFF.
// @ubi: UBI device description object
//
// Returns true if read region should be emulated 0xFF, otherwise
// returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_ff(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_ff_bitflips - if it is time to emulate that read region is only 0xFF
// with error reported by the MTD driver
//
// @ubi: UBI device description object
//
// Returns true if read region should be emulated 0xFF and error
// reported by the MTD driver, otherwise returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_ff_bitflips(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_bad_hdr - if it is time to emulate a bad header
// @ubi: UBI device description object
//
// Returns true if a bad header error should be emulated, otherwise
// returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_bad_hdr(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_bad_hdr_ebadmsg - if it is time to emulate a bad header with
// ECC error.
//
// @ubi: UBI device description object
//
// Returns true if a bad header with ECC error should be emulated, otherwise
// returns false.
//
extern "C" {
    pub fn ubi_dbg_fail_bad_hdr_ebadmsg(_arg: ubi, _arg: caller) -> return;
}
//
// ubi_dbg_is_bgt_disabled - if the background thread is disabled.
// @ubi: UBI device description object
//
// Returns non-zero if the UBI background thread is disabled for testing
// purposes.
//
