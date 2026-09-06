//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/suspend.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2021 Western Digital Corporation or its affiliates.
// Copyright (c) 2022 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct suspend_context {
// Saved and restored by low-level functions
    pub regs: pt_regs,
// Saved and restored by high-level functions
    pub envcfg: c_ulong,
    pub tvec: c_ulong,
    pub ie: c_ulong,

    pub satp: c_ulong,
    pub stimecmp: c_ulong,

    pub stimecmph: c_ulong,

}

//
// Used by hibernation core and cleared during resume sequence
//
// Low-level CPU suspend entry function
extern "C" {
    pub fn __cpu_suspend_enter(context: *mut suspend_context) -> c_int;
}
// High-level CPU suspend which will save context and call finish()
// Low-level CPU resume entry function
extern "C" {
    pub fn __cpu_resume_enter(hartid: c_ulong, context: c_ulong) -> c_int;
}
// Used to save and restore the CSRs
extern "C" {
    pub fn suspend_save_csrs(context: *mut suspend_context);
}
extern "C" {
    pub fn suspend_restore_csrs(context: *mut suspend_context);
}
// Low-level API to support hibernation
extern "C" {
    pub fn swsusp_arch_suspend() -> c_int;
}
extern "C" {
    pub fn swsusp_arch_resume() -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __hibernate_cpu_resume() -> c_int;
}
// Used to resume on the CPU we hibernated on
extern "C" {
    pub fn hibernate_resume_nonboot_cpu_disable() -> c_int;
}
extern "C" {
    pub fn hibernate_core_restore_code() -> asmlinkage int;
}
extern "C" {
    pub fn riscv_sbi_hsm_is_supported() -> bool;
}
extern "C" {
    pub fn riscv_sbi_suspend_state_is_valid(state: u32) -> bool;
}
extern "C" {
    pub fn riscv_sbi_hart_suspend(state: u32) -> c_int;
}
