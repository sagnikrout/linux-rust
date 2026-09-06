//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kernel/signal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2007 Benjamin Herrenschmidt, IBM Corporation
// Extracted from signal_32.c and signal_64.c
//
extern "C" {
    pub fn __get_user(_arg: dst->sig[0], )&src->sig[0]: *mut (u64 __user) -> return;
}

extern "C" {
    pub fn copy_fpr_to_user(to: *mut void __user, task: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn copy_ckfpr_to_user(to: *mut void __user, task: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn copy_fpr_from_user(task: *mut task_struct, from: *mut void __user) -> c_ulong;
}
extern "C" {
    pub fn copy_ckfpr_from_user(task: *mut task_struct, from: *mut void __user) -> c_ulong;
}

extern "C" {
    pub fn sizeof(_arg: double)) -> *mut ELF_NFPREG;
}
extern "C" {
    pub fn sizeof(_arg: double)) -> *mut ELF_NFPREG;
}

extern "C" {
    pub fn sizeof(_arg: double)) -> *mut ELF_NFPREG;
}
extern "C" {
    pub fn sizeof(_arg: double)) -> *mut ELF_NFPREG;
}

