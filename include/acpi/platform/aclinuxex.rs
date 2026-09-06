//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/platform/aclinuxex.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: aclinuxex.h - Extra OS specific defines, etc. for Linux
//
// Copyright (C) 2000 - 2026, Intel Corp.
//

//
// Overrides for in-kernel ACPICA
//
extern "C" {
    pub fn acpi_os_initialize() -> acpi_status ACPI_INIT_FUNCTION;
}
extern "C" {
    pub fn acpi_os_terminate() -> acpi_status;
}
//
// The irqs_disabled() check is for resume from RAM.
// Interrupts are off during resume, just like they are for boot.
// However, boot has  (system_state != SYSTEM_RUNNING)
// to quiet __might_sleep() in kmalloc() and resume does not.
//
// These specialized allocators have to be macros for their allocations to be
// accounted separately (to have separate alloc_tag).
//

//
// When lockdep is enabled, the spin_lock_init() macro stringifies it's
// argument and uses that as a name for the lock in debugging.
// By executing spin_lock_init() in a macro the key changes from "lock" for
// all locks to the name of the argument of acpi_os_create_lock(), which
// prevents lockdep from reporting false positives for ACPICA locks.
//

// (__handle) = lock; \

// (__handle) = lock; \
//
// OSL interfaces added by Linux
//

