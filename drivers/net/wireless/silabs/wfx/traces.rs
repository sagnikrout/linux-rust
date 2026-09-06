//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/traces.h
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
// Tracepoints definitions.
//
// Copyright (c) 2018-2020, Silicon Laboratories, Inc.
//

// The hell below need some explanations. For each symbolic number, we need to define it with
// TRACE_DEFINE_ENUM() and in a list for __print_symbolic.
//
// 1. Define a new macro that call TRACE_DEFINE_ENUM():
//
// #define xxx_name(sym) TRACE_DEFINE_ENUM(sym);
//
// 2. Define list of all symbols:
//
// #define list_names     \
// ...                 \
// xxx_name(XXX)       \
// ...
//
// 3. Instantiate that list_names:
//
// list_names
//
// 4. Redefine xxx_name() as an entry of array for __print_symbolic()
//
// #undef xxx_name
// #define xxx_name(msg) { msg, #msg },
//
// 5. list_name can now nearly be used with __print_symbolic() but, __print_symbolic() dislike
// last comma of list. So we define a new list with a dummy element:
//
// #define list_for_print_symbolic list_names { -1, NULL }
//

// Keep sync with wfx_rates definition in main.c

// This part must be outside protection

