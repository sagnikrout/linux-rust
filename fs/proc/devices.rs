//! Automatically rewritten from C to Rust
//! Source: fs/proc/devices.c
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
unsafe extern "C" fn devinfo_show(f: *mut seq_file, v: *mut c_void) -> c_int {
    static int devinfo_show(struct seq_file *f, void *v)
    {
    let mut i: c_int = *(loff_t *) v;
    if (i < CHRDEV_MAJOR_MAX) {
    if (i == 0)
    seq_puts(f, "Character devices:\n");
    chrdev_show(f, i);
    }

    else {
    i -= CHRDEV_MAJOR_MAX;
    if (i == 0)
    seq_puts(f, "\nBlock devices:\n");
    blkdev_show(f, i);
    }

    return 0;
    }
    static void *devinfo_start(struct seq_file *f, loff_t *pos)
    {
    if (*pos < (BLKDEV_MAJOR_MAX + CHRDEV_MAJOR_MAX))
    return pos;
    return core::ptr::null_mut();
    }
    static void *devinfo_next(struct seq_file *f, void *v, loff_t *pos)
    {
    (*pos)++;
    if (*pos >= (BLKDEV_MAJOR_MAX + CHRDEV_MAJOR_MAX))
    return core::ptr::null_mut();
    return pos;
    }
#[no_mangle]
unsafe extern "C" fn devinfo_stop(f: *mut seq_file, v: *mut c_void) {
    static void devinfo_stop(struct seq_file *f, void *v)
    {
// Nothing to do
    }
    static const struct seq_operations devinfo_ops = {
    .start = devinfo_start,
    .next  = devinfo_next,
    .stop  = devinfo_stop,
    .show  = devinfo_show
    };
#[no_mangle]
unsafe extern "C" fn proc_devices_init() -> int __init {
    static int __init proc_devices_init(void)
    {
    struct proc_dir_entry *pde;
    pde = proc_create_seq("devices", 0, core::ptr::null_mut(), &devinfo_ops);
    pde_make_permanent(pde);
    return 0;
    }
    fs_initcall(proc_devices_init);
