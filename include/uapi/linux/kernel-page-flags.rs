//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/kernel-page-flags.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Stable page flag bits exported to user space
//
pub const KPF_LOCKED: c_int = 0;

pub const KPF_REFERENCED: c_int = 2;
pub const KPF_UPTODATE: c_int = 3;
pub const KPF_DIRTY: c_int = 4;
pub const KPF_LRU: c_int = 5;
pub const KPF_ACTIVE: c_int = 6;
pub const KPF_SLAB: c_int = 7;
pub const KPF_WRITEBACK: c_int = 8;
pub const KPF_RECLAIM: c_int = 9;
pub const KPF_BUDDY: c_int = 10;
// 11-20: new additions in 2.6.31
pub const KPF_MMAP: c_int = 11;
pub const KPF_ANON: c_int = 12;
pub const KPF_SWAPCACHE: c_int = 13;
pub const KPF_SWAPBACKED: c_int = 14;
pub const KPF_COMPOUND_HEAD: c_int = 15;
pub const KPF_COMPOUND_TAIL: c_int = 16;
pub const KPF_HUGE: c_int = 17;
pub const KPF_UNEVICTABLE: c_int = 18;
pub const KPF_HWPOISON: c_int = 19;
pub const KPF_NOPAGE: c_int = 20;
pub const KPF_KSM: c_int = 21;
pub const KPF_THP: c_int = 22;
pub const KPF_OFFLINE: c_int = 23;
pub const KPF_ZERO_PAGE: c_int = 24;
pub const KPF_IDLE: c_int = 25;
pub const KPF_PGTABLE: c_int = 26;
