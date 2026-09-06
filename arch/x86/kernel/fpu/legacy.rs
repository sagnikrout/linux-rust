//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/fpu/legacy.h
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
    pub fn volatile((mxcsr): "ldmxcsr %0" :: "m") -> asm;
}
//
// Returns 0 on success or the trap number when the operation raises an
// exception.
//

extern "C" {
    pub fn user_insn(fwait: fnsave %[fx];, (*fx): *mut [fx] "=m", (*fx): *mut "m") -> return;
}
extern "C" {
    pub fn user_insn(%[fx]: fxsave, (*fx): *mut [fx] "=m", (*fx): *mut "m") -> return;
}
extern "C" {
    pub fn user_insn(%[fx]: fxsaveq, (*fx): *mut [fx] "=m", (*fx): *mut "m") -> return;
}
extern "C" {
    pub fn kernel_insn_err(%[fx]: fxrstor, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn kernel_insn_err(%[fx]: fxrstorq, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn user_insn(%[fx]: fxrstor, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn user_insn(%[fx]: fxrstorq, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn kernel_insn_err(%[fx]: frstor, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn user_insn(%[fx]: frstor, (*fx): *mut "=m", (*fx): *mut [fx] "m") -> return;
}
extern "C" {
    pub fn volatile((*fx): *mut "fxsave %[fx]" : [fx] "=m") -> asm;
}
extern "C" {
    pub fn volatile((*fx): *mut "fxsaveq %[fx]" : [fx] "=m") -> asm;
}
