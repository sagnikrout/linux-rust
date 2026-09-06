//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/linux/bitops.h
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

extern "C" {
    pub fn __sw_hweight8(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight16(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight32(w: c_uint) -> c_uint;
}
extern "C" {
    pub fn __sw_hweight64(w: __u64) -> c_ulong;
}
//
// Defined here because those may be needed by architecture-specific static
// inlines.
//

//
// Include this here because some architectures need generic_ffs/fls in
// scope
//
// XXX: this needs to be asm/bitops.h, when we get to per arch optimizations
//

// same as for_each_set_bit() but use bit as value to start with

extern "C" {
    pub fn sizeof(hweight64(w: w) == 4 ? hweight32(w) :) -> return;
}
extern "C" {
    pub fn fls(_arg: l) -> return;
}
extern "C" {
    pub fn fls64(_arg: l) -> return;
}
//
// rol32 - rotate a 32-bit value left
// @word: value to rotate
// @shift: bits to roll
//
// sign_extend64 - sign extend a 64-bit value using specified bit as sign-bit
// @value: value to sign extend
// @index: 0 based bit index (0<=index<64) to sign bit
//
