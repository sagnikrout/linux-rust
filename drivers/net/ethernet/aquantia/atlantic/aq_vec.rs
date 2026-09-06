//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/aq_vec.h
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
// File aq_vec.h: Definition of common structures for vector of Rx and Tx rings.
// Declaration of functions for Rx and Tx rings.
//

extern "C" {
    pub fn aq_vec_isr(irq: c_int, private: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn aq_vec_isr_legacy(irq: c_int, private: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn aq_vec_deinit(self: *mut aq_vec_s);
}
extern "C" {
    pub fn aq_vec_free(self: *mut aq_vec_s);
}
extern "C" {
    pub fn aq_vec_ring_free(self: *mut aq_vec_s);
}
extern "C" {
    pub fn aq_vec_start(self: *mut aq_vec_s) -> c_int;
}
extern "C" {
    pub fn aq_vec_stop(self: *mut aq_vec_s);
}
extern "C" {
    pub fn aq_vec_is_valid_tc(self: *mut aq_vec_s, tc: c_uint) -> bool;
}
extern "C" {
    pub fn aq_vec_get_sw_stats(self: *mut aq_vec_s, tc: c_uint, data: *mut u64) -> c_uint;
}
