//! Automatically rewritten from C to Rust
//! Source: lib/usercopy.c
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

// out-of-line parts

#[no_mangle]
pub unsafe extern "C" fn _copy_from_user(to: *mut c_void, from: *const void __user, n: c_ulong) -> c_ulong {
    unsigned long _copy_from_user(void *to, const void __user *from, unsigned long n)
    {
    return _inline_copy_from_user(to, from, n);
    }
    EXPORT_SYMBOL(_copy_from_user);
#[no_mangle]
pub unsafe extern "C" fn _copy_to_user(to: *mut void __user, from: *const c_void, n: c_ulong) -> c_ulong {
    unsigned long _copy_to_user(void __user *to, const void *from, unsigned long n)
    {
    return _inline_copy_to_user(to, from, n);
    }
    EXPORT_SYMBOL(_copy_to_user);

//
// check_zeroed_user: check if a userspace buffer only contains zero bytes
// @from: Source address, in userspace.
// @size: Size of buffer.
//
// This is effectively shorthand for "memchr_inv(from, 0, size) == NULL" for
// userspace addresses (and is more efficient because we don't care where the
// first non-zero byte is).
//
// Returns:
// * 0: There were non-zero bytes present in the buffer.
// * 1: The buffer was full of zero bytes.
// * -EFAULT: access to userspace failed.
//
#[no_mangle]
pub unsafe extern "C" fn check_zeroed_user(from: *const void __user, size: usize) -> c_int {
    int check_zeroed_user(const void __user *from, size_t size)
    {
    unsigned long val;
    let mut align: uintptr_t = (uintptr_t) from % sizeof(unsigned long);
    if (unlikely(size == 0))
    return 1;
    from -= align;
    size += align;
    if (!user_read_access_begin(from, size))
    return -EFAULT;
    unsafe_get_user(val, (unsigned long __user *) from, err_fault);
    if (align)
    val &= ~aligned_byte_mask(align);
    while (size > sizeof(unsigned long)) {
    if (unlikely(val))
    goto done;
    from += sizeof(unsigned long);
    size -= sizeof(unsigned long);
    unsafe_get_user(val, (unsigned long __user *) from, err_fault);
    }
    if (size < sizeof(unsigned long))
    val &= aligned_byte_mask(size);
    done:
    user_read_access_end();
    return (val == 0);
    err_fault:
    user_read_access_end();
    return -EFAULT;
    }
    EXPORT_SYMBOL(check_zeroed_user);
