//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/rqspinlock.h
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
// Hardcode res_smp_cond_load_acquire implementations for arm64 to a custom
// version based on [0]. In rqspinlock code, our conditional expression involves
// checking the value _and_ additionally a timeout. However, on arm64, the
// WFE-based implementation may never spin again if no stores occur to the
// locked byte in the lock word. As such, we may be stuck forever if
// event-stream based unblocking is not available on the platform for WFE spin
// loops (arch_timer_evtstrm_available).
//
// Once support for smp_cond_load_acquire_timewait [0] lands, we can drop this
// copy-paste.
//
// While we rely on the implementation to amortize the cost of sampling
// cond_expr for us, it will not happen when event stream support is
// unavailable, time_expr check is amortized. This is not the common case, and
// it would be difficult to fit our logic in the time_expr_ns >= time_limit_ns
// comparison, hence just let it be. In case of event-stream, the loop is woken
// up at microsecond granularity.
//
// [0]: https://lore.kernel.org/lkml/20250203214911.898276-1-ankur.a.arora@oracle.com
//

pub const smp_cond_time_check_count: c_int = 200;

