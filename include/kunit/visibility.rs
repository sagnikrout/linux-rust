//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/visibility.h
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
// KUnit API to allow symbols to be conditionally visible during KUnit
// testing
//
// Copyright (C) 2022, Google LLC.
// Author: Rae Moar <rmoar@google.com>
//

//
// VISIBLE_IF_KUNIT - A macro that sets symbols to be static if
// CONFIG_KUNIT is not enabled. Otherwise if CONFIG_KUNIT is enabled
// there is no change to the symbol definition.
//
// Macro flag: #define VISIBLE_IF_KUNIT
//
// EXPORT_SYMBOL_IF_KUNIT(symbol) - Exports symbol into
// EXPORTED_FOR_KUNIT_TESTING namespace only if CONFIG_KUNIT is
// enabled. Must use MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING")
// in test file in order to use symbols.
// @symbol: the symbol identifier to export
//

// Macro flag: #define EXPORT_SYMBOL_IF_KUNIT(symbol)

