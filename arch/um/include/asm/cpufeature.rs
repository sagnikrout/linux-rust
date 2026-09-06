//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/cpufeature.h
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
// There are 32 bits/features in each mask word.  The high bits
// (selected with (bit>>5) give us the word number and the low 5
// bits give us the bit/feature number inside the word.
// (1UL<<((bit)&31) gives us a mask for the feature_bit so we can
// see if it is set in the mask word.
//

//
// This macro is for detection of features which need kernel
// infrastructure to be used.  It may *not* directly test the CPU
// itself.  Use the cpu_has() family if you want true runtime
// testing of CPU features, like in hypervisor code where you are
// supporting a possible guest feature where host support for it
// is not relevant.
//

extern "C" {
    pub fn setup_clear_cpu_cap(bit: c_uint);
}

