//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/vdso/compat_gettimeofday.h
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
// Copyright (C) 2018 ARM Limited
//

pub const VDSO_HAS_CLOCK_GETRES: c_int = 1;
pub const BUILD_VDSO32: c_int = 1;
extern "C" {
    pub fn asm(_arg: "r0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "r0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "r0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "r0") -> register long ret;
}
extern "C" {
    pub fn asm(_arg: "r0") -> register long ret;
}
//
// Core checks for mode already, so this raced against a concurrent
// update. Return something. Core will do another round and then
// see the mode change and fallback to the syscall.
//
// This isb() is required to prevent that the counter value
// is speculated.
//
extern "C" {
    pub fn volatile(p15: "mrrc, _arg: 1, _arg: %Q0, _arg: %R0, (res): c14" : "=r") -> asm;
}
//
// This isb() is required to prevent that the seq lock is
// speculated.
//
// This simply puts &_vdso_time_data into ret. The reason why we don't use
// `ret = _vdso_time_data` is that the compiler tends to optimise this in a
// very suboptimal way: instead of keeping &_vdso_time_data in a register,
// it goes through a relocation almost every time _vdso_time_data must be
// accessed (even in subfunctions). This is both time and space
// consuming: each relocation uses a word in the code section, and it
// has to be loaded at runtime.
//
// This trick hides the assignment from the compiler. Since it cannot
// track where the pointer comes from, it will only use one relocation
// where __aarch64_get_vdso_u_time_data() is called, and then keep the
// result in a register.
//
extern "C" {
    pub fn volatile(%0: "mov, "r"(&vdso_u_time_data): %1" : "=r"(ret) :) -> asm;
}

