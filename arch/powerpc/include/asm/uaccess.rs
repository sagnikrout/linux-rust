//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/uaccess.h
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

// We use TASK_SIZE_USER64 as TASK_SIZE is not constant

// Threshold above which VMX copy path is used
pub const VMX_COPY_THRESHOLD: c_int = 3328;

//
// On powerpc64, TASK_SIZE_MAX is 0x0010000000000000 then even if both ptr and size
// are TASK_SIZE_MAX we are still inside the memory gap. So make it simple.
//
// These are the main single-value transfer routines.  They automatically
// use the right size if we just have the right pointer type.
//
// This gets kind of ugly. We want to return _two_ values in "get_user()"
// and yet we don't want to do any pointers, because that is too much
// of a performance impact. Thus we have a few rather ugly macros here,
// and hide all the ugliness from the user.
//
// The "__xxx" versions of the user access functions are versions that
// do not verify the address space, that must have been done previously
// with a separate "access_ok()" call (this is used when we do multiple
// accesses to the same area of user memory).
//
// As we use the same address space for kernel and user data on the
// PowerPC, we can just do these as direct assignments.  (Of course, the
// exception handling means that it's no longer "just"...)
//

//
// We don't tell gcc that we are accessing memory, but this is OK
// because we do not write to any memory gcc knows about, so there
// are no aliasing issues.
//
// -mprefixed can generate offsets beyond range, fall back hack

//
// This does an atomic 128 byte aligned load from userspace.
// Upto caller to do enable_kernel_vmx() before calling!
//

// -mprefixed can generate offsets beyond range, fall back hack

//
// This is a type: either unsigned long, if the argument fits into
// that type, or otherwise unsigned long long.
//

// more complex routines

extern "C" {
    pub fn raw_copy_tofrom_user(_arg: to, _arg: from, _arg: n, _arg: KUAP_READ_WRITE) -> return;
}

extern "C" {
    pub fn raw_copy_tofrom_user()to: *mut ( void __user, _arg: from, _arg: n, _arg: KUAP_READ) -> return;
}
extern "C" {
    pub fn raw_copy_tofrom_user(_arg: to, )from: *const ( void __user, _arg: n, _arg: KUAP_WRITE) -> return;
}
extern "C" {
    pub fn __arch_clear_user(addr: *mut void __user, size: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn strncpy_from_user(dst: *mut c_char, src: *const char __user, count: c_long) -> c_long;
}
extern "C" {
    pub fn strnlen_user(str: *const char __user, n: c_long) -> __must_check long;
}

extern "C" {
    pub fn copy_mc_generic(_arg: to, _arg: from, _arg: size) -> return;
}

extern "C" {
    pub fn copy_from_user_flushcache(dst: *mut c_void, src: *const void __user, size: usize) -> usize;
}

//
// Masking the user address is an alternative to a conditional
// user_access_begin that can avoid the fencing. This only works
// for dense accesses starting at the address.
//
// TASK_SIZE is a multiple of 128K for shifting by 17 to the right

extern "C" {
    pub fn mask_user_address_simple(_arg: ptr) -> return;
}
extern "C" {
    pub fn mask_user_address_isel(_arg: ptr) -> return;
}
extern "C" {
    pub fn mask_user_address_simple(_arg: ptr) -> return;
}
extern "C" {
    pub fn mask_user_address_32(_arg: ptr) -> return;
}
extern "C" {
    pub fn mask_user_address_fallback(_arg: ptr) -> return;
}

