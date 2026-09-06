//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/switch_to.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// __switch_to - switch execution of a task
// @prev:	The task previously executed.
// @next:	The task to begin executing.
// @sched_ra:	__schedule return address.
// @sched_cfa:	__schedule call frame address.
//
// This function is used whilst scheduling to save the context of prev & load
// the context of next. Returns prev.
//
// For newly created kernel threads switch_to() will return to
// ret_from_kernel_thread, newly created user threads to ret_from_fork.
// That is, everything following __switch_to() will be skipped for new threads.
// So everything that matters to new threads should be placed before __switch_to().
//

