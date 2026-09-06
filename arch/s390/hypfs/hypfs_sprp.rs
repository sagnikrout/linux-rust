//! Automatically rewritten from C to Rust
//! Source: arch/s390/hypfs/hypfs_sprp.c
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
// Hypervisor filesystem for Linux on s390.
// Set Partition-Resource Parameter interface.
//
// Copyright IBM Corp. 2013
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
//

pub const DIAG304_SET_WEIGHTS: c_int = 0;
pub const DIAG304_QUERY_PRP: c_int = 1;
pub const DIAG304_SET_CAPPING: c_int = 2;
pub const DIAG304_CMD_MAX: c_int = 2;
#[no_mangle]
pub unsafe extern "C" fn __hypfs_sprp_diag304(data: *mut c_void, cmd: c_ulong) -> c_ulong {
    static inline unsigned long __hypfs_sprp_diag304(void *data, unsigned long cmd)
    {
    let mut r1: union register_pair = { .even = virt_to_phys(data), };
    asm volatile("diag %[r1],%[r3],0x304"
    : [r1] "+&d" (r1.pair)
    : [r3] "d" (cmd)
    : "memory");
    return r1.odd;
    }
#[no_mangle]
unsafe extern "C" fn hypfs_sprp_diag304(data: *mut c_void, cmd: c_ulong) -> c_ulong {
    static unsigned long hypfs_sprp_diag304(void *data, unsigned long cmd)
    {
    diag_stat_inc(DIAG_STAT_X304);
    return __hypfs_sprp_diag304(data, cmd);
    }
#[no_mangle]
unsafe extern "C" fn hypfs_sprp_free(data: *const c_void) {
    static void hypfs_sprp_free(const void *data)
    {
    free_page((unsigned long) data);
    }
#[no_mangle]
unsafe extern "C" fn hypfs_sprp_create(data_ptr: *mut c_void, free_ptr: *mut c_void, size: *mut usize) -> c_int {
    static int hypfs_sprp_create(void **data_ptr, void **free_ptr, size_t *size)
    {
    unsigned long rc;
    void *data;
    data = (void *) get_zeroed_page(GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    rc = hypfs_sprp_diag304(data, DIAG304_QUERY_PRP);
    if (rc != 1) {
// data_ptr = *free_ptr = NULL;
// size = 0;
    free_page((unsigned long) data);
    return -EIO;
    }
// data_ptr = *free_ptr = data;
// size = PAGE_SIZE;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __hypfs_sprp_ioctl(user_area: *mut void __user) -> c_int {
    static int __hypfs_sprp_ioctl(void __user *user_area)
    {
    struct hypfs_diag304 *diag304;
    unsigned long cmd;
    void __user *udata;
    void *data;
    int rc;
    rc = -ENOMEM;
    data = (void *)get_zeroed_page(GFP_KERNEL);
    diag304 = kzalloc_obj(*diag304);
    if (!data || !diag304)
    goto out;
    rc = -EFAULT;
    if (copy_from_user(diag304, user_area, sizeof(*diag304)))
    goto out;
    rc = -EINVAL;
    if ((diag304.args[0] >> 8) != 0 || diag304.args[1] > DIAG304_CMD_MAX)
    goto out;
    rc = -EFAULT;
    udata = (void __user *)(unsigned long) diag304.data;
    if (diag304.args[1] == DIAG304_SET_WEIGHTS ||
    diag304.args[1] == DIAG304_SET_CAPPING)
    if (copy_from_user(data, udata, PAGE_SIZE))
    goto out;
    cmd = *(unsigned long *) &diag304.args[0];
    diag304.rc = hypfs_sprp_diag304(data, cmd);
    if (diag304.args[1] == DIAG304_QUERY_PRP)
    if (copy_to_user(udata, data, PAGE_SIZE)) {
    rc = -EFAULT;
    goto out;
    }
    rc = copy_to_user(user_area, diag304, sizeof(*diag304)) ? -EFAULT : 0;
    out:
    kfree(diag304);
    free_page((unsigned long) data);
    return rc;
    }
    static long hypfs_sprp_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    void __user *argp;
    if (!capable(CAP_SYS_ADMIN))
    return -EACCES;
    argp = (void __user *)arg;
    switch (cmd) {
    case HYPFS_DIAG304:
    return __hypfs_sprp_ioctl(argp);
    default: /* unknown ioctl number */
    return -ENOTTY;
    }
    return 0;
    }
    static struct hypfs_dbfs_file hypfs_sprp_file = {
    .name		= "diag_304",
    .data_create	= hypfs_sprp_create,
    .data_free	= hypfs_sprp_free,
    .unlocked_ioctl = hypfs_sprp_ioctl,
    };
#[no_mangle]
pub unsafe extern "C" fn hypfs_sprp_init() {
    void hypfs_sprp_init(void)
    {
    if (!sclp.has_sprp)
    return;
    hypfs_dbfs_create_file(&hypfs_sprp_file);
    }
#[no_mangle]
pub unsafe extern "C" fn hypfs_sprp_exit() {
    void hypfs_sprp_exit(void)
    {
    if (!sclp.has_sprp)
    return;
    hypfs_dbfs_remove_file(&hypfs_sprp_file);
    }
