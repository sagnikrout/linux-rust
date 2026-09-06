//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/copy_mc.c
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
// Copyright(c) 2016-2020 Intel Corporation. All rights reserved.

    static DEFINE_STATIC_KEY_FALSE(copy_mc_fragile_key);
#[no_mangle]
pub unsafe extern "C" fn enable_copy_mc_fragile() {
    void enable_copy_mc_fragile(void)
    {
    static_branch_inc(&copy_mc_fragile_key);
    }

//
// Similar to copy_user_handle_tail, probe for the write fault point, or
// source exception point.
//
    __visible notrace unsigned long
    copy_mc_fragile_handle_tail(char *to, char *from, unsigned len)
    {
    for (; len; --len, to++, from++)
    if (copy_mc_fragile(to, from, 1))
    break;
    return len;
    }

//
// No point in doing careful copying, or consulting a static key when
// there is no #MC handler in the CONFIG_X86_MCE=n case.
//
#[no_mangle]
pub unsafe extern "C" fn enable_copy_mc_fragile() {
    void enable_copy_mc_fragile(void)
    {
    }

    unsigned long copy_mc_enhanced_fast_string(void *dst, const void *src, unsigned len);
//
// copy_mc_to_kernel - memory copy that handles source exceptions
//
// @dst:	destination address
// @src:	source address
// @len:	number of bytes to copy
//
// Call into the 'fragile' version on systems that benefit from avoiding
// corner case poison consumption scenarios, For example, accessing
// poison across 2 cachelines with a single instruction. Almost all
// other uses case can use copy_mc_enhanced_fast_string() for a fast
// recoverable copy, or fallback to plain memcpy.
//
// Return 0 for success, or number of bytes not copied if there was an
// exception.
//
#[no_mangle]
pub unsafe extern "C" fn copy_mc_to_kernel(dst: *mut c_void, src: *const c_void, len: unsigned) -> unsigned long __must_check {
    unsigned long __must_check copy_mc_to_kernel(void *dst, const void *src, unsigned len)
    {
    unsigned long ret;
    if (copy_mc_fragile_enabled) {
    instrument_memcpy_before(dst, src, len);
    ret = copy_mc_fragile(dst, src, len);
    instrument_memcpy_after(dst, src, len, ret);
    return ret;
    }
    if (cpu_feature_enabled(X86_FEATURE_ERMS)) {
    instrument_memcpy_before(dst, src, len);
    ret = copy_mc_enhanced_fast_string(dst, src, len);
    instrument_memcpy_after(dst, src, len, ret);
    return ret;
    }
    memcpy(dst, src, len);
    return 0;
    }
    EXPORT_SYMBOL_GPL(copy_mc_to_kernel);
#[no_mangle]
pub unsafe extern "C" fn copy_mc_to_user(dst: *mut void __user, src: *const c_void, len: unsigned) -> unsigned long __must_check {
    unsigned long __must_check copy_mc_to_user(void __user *dst, const void *src, unsigned len)
    {
    unsigned long ret;
    if (copy_mc_fragile_enabled) {
    instrument_copy_to_user(dst, src, len);
    __uaccess_begin();
    ret = copy_mc_fragile(( void *)dst, src, len);
    __uaccess_end();
    return ret;
    }
    if (cpu_feature_enabled(X86_FEATURE_ERMS)) {
    instrument_copy_to_user(dst, src, len);
    __uaccess_begin();
    ret = copy_mc_enhanced_fast_string(( void *)dst, src, len);
    __uaccess_end();
    return ret;
    }
    return copy_user_generic(( void *)dst, src, len);
    }
