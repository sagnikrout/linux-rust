//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpufeature.h
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
// In order to save room, we index into this array by doing
// X86_BUG_<name> - NCAPINTS*32.
//

//
// This is the default CPU features testing macro to use in code.
//
// It is for detection of features which need kernel infrastructure to be
// used.  It may *not* directly test the CPU itself.  Use the cpu_has() family
// if you want true runtime testing of CPU features, like in hypervisor code
// where you are supporting a possible guest feature where host support for it
// is not relevant.
//

extern "C" {
    pub fn setup_clear_cpu_cap(bit: c_uint);
}
extern "C" {
    pub fn clear_cpu_cap(c: *mut cpuinfo_x86, bit: c_uint);
}
extern "C" {
    pub fn check_cpufeature_deps(c: *mut cpuinfo_x86);
}

//
// Do not use an "m" constraint for [cap_byte] here: gcc doesn't know
// that this is only used on a fallback path and will sometimes cause
// it to manifest the address of boot_cpu_data in a register, fouling
// the mainline (post-initialization) code.
//

