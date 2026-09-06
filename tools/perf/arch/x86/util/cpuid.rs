//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/arch/x86/util/cpuid.h
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
pub const PERF_CPUID_H: c_int = 1;
//
// Preserve %ebx/%rbx register by either placing it in %rdi or saving it
// on the stack - x86-64 needs to avoid the stack red zone. In PIC
// compilations %ebx contains the address of the global offset
// table. %rbx is occasionally used to address stack variables in
// presence of dynamic allocas.
//

extern "C" {
    pub fn get_cpuid_0(vendor: *mut c_char, lvl: *mut c_uint);
}
