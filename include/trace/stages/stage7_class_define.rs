//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/stages/stage7_class_define.h
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
// Stage 7 definitions for creating trace events

//
// The TRACE_FLAG_* are enums. Instead of using TRACE_DEFINE_ENUM(),
// use their hardcoded values. These values are parsed by user space
// tooling elsewhere so they will never change.
//
// See "enum trace_flag_type" in linux/trace_events.h:
// TRACE_FLAG_HARDIRQ
// TRACE_FLAG_SOFTIRQ
//
// This is what is displayed in the format files

//
// The below is not executed in the kernel. It is only what is
// displayed in the print format for userspace to parse.
//

