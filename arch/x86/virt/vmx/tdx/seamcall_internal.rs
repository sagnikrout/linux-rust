//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/virt/vmx/tdx/seamcall_internal.h
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
// SEAMCALL utilities for TDX host-side operations.
//
// Provides convenient wrappers around SEAMCALL assembly with retry logic,
// error reporting and cache coherency tracking.
//
// Copyright (C) 2021-2023 Intel Corporation
//

extern "C" {
    pub fn __seamcall(fn: u64, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn __seamcall_ret(fn: u64, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn __seamcall_saved_ret(fn: u64, args: *mut tdx_module_args) -> u64;
}
extern "C" {
    pub fn u64(fn: *mut *mut sc_func_t)(u64, args: *mut tdx_module_args) -> typedef;
}
//
// SEAMCALLs are made to the TDX module and can generate dirty
// cachelines of TDX private memory.  Mark cache state incoherent
// so that the cache can be flushed during kexec.
//
// This needs to be done before actually making the SEAMCALL,
// because kexec-ing CPU could send NMI to stop remote CPUs,
// in which case even disabling IRQ won't help here.
//
extern "C" {
    pub fn func(_arg: fn, _arg: args) -> return;
}

extern "C" {
    pub fn void(fn: *mut *mut sc_err_func_t)(u64, err: u64, args: *mut tdx_module_args) -> typedef;
}

