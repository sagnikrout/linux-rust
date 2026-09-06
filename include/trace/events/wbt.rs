//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/wbt.h
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
// wbt_stat - trace stats for blk_wb
// @stat: array of read/write stats
//
// wbt_lat - trace latency event
// @lat: latency trigger
//
// wbt_step - trace wb event step
// @msg: context message
// @step: the current scale step count
// @window: the current monitoring window
// @bg: the current background queue limit
// @normal: the current normal writeback limit
// @max: the current max throughput writeback limit
//
// wbt_timer - trace wb timer event
// @status: timer state status
// @step: the current scale step count
// @inflight: tracked writes inflight
//

// This part must be outside protection
