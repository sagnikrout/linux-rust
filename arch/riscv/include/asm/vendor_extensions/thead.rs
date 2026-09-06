//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/vendor_extensions/thead.h
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
// Extension keys must be strictly less than RISCV_ISA_VENDOR_EXT_MAX.
//
pub const RISCV_ISA_VENDOR_EXT_XTHEADVECTOR: c_int = 0;

extern "C" {
    pub fn disable_xtheadvector();
}

// Extension specific helpers
//
// Vector 0.7.1 as used for example on T-Head Xuantie cores, uses an older
// encoding for vsetvli (ta, ma vs. d1), so provide an instruction for
// vsetvli	t4, x0, e8, m8, d1
//

//
// While in theory, the vector-0.7.1 vsb.v and vlb.v result in the same
// encoding as the standard vse8.v and vle8.v, compilers seem to optimize
// the call resulting in a different encoding and then using a value for
// the "mop" field that is not part of vector-0.7.1
// So encode specific variants for vstate_save and _restore.
//

