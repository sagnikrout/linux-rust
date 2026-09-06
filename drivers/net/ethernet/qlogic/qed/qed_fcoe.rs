//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_fcoe.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_fcoe_info {
    pub /: *mut *mut spinlock_t lock; / Connection resources.,
    pub free_list: list_head,
}

extern "C" {
    pub fn qed_fcoe_alloc(p_hwfn: *mut qed_hwfn) -> c_int;
}
extern "C" {
    pub fn qed_fcoe_setup(p_hwfn: *mut qed_hwfn);
}
extern "C" {
    pub fn qed_fcoe_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_get_protocol_stats_fcoe(): Fills provided statistics
// struct with statistics.
//
// @cdev: Qed dev pointer.
// @stats: Points to struct that will be filled with statistics.
// @is_atomic: Hint from the caller - if the func can sleep or not.
//
// Context: The function should not sleep in case is_atomic == true.
// Return: Void.
//

