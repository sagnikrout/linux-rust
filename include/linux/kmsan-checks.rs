//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kmsan-checks.h
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
// KMSAN checks to be used for one-off annotations in subsystems.
//
// Copyright (C) 2017-2022 Google LLC
// Author: Alexander Potapenko <glider@google.com>
//

//
// kmsan_poison_memory() - Mark the memory range as uninitialized.
// @address: address to start with.
// @size:    size of buffer to poison.
// @flags:   GFP flags for allocations done by this function.
//
// Until other data is written to this range, KMSAN will treat it as
// uninitialized. Error reports for this memory will reference the call site of
// kmsan_poison_memory() as origin.
//
extern "C" {
    pub fn kmsan_poison_memory(address: *const c_void, size: usize, flags: gfp_t);
}
//
// kmsan_unpoison_memory() -  Mark the memory range as initialized.
// @address: address to start with.
// @size:    size of buffer to unpoison.
//
// Until other data is written to this range, KMSAN will treat it as
// initialized.
//
extern "C" {
    pub fn kmsan_unpoison_memory(address: *const c_void, size: usize);
}
//
// kmsan_check_memory() - Check the memory range for being initialized.
// @address: address to start with.
// @size:    size of buffer to check.
//
// If any piece of the given range is marked as uninitialized, KMSAN will report
// an error.
//
extern "C" {
    pub fn kmsan_check_memory(address: *const c_void, size: usize);
}
//
// kmsan_copy_to_user() - Notify KMSAN about a data transfer to userspace.
// @to:      destination address in the userspace.
// @from:    source address in the kernel.
// @to_copy: number of bytes to copy.
// @left:    number of bytes not copied.
//
// If this is a real userspace data transfer, KMSAN checks the bytes that were
// actually copied to ensure there was no information leak. If @to belongs to
// the kernel space (which is possible for compat syscalls), KMSAN just copies
// the metadata.
//
// kmsan_memmove() - Notify KMSAN about a data copy within kernel.
// @to:   destination address in the kernel.
// @from: source address in the kernel.
// @size: number of bytes to copy.
//
// Invoked after non-instrumented version (e.g. implemented using assembly
// code) of memmove()/memcpy() is called, in order to copy KMSAN's metadata.
//
extern "C" {
    pub fn kmsan_memmove(to: *mut c_void, from: *const c_void, to_copy: usize);
}

