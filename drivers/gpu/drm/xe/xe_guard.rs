//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guard.h
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
// Copyright © 2025 Intel Corporation
//

//
// struct xe_guard - Simple logic to protect a feature.
//
// Implements simple semaphore-like logic that can be used to lockdown the
// feature unless it is already in use.  Allows enabling of the otherwise
// incompatible features, where we can't follow the strict owner semantics
// required by the &rw_semaphore.
//
// NOTE! It shouldn't be used to protect a data, use &rw_semaphore instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guard {
//
// @counter: implements simple exclusive/lockdown logic:
// if == 0 then guard/feature is idle/not in use,
// if < 0 then feature is active and can't be locked-down,
// if > 0 then feature is lockded-down and can't be activated.
//
    pub counter: c_int,
// @name: the name of the guard (useful for debug)
    pub name: *const c_char,
// @owner: the info about the last owner of the guard (for debug)
    pub owner: *mut c_void,
// @lock: protects guard's data
    pub lock: spinlock_t,
}

//
// xe_guard_init() - Initialize the guard.
// @guard: the &xe_guard to init
// @name: name of the guard
//
// xe_guard_arm() - Arm the guard for the exclusive/lockdown mode.
// @guard: the &xe_guard to arm
// @lockdown: arm for lockdown(true) or exclusive(false) mode
// @who: optional owner info (for debug only)
//
// Multiple lockdown requests are allowed.
// Only single exclusive access can be granted.
// Will fail if the guard is already in exclusive mode.
// On success, must call the xe_guard_disarm() to release.
//
// Return: 0 on success or a negative error code on failure.
//
// xe_guard_disarm() - Disarm the guard from exclusive/lockdown mode.
// @guard: the &xe_guard to disarm
// @lockdown: disarm from lockdown(true) or exclusive(false) mode
//
// Return: true if successfully disarmed or false in case of mismatch.
//
// xe_guard_mode_str() - Convert guard mode into a string.
// @lockdown: flag used to select lockdown or exclusive mode
//
// Return: "lockdown" or "exclusive" string.
//
