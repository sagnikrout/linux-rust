//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/compressed/cmdline.c
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

    static unsigned long fs;
#[no_mangle]
pub unsafe extern "C" fn set_fs(seg: c_ulong) {
    static inline void set_fs(unsigned long seg)
    {
    fs = seg << 4;  /* shift it back */
    }
    typedef unsigned long addr_t;
#[no_mangle]
pub unsafe extern "C" fn rdfs8(addr: addr_t) -> c_char {
    static inline char rdfs8(addr_t addr)
    {
    return *((char *)(fs + addr));
    }

#[no_mangle]
pub unsafe extern "C" fn get_cmd_line_ptr() -> c_ulong {
    unsigned long get_cmd_line_ptr(void)
    {
    let mut cmd_line_ptr: c_ulong = boot_params_ptr.hdr.cmd_line_ptr;
    cmd_line_ptr |= (u64)boot_params_ptr.ext_cmd_line_ptr << 32;
    return cmd_line_ptr;
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline_find_option(option: *const c_char, buffer: *mut c_char, bufsize: c_int) -> c_int {
    int cmdline_find_option(const char *option, char *buffer, int bufsize)
    {
    return __cmdline_find_option(get_cmd_line_ptr(), option, buffer, bufsize);
    }
#[no_mangle]
pub unsafe extern "C" fn cmdline_find_option_bool(option: *const c_char) -> c_int {
    int cmdline_find_option_bool(const char *option)
    {
    return __cmdline_find_option_bool(get_cmd_line_ptr(), option);
    }
