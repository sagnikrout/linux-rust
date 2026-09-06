//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/mmiowb.h
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
// Generic implementation of mmiowb() tracking for spinlocks.
//
// If your architecture doesn't ensure that writes to an I/O peripheral
// within two spinlocked sections on two different CPUs are seen by the
// peripheral in the order corresponding to the lock handover, then you
// need to follow these FIVE easy steps:
//
// 1. Implement mmiowb() (and arch_mmiowb_state() if you're fancy)
// in asm/mmiowb.h, then #include this file
// 2. Ensure your I/O write accessors call mmiowb_set_pending()
// 3. Select ARCH_HAS_MMIOWB
// 4. Untangle the resulting mess of header files
// 5. Complain to your architects
//

