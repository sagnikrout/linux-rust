//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/prio.h
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
pub const MAX_NICE: c_int = 19;

//
// Priority of a process goes from 0..MAX_PRIO-1, valid RT
// priority is 0..MAX_RT_PRIO-1, and SCHED_NORMAL/SCHED_BATCH
// tasks are in the range MAX_RT_PRIO..MAX_PRIO-1. Priority
// values are inverted: lower p->prio value means higher priority.
//
pub const MAX_RT_PRIO: c_int = 100;
pub const MAX_DL_PRIO: c_int = 0;

//
// Convert user-nice values [ -20 ... 0 ... 19 ]
// to static priority [ MAX_RT_PRIO..MAX_PRIO-1 ],
// and back.
//

//
// Convert nice value [19,-20] to rlimit style value [1,40].
//
// Convert rlimit style value [1,40] to nice value [-20, 19].
//
