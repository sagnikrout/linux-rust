//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/user_exit_info_common.h
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
// Define struct user_exit_info which is shared between BPF and userspace parts
// to communicate exit status and other information.
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uei_sizes {
    UEI_REASON_LEN		= 128,
    UEI_MSG_LEN		= 1024,
    UEI_DUMP_DFL_LEN	= 32768,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_exit_info {
    pub kind: c_int,
//
// CPU that triggered the exit, or -1 if unset (e.g. running on an
// older kernel that does not expose this field).
//
    pub exit_cpu: i32,
    pub exit_code: i64,
    pub reason: [c_char; UEI_REASON_LEN],
    pub msg: [c_char; UEI_MSG_LEN],
}
