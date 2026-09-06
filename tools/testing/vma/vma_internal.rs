//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vma/vma_internal.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// vma_internal.h
//
// Header providing userland wrappers and shims for the functionality provided
// by mm/vma_internal.h.
//
// We make the header guard the same as mm/vma_internal.h, so if this shim
// header is included, it precludes the inclusion of the kernel one.
//

pub const CONFIG_MMU: c_int = 1;
pub const CONFIG_PER_VMA_LOCK: c_int = 1;

//
// DUPLICATE typedef definitions from kernel source that have to be declared
// ahead of all other headers.
//
// Macro flag: #define __private
// NUM_MM_FLAG_BITS defined by test code.
extern "C" {
    pub fn DECLARE_BITMAP(_arg: __mm_flags, _arg: NUM_MM_FLAG_BITS) -> __private;
}
// NUM_VMA_FLAG_BITS defined by test code.
pub type vm_flags_t = c_ulong;

pub type pgprotval_t = c_ulong;
pub type vm_fault_t =  unsigned int;

