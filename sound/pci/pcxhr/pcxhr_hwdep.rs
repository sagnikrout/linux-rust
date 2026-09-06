//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/pcxhr/pcxhr_hwdep.h
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
// Driver for Digigram pcxhr compatible soundcards
//
// definitions and makros for basic card access
//
// Copyright (c) 2004 by Digigram <alsa@digigram.com>
//
// firmware status codes
pub const PCXHR_FIRMWARE_XLX_INT_INDEX: c_int = 0;
pub const PCXHR_FIRMWARE_XLX_COM_INDEX: c_int = 1;
pub const PCXHR_FIRMWARE_DSP_EPRM_INDEX: c_int = 2;
pub const PCXHR_FIRMWARE_DSP_BOOT_INDEX: c_int = 3;
pub const PCXHR_FIRMWARE_DSP_MAIN_INDEX: c_int = 4;
pub const PCXHR_FIRMWARE_FILES_MAX_INDEX: c_int = 5;
// exported
extern "C" {
    pub fn pcxhr_setup_firmware(mgr: *mut pcxhr_mgr) -> c_int;
}
extern "C" {
    pub fn pcxhr_reset_board(mgr: *mut pcxhr_mgr);
}
