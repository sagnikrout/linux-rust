//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_nvm.h
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
// Copyright (c)  2018 Intel Corporation
extern "C" {
    pub fn igc_read_mac_addr(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_read_nvm_eerd(hw: *mut igc_hw, offset: u16, words: u16, data: *mut u16) -> i32;
}
extern "C" {
    pub fn igc_validate_nvm_checksum(hw: *mut igc_hw) -> i32;
}
extern "C" {
    pub fn igc_update_nvm_checksum(hw: *mut igc_hw) -> i32;
}
