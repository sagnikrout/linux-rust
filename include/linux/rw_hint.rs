//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rw_hint.h
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

// Block storage write lifetime hint values.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rw_hint {
    WRITE_LIFE_NOT_SET	= RWH_WRITE_LIFE_NOT_SET,
    WRITE_LIFE_NONE		= RWH_WRITE_LIFE_NONE,
    WRITE_LIFE_SHORT	= RWH_WRITE_LIFE_SHORT,
    WRITE_LIFE_MEDIUM	= RWH_WRITE_LIFE_MEDIUM,
    WRITE_LIFE_LONG		= RWH_WRITE_LIFE_LONG,
    WRITE_LIFE_EXTREME	= RWH_WRITE_LIFE_EXTREME,
    WRITE_LIFE_HINT_NR,
    } __packed;

// Sparse ignores __packed annotations on enums, hence the #ifndef below.
    static_assert(sizeof(enum rw_hint) == 1);

