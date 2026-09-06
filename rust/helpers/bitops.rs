//! Automatically rewritten from C to Rust
//! Source: rust/helpers/bitops.c
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

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper___set_bit(nr: c_ulong, addr: *mut c_ulong) {
    void rust_helper___set_bit(unsigned long nr, unsigned long *addr)
    {
    __set_bit(nr, addr);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper___clear_bit(nr: c_ulong, addr: *mut c_ulong) {
    void rust_helper___clear_bit(unsigned long nr, unsigned long *addr)
    {
    __clear_bit(nr, addr);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_set_bit(nr: c_ulong, addr: *mut volatile unsigned long) {
    void rust_helper_set_bit(unsigned long nr, volatile unsigned long *addr)
    {
    set_bit(nr, addr);
    }
    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn rust_helper_clear_bit(nr: c_ulong, addr: *mut volatile unsigned long) {
    void rust_helper_clear_bit(unsigned long nr, volatile unsigned long *addr)
    {
    clear_bit(nr, addr);
    }
//
// The rust_helper_ prefix is intentionally omitted below so that the
// declarations in include/linux/find.h are compatible with these helpers.
//
// Note that the below #ifdefs mean that the helper is only created if C does
// not provide a definition.
//

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn _find_first_zero_bit(p: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_first_zero_bit(const unsigned long *p, unsigned long size)
    {
    return find_first_zero_bit(p, size);
    }

    __rust_helper
    unsigned long _find_next_zero_bit(const unsigned long *addr,
    unsigned long size, unsigned long offset)
    {
    return find_next_zero_bit(addr, size, offset);
    }

    __rust_helper
#[no_mangle]
pub unsafe extern "C" fn _find_first_bit(addr: *const c_ulong, size: c_ulong) -> c_ulong {
    unsigned long _find_first_bit(const unsigned long *addr, unsigned long size)
    {
    return find_first_bit(addr, size);
    }

    __rust_helper
    unsigned long _find_next_bit(const unsigned long *addr, unsigned long size,
    unsigned long offset)
    {
    return find_next_bit(addr, size, offset);
    }
