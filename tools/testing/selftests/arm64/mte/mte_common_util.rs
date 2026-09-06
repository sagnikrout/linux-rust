//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/mte/mte_common_util.h
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
// Copyright (C) 2020 ARM Limited

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mte_mem_type {
    USE_MALLOC,
    USE_MMAP,
    USE_MPROTECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mte_mode {
    MTE_NONE_ERR,
    MTE_SYNC_ERR,
    MTE_ASYNC_ERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mte_fault_cxt {
// Address start which triggers mte tag fault
    pub trig_addr: c_ulong,
// Address range for mte tag fault and negative value means underflow
    pub trig_range: isize,
// siginfo si code
    pub trig_si_code: c_ulong,
// Flag to denote if correct fault caught
    pub fault_valid: bool,
}

// MTE utility functions
extern "C" {
    pub fn mte_default_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void);
}
extern "C" {
    pub fn mte_wait_after_trig();
}
extern "C" {
    pub fn mte_free_memory(ptr: *mut c_void, size: usize, mem_type: c_int, tags: bool);
}
extern "C" {
    pub fn mte_clear_tags(ptr: *mut c_void, size: usize);
}
extern "C" {
    pub fn mte_default_setup() -> c_int;
}
extern "C" {
    pub fn mte_restore_setup();
}
extern "C" {
    pub fn mte_switch_mode(mte_option: c_int, incl_mask: c_ulong, stonly: bool) -> c_int;
}
extern "C" {
    pub fn mte_initialize_current_context(mode: c_int, ptr: uintptr_t, range: isize);
}
// Common utility functions
extern "C" {
    pub fn create_temp_file() -> c_int;
}
// Assembly MTE utility functions
extern "C" {
    pub fn mte_set_tag_address_range(ptr: *mut c_void, range: c_int);
}
extern "C" {
    pub fn mte_clear_tag_address_range(ptr: *mut c_void, range: c_int);
}
extern "C" {
    pub fn mte_disable_pstate_tco();
}
extern "C" {
    pub fn mte_enable_pstate_tco();
}
extern "C" {
    pub fn mte_get_pstate_tco() -> c_uint;
}
// Test framework static inline functions/macros
