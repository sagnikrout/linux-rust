//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/vdso/vgetrandom.c
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

    typeof(__cvdso_getrandom) __kernel_getrandom;
#[no_mangle]
pub unsafe extern "C" fn __kernel_getrandom(buffer: *mut c_void, len: usize, flags: c_uint, opaque_state: *mut c_void, opaque_len: usize) -> isize {
    ssize_t __kernel_getrandom(void *buffer, size_t len, unsigned int flags, void *opaque_state, size_t opaque_len)
    {
    if (alternative_has_cap_likely(ARM64_HAS_FPSIMD))
    return __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len);
    if (unlikely(opaque_len == ~0UL && !buffer && !len && !flags))
    return -ENOSYS;
    return getrandom_syscall(buffer, len, flags);
    }
