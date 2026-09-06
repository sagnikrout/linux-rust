//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/switch_to.h
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
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//

extern "C" {
    pub fn switch_booke_debug_regs(new_debug: *mut debug_reg);
}
extern "C" {
    pub fn emulate_altivec(: *mut pt_regs) -> c_int;
}

extern "C" {
    pub fn restore_math(regs: *mut pt_regs);
}

extern "C" {
    pub fn restore_tm_state(regs: *mut pt_regs);
}
extern "C" {
    pub fn flush_all_to_thread(: *mut task_struct);
}
extern "C" {
    pub fn giveup_all(: *mut task_struct);
}

extern "C" {
    pub fn enable_kernel_fp();
}
extern "C" {
    pub fn flush_fp_to_thread(: *mut task_struct);
}
extern "C" {
    pub fn giveup_fpu(: *mut task_struct);
}
extern "C" {
    pub fn save_fpu(: *mut task_struct);
}

extern "C" {
    pub fn enable_kernel_altivec();
}
extern "C" {
    pub fn flush_altivec_to_thread(: *mut task_struct);
}
extern "C" {
    pub fn giveup_altivec(: *mut task_struct);
}
extern "C" {
    pub fn save_altivec(: *mut task_struct);
}

extern "C" {
    pub fn enable_kernel_vsx();
}
extern "C" {
    pub fn flush_vsx_to_thread(: *mut task_struct);
}

extern "C" {
    pub fn enable_kernel_spe();
}
extern "C" {
    pub fn flush_spe_to_thread(: *mut task_struct);
}
extern "C" {
    pub fn giveup_spe(: *mut task_struct);
}
extern "C" {
    pub fn __giveup_spe(: *mut task_struct);
}

// EBB perf events are not inherited, so clear all EBB state.

extern "C" {
    pub fn kvmppc_save_user_regs();
}
extern "C" {
    pub fn kvmppc_save_current_sprs();
}
extern "C" {
    pub fn set_thread_tidr(t: *mut task_struct) -> c_int;
}
