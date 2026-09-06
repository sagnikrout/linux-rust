//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/internals.h
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
// Copyright (c) 2018 - Bootlin
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//
// Header containing internal definitions to be used only by core files.
// NAND controller drivers should not include this file.
//

//
// NAND Flash Manufacturer ID Codes
//
pub const NAND_MFR_AMD: c_uint = 0x01;
pub const NAND_MFR_ATO: c_uint = 0x9b;
pub const NAND_MFR_EON: c_uint = 0x92;
pub const NAND_MFR_ESMT: c_uint = 0xc8;
pub const NAND_MFR_FUJITSU: c_uint = 0x04;
pub const NAND_MFR_HYNIX: c_uint = 0xad;
pub const NAND_MFR_INTEL: c_uint = 0x89;
pub const NAND_MFR_MACRONIX: c_uint = 0xc2;
pub const NAND_MFR_MICRON: c_uint = 0x2c;
pub const NAND_MFR_NATIONAL: c_uint = 0x8f;
pub const NAND_MFR_RENESAS: c_uint = 0x07;
pub const NAND_MFR_SAMSUNG: c_uint = 0xec;
pub const NAND_MFR_SANDISK: c_uint = 0x45;
pub const NAND_MFR_STMICRO: c_uint = 0x20;
// Kioxia is new name of Toshiba memory.
pub const NAND_MFR_TOSHIBA: c_uint = 0x98;
pub const NAND_MFR_WINBOND: c_uint = 0xef;
//
// struct nand_manufacturer_ops - NAND Manufacturer operations
// @detect: detect the NAND memory organization and capabilities
// @init: initialize all vendor specific fields (like the ->read_retry()
// implementation) if any.
// @cleanup: the ->init() function may have allocated resources, ->cleanup()
// is here to let vendor specific code release those resources.
// @fixup_onfi_param_page: apply vendor specific fixups to the ONFI parameter
// page. This is called after the checksum is verified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_manufacturer_ops {
    pub chip): *mut *mut void (detect)(struct nand_chip,
    pub chip): *mut *mut int (init)(struct nand_chip,
    pub chip): *mut *mut void (cleanup)(struct nand_chip,
    pub p): *mut nand_onfi_params,
}

//
// struct nand_manufacturer_desc - NAND Flash Manufacturer descriptor
// @name: Manufacturer name
// @id: manufacturer ID code of device.
// @ops: manufacturer operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_manufacturer_desc {
    pub id: c_int,
    pub name: *mut c_char,
    pub ops: *const nand_manufacturer_ops,
}

// MLC pairing schemes
// Core functions
extern "C" {
    pub fn nand_bbm_get_next_page(chip: *mut nand_chip, page: c_int) -> c_int;
}
extern "C" {
    pub fn nand_markbad_bbm(chip: *mut nand_chip, ofs: loff_t) -> c_int;
}
extern "C" {
    pub fn nand_get_features(chip: *mut nand_chip, addr: c_int, subfeature_param: *mut u8) -> c_int;
}
extern "C" {
    pub fn nand_set_features(chip: *mut nand_chip, addr: c_int, subfeature_param: *mut u8) -> c_int;
}
extern "C" {
    pub fn nand_decode_ext_id(chip: *mut nand_chip);
}
extern "C" {
    pub fn panic_nand_wait(chip: *mut nand_chip, timeo: c_ulong);
}
extern "C" {
    pub fn sanitize_string(s: *mut u8, len: usize);
}
// BBT functions
extern "C" {
    pub fn nand_markbad_bbt(chip: *mut nand_chip, offs: loff_t) -> c_int;
}
extern "C" {
    pub fn nand_isreserved_bbt(chip: *mut nand_chip, offs: loff_t) -> c_int;
}
extern "C" {
    pub fn nand_isbad_bbt(chip: *mut nand_chip, offs: loff_t, allowbbt: c_int) -> c_int;
}
// Legacy
extern "C" {
    pub fn nand_legacy_set_defaults(chip: *mut nand_chip);
}
extern "C" {
    pub fn nand_legacy_adjust_cmdfunc(chip: *mut nand_chip);
}
extern "C" {
    pub fn nand_legacy_check_hooks(chip: *mut nand_chip) -> c_int;
}
// ONFI functions
extern "C" {
    pub fn onfi_crc16(crc: u16, p: *const u8, len: usize) -> u16;
}
extern "C" {
    pub fn nand_onfi_detect(chip: *mut nand_chip) -> c_int;
}
// JEDEC functions
extern "C" {
    pub fn nand_jedec_detect(chip: *mut nand_chip) -> c_int;
}
