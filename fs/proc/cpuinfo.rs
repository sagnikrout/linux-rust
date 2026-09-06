//! Automatically rewritten from C to Rust
//! Source: fs/proc/cpuinfo.c
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

    extern const struct seq_operations cpuinfo_op;
#[no_mangle]
unsafe extern "C" fn cpuinfo_open(inode: *mut inode, file: *mut file) -> c_int {
    static int cpuinfo_open(struct inode *inode, struct file *file)
    {
    return seq_open(file, &cpuinfo_op);
    }
    static const struct proc_ops cpuinfo_proc_ops = {
    .proc_flags	= PROC_ENTRY_PERMANENT,
    .proc_open	= cpuinfo_open,
    .proc_read_iter	= seq_read_iter,
    .proc_lseek	= seq_lseek,
    .proc_release	= seq_release,
    };
#[no_mangle]
unsafe extern "C" fn proc_cpuinfo_init() -> int __init {
    static int __init proc_cpuinfo_init(void)
    {
    proc_create("cpuinfo", 0, core::ptr::null_mut(), &cpuinfo_proc_ops);
    return 0;
    }
    fs_initcall(proc_cpuinfo_init);
