//! Automatically rewritten from C to Rust
//! Source: fs/proc/version.c
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
unsafe extern "C" fn version_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int version_proc_show(struct seq_file *m, void *v)
    {
    seq_printf(m, linux_proc_banner,
    utsname().sysname,
    utsname().release,
    utsname().version);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_version_init() -> int __init {
    static int __init proc_version_init(void)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_single("version", 0, core::ptr::null_mut(), version_proc_show);
    pde_make_permanent(pde);
    return 0;
    }
    fs_initcall(proc_version_init);
