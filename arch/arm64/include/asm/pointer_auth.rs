//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/pointer_auth.h
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
// The EL0/EL1 pointer bits used by a pointer authentication code.
// This is dependent on TBI0/TBI1 being enabled, or bits 63:56 would also apply.
//

//
// Each key is a 128-bit quantity which is split across a pair of 64-bit
// registers (Lo and Hi).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrauth_key {
    pub hi: unsigned long lo,,
}

//
// We give each process its own keys, which are shared by all threads. The keys
// are inherited upon fork(), and reinitialised upon exec*().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrauth_keys_user {
    pub apia: ptrauth_key,
    pub apib: ptrauth_key,
    pub apda: ptrauth_key,
    pub apdb: ptrauth_key,
    pub apga: ptrauth_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptrauth_keys_kernel {
    pub apia: ptrauth_key,
}

extern "C" {
    pub fn ptrauth_prctl_reset_keys(tsk: *mut task_struct, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn ptrauth_get_enabled_keys(tsk: *mut task_struct) -> c_int;
}

// enable all keys */                                          \

// Macro flag: #define ptrauth_enable()

// Macro flag: #define ptrauth_suspend_exit()
// Macro flag: #define ptrauth_thread_init_user()
// Macro flag: #define ptrauth_thread_switch_user(tsk)

// Macro flag: #define ptrauth_thread_init_kernel(tsk)
// Macro flag: #define ptrauth_thread_switch_kernel(tsk)

