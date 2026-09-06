//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/uaccess.h
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
// Based on arch/arm/include/asm/uaccess.h
//
// Copyright (C) 2012 ARM Ltd.
//

//
// User space memory access functions
//

extern "C" {
    pub fn __access_ok(ptr: *const void __user, size: c_ulong) -> c_int;
}
//
// Test whether a block of memory is a valid user space address.
// Returns 1 if the range is valid, 0 otherwise.
//
// This is equivalent to the following test:
// (u65)addr + (u65)size <= (u65)TASK_SIZE_MAX
//
// Asynchronous I/O running in a kernel thread does not have the
// TIF_TAGGED_ADDR flag of the process owning the mm, so always untag
// the user address before checking.
//
extern "C" {
    pub fn likely(_arg: __access_ok(addr, _arg: size)) -> return;
}

//
// User access enabling/disabling.
//

// reserved_pg_dir placed before swapper_pg_dir
// Set reserved ASID
//
// Disable interrupts to avoid preemption between reading the 'ttbr0'
// variable and the MSR. A context switch could trigger an ASID
// roll-over and an update of 'ttbr0'.
//
// Restore active ASID
// Restore user page table

//
// Sanitize a uaccess pointer such that it cannot reach any kernel address.
//
// Clearing bit 55 ensures the pointer cannot address any portion of the TTBR1
// address range (i.e. any kernel address), and either the pointer falls within
// the TTBR0 address range or must cause a fault.
//

//
// The "__xxx" versions of the user access functions do not verify the address
// space - it must have been done previously with a separate "access_ok()"
// call.
//
// The "__xxx_error" versions set the third argument to -EFAULT if an error
// occurs, and leave it unchanged on success.
//

//
// We must not call into the scheduler between uaccess_ttbr0_enable() and
// uaccess_ttbr0_disable(). As `x` and `ptr` could contain blocking functions,
// we must evaluate these outside of the critical section.
//

//
// We must not call into the scheduler between __mte_enable_tco_async() and
// __mte_disable_tco_async(). As `dst` and `src` may contain blocking
// functions, we must evaluate these outside of the critical section.
//

//
// We must not call into the scheduler between uaccess_ttbr0_enable() and
// uaccess_ttbr0_disable(). As `x` and `ptr` could contain blocking functions,
// we must evaluate these outside of the critical section.
//

//
// We must not call into the scheduler between __mte_enable_tco_async() and
// __mte_disable_tco_async(). As `dst` and `src` may contain blocking
// functions, we must evaluate these outside of the critical section.
//

extern "C" {
    pub fn __arch_copy_from_user(to: *mut c_void, from: *const void __user, n: c_ulong) -> unsigned long __must_check;
}

extern "C" {
    pub fn __arch_copy_to_user(to: *mut void __user, from: *const c_void, n: c_ulong) -> unsigned long __must_check;
}

//
// KCSAN uses these to save and restore ttbr state.
// We do not support KCSAN with ARM64_SW_TTBR0_PAN, so
// they are no-ops.
//
// We want the unsafe accessors to always be inlined and use
// the error labels - thus the macro games.
//

// Macro flag: #define INLINE_COPY_USER
extern "C" {
    pub fn __arch_clear_user(to: *mut void __user, n: c_ulong) -> unsigned long __must_check;
}

extern "C" {
    pub fn strncpy_from_user(dest: *mut c_char, src: *const char __user, count: c_long) -> c_long;
}
extern "C" {
    pub fn strnlen_user(str: *const char __user, n: c_long) -> __must_check long;
}

extern "C" {
    pub fn __copy_user_flushcache(to: *mut c_void, from: *const void __user, n: c_ulong) -> unsigned long __must_check;
}
extern "C" {
    pub fn __copy_user_flushcache(_arg: dst, _arg: __uaccess_mask_ptr(src), _arg: size) -> return;
}

//
// Return 0 on success, the number of bytes not probed otherwise.
//
extern "C" {
    pub fn mte_probe_user_range(_arg: uaddr, _arg: size) -> return;
}

