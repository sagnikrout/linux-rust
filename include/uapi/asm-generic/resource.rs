//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/resource.h
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
// Resource limit IDs
//
// ( Compatibility detail: there are architectures that have
// a different rlimit ID order in the 5-9 range and want
// to keep that order for binary compatibility. The reasons
// are historic and all new rlimits are identical across all
// arches. If an arch has such special order for some rlimits
// then it defines them prior including asm-generic/resource.h. )
//

pub const RLIM_NLIMITS: c_int = 16;
//
// SuS says limits have to be unsigned.
// Which makes a ton more sense anyway.
//
// Some architectures override this (for compatibility reasons):
//

