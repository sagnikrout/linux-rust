//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/xtensa/include/uapi/asm/mman.h
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
pub const MADV_DODUMP: c_int = 17;
pub const MADV_DOFORK: c_int = 11;
pub const MADV_DONTDUMP: c_int = 16;
pub const MADV_DONTFORK: c_int = 10;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_FREE: c_int = 8;
pub const MADV_HUGEPAGE: c_int = 14;
pub const MADV_MERGEABLE: c_int = 12;
pub const MADV_NOHUGEPAGE: c_int = 15;
pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_REMOVE: c_int = 9;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_UNMERGEABLE: c_int = 13;
pub const MADV_WILLNEED: c_int = 3;
pub const MAP_ANONYMOUS: c_uint = 0x0800;
pub const MAP_DENYWRITE: c_uint = 0x2000;
pub const MAP_EXECUTABLE: c_uint = 0x4000;
pub const MAP_FILE: c_int = 0;
pub const MAP_FIXED: c_uint = 0x010;
pub const MAP_GROWSDOWN: c_uint = 0x1000;
pub const MAP_HUGETLB: c_uint = 0x80000;
pub const MAP_LOCKED: c_uint = 0x8000;
pub const MAP_NONBLOCK: c_uint = 0x20000;
pub const MAP_NORESERVE: c_uint = 0x0400;
pub const MAP_POPULATE: c_uint = 0x10000;
pub const MAP_STACK: c_uint = 0x40000;
pub const PROT_EXEC: c_uint = 0x4;
pub const PROT_GROWSDOWN: c_uint = 0x01000000;
pub const PROT_GROWSUP: c_uint = 0x02000000;
pub const PROT_NONE: c_uint = 0x0;
pub const PROT_READ: c_uint = 0x1;
pub const PROT_SEM: c_uint = 0x10;
pub const PROT_WRITE: c_uint = 0x2;
// MADV_HWPOISON is undefined on xtensa, fix it for perf
pub const MADV_HWPOISON: c_int = 100;
// MADV_SOFT_OFFLINE is undefined on xtensa, fix it for perf
pub const MADV_SOFT_OFFLINE: c_int = 101;
// MAP_32BIT is undefined on xtensa, fix it for perf
pub const MAP_32BIT: c_int = 0;
// MAP_UNINITIALIZED is undefined on xtensa, fix it for perf
pub const MAP_UNINITIALIZED: c_int = 0;
