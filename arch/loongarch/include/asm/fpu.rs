//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/fpu.h
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

extern "C" {
    pub fn kernel_fpu_begin();
}
extern "C" {
    pub fn kernel_fpu_end();
}
extern "C" {
    pub fn _init_fpu(int: unsigned) -> asmlinkage void;
}
extern "C" {
    pub fn _save_fp(: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _restore_fp(: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _save_fp_context(fpregs: *mut void __user, fcc: *mut void __user, csr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn _restore_fp_context(fpregs: *mut void __user, fcc: *mut void __user, csr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn _save_lsx(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _restore_lsx(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _init_lsx_upper() -> asmlinkage void;
}
extern "C" {
    pub fn _restore_lsx_upper(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _save_lsx_context(fpregs: *mut void __user, fcc: *mut void __user, fcsr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn _restore_lsx_context(fpregs: *mut void __user, fcc: *mut void __user, fcsr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn _save_lasx(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _restore_lasx(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _init_lasx_upper() -> asmlinkage void;
}
extern "C" {
    pub fn _restore_lasx_upper(fpu: *mut loongarch_fpu) -> asmlinkage void;
}
extern "C" {
    pub fn _save_lasx_context(fpregs: *mut void __user, fcc: *mut void __user, fcsr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn _restore_lasx_context(fpregs: *mut void __user, fcc: *mut void __user, fcsr: *mut void __user) -> asmlinkage int;
}
extern "C" {
    pub fn enable_lsx();
}
extern "C" {
    pub fn disable_lsx();
}
extern "C" {
    pub fn save_lsx(t: *mut task_struct);
}
extern "C" {
    pub fn restore_lsx(t: *mut task_struct);
}
extern "C" {
    pub fn enable_lasx();
}
extern "C" {
    pub fn disable_lasx();
}
extern "C" {
    pub fn save_lasx(t: *mut task_struct);
}
extern "C" {
    pub fn restore_lasx(t: *mut task_struct);
}
//
// Mask the FCSR Cause bits according to the Enable bits, observing
// that Unimplemented is always enabled.
//
extern "C" {
    pub fn is_lsx_enabled(is_lasx_enabled(: ) |) -> return;
}

extern "C" {
    pub fn test_thread_flag(_arg: TIF_USEDFPU) -> return;
}

extern "C" {
    pub fn test_thread_flag(_arg: TIF_USEDSIMD) -> return;
}

extern "C" {
    pub fn test_thread_flag(_arg: TIF_LSX_CTX_LIVE) -> return;
}
extern "C" {
    pub fn test_thread_flag(_arg: TIF_LASX_CTX_LIVE) -> return;
}
