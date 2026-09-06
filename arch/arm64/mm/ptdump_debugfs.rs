//! Automatically rewritten from C to Rust
//! Source: arch/arm64/mm/ptdump_debugfs.c
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

#[no_mangle]
unsafe extern "C" fn ptdump_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int ptdump_show(struct seq_file *m, void *v)
    {
    struct ptdump_info *info = m.private;
    ptdump_walk(m, info);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(ptdump);
#[no_mangle]
pub unsafe extern "C" fn ptdump_debugfs_register(info: *mut ptdump_info, name: *const c_char) -> void __init {
    void __init ptdump_debugfs_register(struct ptdump_info *info, const char *name)
    {
    debugfs_create_file(name, 0400, core::ptr::null_mut(), info, &ptdump_fops);
    }
