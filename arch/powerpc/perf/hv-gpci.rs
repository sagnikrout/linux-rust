//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/perf/hv-gpci.h
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
// counter info version => fw version/reference (spec version)
//
// 8 => power8 (1.07)
// [7 is skipped by spec 1.07]
// 6 => TLBIE (1.07)
// 5 => v7r7m0.phyp (1.05)
// [4 skipped]
// 3 => v7r6m0.phyp (?)
// [1,2 skipped]
// 0 => v7r{2,3,4}m0.phyp (?)
//
pub const COUNTER_INFO_VERSION_CURRENT: c_uint = 0x8;
// capability mask masks.

// Macro flag: #define ENABLE_EVENTS_COUNTERINFO_V6

