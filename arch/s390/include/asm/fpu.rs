//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/fpu.h
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
// In-kernel FPU support functions
//
// Consider these guidelines before using in-kernel FPU functions:
//
// 1. Use kernel_fpu_begin() and kernel_fpu_end() to enclose all in-kernel
// use of floating-point or vector registers and instructions.
//
// 2. For kernel_fpu_begin(), specify the vector register range you want to
// use with the KERNEL_VXR_* constants. Consider these usage guidelines:
//
// a) If your function typically runs in process-context, use the lower
// half of the vector registers, for example, specify KERNEL_VXR_LOW.
// b) If your function typically runs in soft-irq or hard-irq context,
// prefer using the upper half of the vector registers, for example,
// specify KERNEL_VXR_HIGH.
//
// If you adhere to these guidelines, an interrupted process context
// does not require to save and restore vector registers because of
// disjoint register ranges.
//
// Also note that the __kernel_fpu_begin()/__kernel_fpu_end() functions
// includes logic to save and restore up to 16 vector registers at once.
//
// 3. You can nest kernel_fpu_begin()/kernel_fpu_end() by using different
// struct kernel_fpu states.  Vector registers that are in use by outer
// levels are saved and restored.  You can minimize the save and restore
// effort by choosing disjoint vector register ranges.
//
// 5. To use vector floating-point instructions, specify the KERNEL_FPC
// flag to save and restore floating-point controls in addition to any
// vector register range.
//
// 6. To use floating-point registers and instructions only, specify the
// KERNEL_FPR flag.  This flag triggers a save and restore of vector
// registers V0 to V15 and floating-point controls.
//
// Copyright IBM Corp. 2015
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

extern "C" {
    pub fn load_fpu_state(state: *mut fpu, flags: c_int);
}
extern "C" {
    pub fn save_fpu_state(state: *mut fpu, flags: c_int);
}
extern "C" {
    pub fn __kernel_fpu_begin(state: *mut kernel_fpu, flags: c_int);
}
extern "C" {
    pub fn __kernel_fpu_end(state: *mut kernel_fpu, flags: c_int);
}
extern "C" {
    pub fn __kernel_fpu_invalid_size();
}

