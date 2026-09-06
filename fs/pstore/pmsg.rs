//! Automatically rewritten from C to Rust
//! Source: fs/pstore/pmsg.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2014  Google, Inc.
//

    static DEFINE_MUTEX(pmsg_lock);
    static ssize_t write_pmsg(struct file *file, const char __user *buf,
    size_t count, loff_t *ppos)
    {
    struct pstore_record record;
    int ret;
    if (!count)
    return 0;
    pstore_record_init(&record, psinfo);
    record.type = PSTORE_TYPE_PMSG;
    record.size = count;
// check outside lock, page in any data. write_user also checks
    if (!access_ok(buf, count))
    return -EFAULT;
    mutex_lock(&pmsg_lock);
    ret = psinfo.write_user(&record, buf);
    mutex_unlock(&pmsg_lock);
    return ret ? ret : count;
    }
    static const struct file_operations pmsg_fops = {
    .owner		= THIS_MODULE,
    .llseek		= noop_llseek,
    .write		= write_pmsg,
    };
    static struct class *pmsg_class;
    static int pmsg_major;

    static char *pmsg_devnode(const struct device *dev, umode_t *mode)
    {
    if (mode)
// mode = 0220;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pstore_register_pmsg() {
    void pstore_register_pmsg(void)
    {
    struct device *pmsg_device;
    pmsg_major = register_chrdev(0, PMSG_NAME, &pmsg_fops);
    if (pmsg_major < 0) {
    pr_err("register_chrdev failed\n");
    goto err;
    }
    pmsg_class = class_create(PMSG_NAME);
    if (IS_ERR(pmsg_class)) {
    pr_err("device class file already in use\n");
    goto err_class;
    }
    pmsg_class.devnode = pmsg_devnode;
    pmsg_device = device_create(pmsg_class, core::ptr::null_mut(), MKDEV(pmsg_major, 0),
    core::ptr::null_mut(), "%s%d", PMSG_NAME, 0);
    if (IS_ERR(pmsg_device)) {
    pr_err("failed to create device\n");
    goto err_device;
    }
    return;
    err_device:
    class_destroy(pmsg_class);
    err_class:
    unregister_chrdev(pmsg_major, PMSG_NAME);
    err:
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn pstore_unregister_pmsg() {
    void pstore_unregister_pmsg(void)
    {
    device_destroy(pmsg_class, MKDEV(pmsg_major, 0));
    class_destroy(pmsg_class);
    unregister_chrdev(pmsg_major, PMSG_NAME);
    }
