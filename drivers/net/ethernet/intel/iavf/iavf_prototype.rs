//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf_prototype.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

// Prototypes for shared code functions that are not in
// the standard function pointer structures.  These are
// mostly because they are needed even before the init
// has happened and will assist in the early SW and FW
// setup.
//
// adminq functions
extern "C" {
    pub fn iavf_init_adminq(hw: *mut iavf_hw) -> iavf_status;
}
extern "C" {
    pub fn iavf_shutdown_adminq(hw: *mut iavf_hw) -> iavf_status;
}
extern "C" {
    pub fn iavf_asq_done(hw: *mut iavf_hw) -> bool;
}
// debug function for adminq
extern "C" {
    pub fn iavf_check_asq_alive(hw: *mut iavf_hw) -> bool;
}
extern "C" {
    pub fn iavf_aq_queue_shutdown(hw: *mut iavf_hw, unloading: bool) -> iavf_status;
}
