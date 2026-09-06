//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/asm-generic/mman-common-tools.h
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

// We need this because we need to have tools/include/uapi/ included in the tools
// header search path to get access to stuff that is not yet in the system's
// copy of the files in that directory, but since this cset:
//
// 746c9398f5ac ("arch: move common mmap flags to linux/mman.h")
//
// We end up making sys/mman.h, that is in the system headers, to not find the
// MAP_SHARED and MAP_PRIVATE defines because they are not anymore in our copy
// of asm-generic/mman-common.h. So we define them here and include this header
// from each of the per arch mman.h headers.
//

pub const MAP_SHARED: c_uint = 0x01		/* Share changes */;
pub const MAP_PRIVATE: c_uint = 0x02		/* Changes are private */;
pub const MAP_SHARED_VALIDATE: c_uint = 0x03	/* share + validate extension flags */;

