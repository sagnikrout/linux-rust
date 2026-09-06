//! Automatically rewritten from C to Rust
//! Source: arch/s390/hypfs/hypfs_dbfs.c
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
// Hypervisor filesystem for Linux on s390 - debugfs interface
//
// Copyright IBM Corp. 2010
// Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

    static struct dentry *dbfs_dir;
    static struct hypfs_dbfs_data *hypfs_dbfs_data_alloc(struct hypfs_dbfs_file *f)
    {
    struct hypfs_dbfs_data *data;
    data = kmalloc_obj(*data);
    if (!data)
    return core::ptr::null_mut();
    data.dbfs_file = f;
    return data;
    }
#[no_mangle]
unsafe extern "C" fn hypfs_dbfs_data_free(data: *mut hypfs_dbfs_data) {
    static void hypfs_dbfs_data_free(struct hypfs_dbfs_data *data)
    {
    data.dbfs_file.data_free(data.buf_free_ptr);
    kfree(data);
    }
    static ssize_t dbfs_read(struct file *file, char __user *buf,
    size_t size, loff_t *ppos)
    {
    struct hypfs_dbfs_data *data;
    struct hypfs_dbfs_file *df;
    ssize_t rc;
    if (*ppos != 0)
    return 0;
    df = file_inode(file).i_private;
    if (mutex_lock_interruptible(&df.lock))
    return -ERESTARTSYS;
    data = hypfs_dbfs_data_alloc(df);
    if (!data) {
    mutex_unlock(&df.lock);
    return -ENOMEM;
    }
    rc = df.data_create(&data.buf, &data.buf_free_ptr, &data.size);
    if (rc) {
    mutex_unlock(&df.lock);
    kfree(data);
    return rc;
    }
    mutex_unlock(&df.lock);
    rc = simple_read_from_buffer(buf, size, ppos, data.buf, data.size);
    hypfs_dbfs_data_free(data);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn dbfs_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    static long dbfs_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct hypfs_dbfs_file *df = file_inode(file).i_private;
    long rc;
    mutex_lock(&df.lock);
    rc = df.unlocked_ioctl(file, cmd, arg);
    mutex_unlock(&df.lock);
    return rc;
    }
    static const struct file_operations dbfs_ops_ioctl = {
    .read		= dbfs_read,
    .unlocked_ioctl = dbfs_ioctl,
    };
    static const struct file_operations dbfs_ops = {
    .read		= dbfs_read,
    };
#[no_mangle]
pub unsafe extern "C" fn hypfs_dbfs_create_file(df: *mut hypfs_dbfs_file) {
    void hypfs_dbfs_create_file(struct hypfs_dbfs_file *df)
    {
    const struct file_operations *fops = &dbfs_ops;
    if (df.unlocked_ioctl && !security_locked_down(LOCKDOWN_DEBUGFS))
    fops = &dbfs_ops_ioctl;
    df.dentry = debugfs_create_file(df.name, 0400, dbfs_dir, df, fops);
    mutex_init(&df.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn hypfs_dbfs_remove_file(df: *mut hypfs_dbfs_file) {
    void hypfs_dbfs_remove_file(struct hypfs_dbfs_file *df)
    {
    debugfs_remove(df.dentry);
    }
#[no_mangle]
unsafe extern "C" fn hypfs_dbfs_init() -> int __init {
    static int __init hypfs_dbfs_init(void)
    {
    let mut rc: c_int = -ENODATA;
    dbfs_dir = debugfs_create_dir("s390_hypfs", core::ptr::null_mut());
    if (hypfs_diag_init())
    goto fail_dbfs_exit;
    if (hypfs_vm_init())
    goto fail_hypfs_diag_exit;
    hypfs_sprp_init();
    if (hypfs_diag0c_init())
    goto fail_hypfs_sprp_exit;
    rc = hypfs_fs_init();
    if (rc)
    goto fail_hypfs_diag0c_exit;
    return 0;
    fail_hypfs_diag0c_exit:
    hypfs_diag0c_exit();
    fail_hypfs_sprp_exit:
    hypfs_sprp_exit();
    hypfs_vm_exit();
    fail_hypfs_diag_exit:
    hypfs_diag_exit();
    pr_err("Initialization of hypfs failed with rc=%i\n", rc);
    fail_dbfs_exit:
    debugfs_remove(dbfs_dir);
    return rc;
    }
    device_initcall(hypfs_dbfs_init)
