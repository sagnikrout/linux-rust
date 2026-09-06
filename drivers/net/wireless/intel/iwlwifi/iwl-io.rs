//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-io.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2018-2021, 2025-2026 Intel Corporation
//

// Macro flag: #define __iwl_io_h__

extern "C" {
    pub fn iwl_write8(trans: *mut iwl_trans, ofs: u32, val: u8);
}
extern "C" {
    pub fn iwl_write32(trans: *mut iwl_trans, ofs: u32, val: u32);
}
extern "C" {
    pub fn iwl_write64(trans: *mut iwl_trans, ofs: u64, val: u64);
}
extern "C" {
    pub fn iwl_read32(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_poll_bits_mask(_arg: trans, _arg: addr, _arg: bits, _arg: bits, _arg: timeout) -> return;
}
extern "C" {
    pub fn iwl_read_direct32(trans: *mut iwl_trans, reg: u32) -> u32;
}
extern "C" {
    pub fn iwl_write_direct32(trans: *mut iwl_trans, reg: u32, value: u32);
}
extern "C" {
    pub fn iwl_write_direct64(trans: *mut iwl_trans, reg: u64, value: u64);
}
extern "C" {
    pub fn iwl_read_prph_no_grab(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_read_prph(trans: *mut iwl_trans, ofs: u32) -> u32;
}
extern "C" {
    pub fn iwl_write_prph_no_grab(trans: *mut iwl_trans, ofs: u32, val: u32);
}
extern "C" {
    pub fn iwl_write_prph64_no_grab(trans: *mut iwl_trans, ofs: u64, val: u64);
}
extern "C" {
    pub fn iwl_set_bits_prph(trans: *mut iwl_trans, ofs: u32, mask: u32);
}
extern "C" {
    pub fn iwl_clear_bits_prph(trans: *mut iwl_trans, ofs: u32, mask: u32);
}
extern "C" {
    pub fn iwl_force_nmi(trans: *mut iwl_trans);
}
// Error handling
extern "C" {
    pub fn iwl_dump_fh(trans: *mut iwl_trans, buf: *mut c_char) -> c_int;
}
//
// UMAC periphery address space changed from 0xA00000 to 0xD00000 starting from
// device family AX200. So peripheries used in families above and below AX200
// should go through iwl_..._umac_..._prph.
//
extern "C" {
    pub fn iwl_read_prph(_arg: trans, trans->mac_cfg->umac_prph_offset: ofs +) -> return;
}
