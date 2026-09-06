//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/user_events.h
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
//
// Copyright (c) 2022, Microsoft Corporation.
//
// Authors:
// Beau Belgrave <beaub@linux.microsoft.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_event_mm {
    pub mms_link: list_head,
    pub enablers: list_head,
    pub mm: *mut mm_struct,
// Used for one-shot lists, protected by event_mutex
    pub next: *mut user_event_mm,
    pub refcnt: refcount_t,
    pub tasks: refcount_t,
    pub put_rwork: rcu_work,
}

extern "C" {
    pub fn user_event_mm_remove(t: *mut task_struct);
}

