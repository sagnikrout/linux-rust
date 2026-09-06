//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_hw_utils.h
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
// Atlantic Network Driver
//
// Copyright (C) 2014-2019 aQuantia Corporation
// Copyright (C) 2019-2020 Marvell International Ltd.
//
// File aq_hw_utils.h: Declaration of helper functions used across hardware
// layer.
//

extern "C" {
    pub fn aq_hw_read_reg_bit(aq_hw: *mut aq_hw_s, addr: u32, msk: u32, shift: u32) -> u32;
}
extern "C" {
    pub fn aq_hw_read_reg(hw: *mut aq_hw_s, reg: u32) -> u32;
}
extern "C" {
    pub fn aq_hw_write_reg(hw: *mut aq_hw_s, reg: u32, value: u32);
}
extern "C" {
    pub fn aq_hw_read_reg64(hw: *mut aq_hw_s, reg: u32) -> u64;
}
extern "C" {
    pub fn aq_hw_write_reg64(hw: *mut aq_hw_s, reg: u32, value: u64);
}
extern "C" {
    pub fn aq_hw_invalidate_descriptor_cache(hw: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn aq_hw_err_from_flags(hw: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn aq_hw_num_tcs(hw: *mut aq_hw_s) -> c_int;
}
extern "C" {
    pub fn aq_hw_q_per_tc(hw: *mut aq_hw_s) -> c_int;
}
