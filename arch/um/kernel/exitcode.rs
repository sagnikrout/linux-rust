//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/exitcode.c
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
//
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

//
// If read and write race, the read will still atomically read a valid
// value.
//
    let mut uml_exitcode: c_int = 0;
#[no_mangle]
unsafe extern "C" fn exitcode_proc_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int exitcode_proc_show(struct seq_file *m, void *v)
    {
    int val;
//
// Save uml_exitcode in a local so that we don't need to guarantee
// that sprintf accesses it atomically.
//
    val = uml_exitcode;
    seq_printf(m, "%d\n", val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exitcode_proc_open(inode: *mut inode, file: *mut file) -> c_int {
    static int exitcode_proc_open(struct inode *inode, struct file *file)
    {
    return single_open(file, exitcode_proc_show, core::ptr::null_mut());
    }
    static ssize_t exitcode_proc_write(struct file *file,
    const char __user *buffer, size_t count, loff_t *pos)
    {
    char *end, buf[sizeof("nnnnn\0")];
    size_t size;
    int tmp;
    size = min(count, sizeof(buf));
    if (copy_from_user(buf, buffer, size))
    return -EFAULT;
    tmp = simple_strtol(buf, &end, 0);
    if ((*end != '\0') && !isspace(*end))
    return -EINVAL;
    uml_exitcode = tmp;
    return count;
    }
    static const struct proc_ops exitcode_proc_ops = {
    .proc_open	= exitcode_proc_open,
    .proc_read	= seq_read,
    .proc_lseek	= seq_lseek,
    .proc_release	= single_release,
    .proc_write	= exitcode_proc_write,
    };
#[no_mangle]
unsafe extern "C" fn make_proc_exitcode() -> c_int {
    static int make_proc_exitcode(void)
    {
    struct proc_dir_entry *ent;
    ent = proc_create("exitcode", 0600, core::ptr::null_mut(), &exitcode_proc_ops);
    if (ent == core::ptr::null_mut()) {
    printk(KERN_WARNING "make_proc_exitcode : Failed to register "
    "/proc/exitcode\n");
    return 0;
    }
    return 0;
    }
    __initcall(make_proc_exitcode);
