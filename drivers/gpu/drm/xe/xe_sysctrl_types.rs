//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sysctrl_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2026 Intel Corporation
//

//
// struct xe_sysctrl - System Controller driver context
//
// This structure maintains the runtime state for System Controller
// communication. All fields are initialized during xe_sysctrl_init()
// and protected appropriately for concurrent access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_sysctrl {
// @mmio: MMIO region for system control registers
    pub mmio: *mut xe_mmio,
// @cmd_lock: Mutex protecting mailbox command operations
    pub cmd_lock: mutex,
// @work: Pending events worker
    pub work: work_struct,
// @event_lock: Mutex protecting pending events
    pub event_lock: mutex,
}
