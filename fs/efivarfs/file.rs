//! Automatically rewritten from C to Rust
//! Source: fs/efivarfs/file.c
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
// Copyright (C) 2012 Red Hat, Inc.
// Copyright (C) 2012 Jeremy Kerr <jeremy.kerr@canonical.com>
//

    static ssize_t efivarfs_file_write(struct file *file,
    const char __user *userbuf, size_t count, loff_t *ppos)
    {
    struct efivar_entry *var = file.private_data;
    void *data;
    u32 attributes;
    struct inode *inode = file.f_mapping.host;
    let mut datasize: c_ulong = count - sizeof(attributes);
    ssize_t bytes;
    let mut set: bool = false;
    if (count < sizeof(attributes))
    return -EINVAL;
    if (copy_from_user(&attributes, userbuf, sizeof(attributes)))
    return -EFAULT;
    if (attributes & ~(EFI_VARIABLE_MASK))
    return -EINVAL;
    data = memdup_user(userbuf + sizeof(attributes), datasize);
    if (IS_ERR(data))
    return PTR_ERR(data);
    inode_lock(inode);
    if (var.removed) {
//
// file got removed; don't allow a set.  Caused by an
// unsuccessful create or successful delete write
// racing with us.
//
    bytes = -EIO;
    goto out;
    }
    bytes = efivar_entry_set_get_size(var, attributes, &datasize,
    data, &set);
    if (!set) {
    if (bytes == -ENOENT)
    bytes = -EIO;
    goto out;
    }
    if (bytes == -ENOENT) {
//
// zero size signals to release that the write deleted
// the variable
//
    i_size_write(inode, 0);
    } else {
    i_size_write(inode, datasize + sizeof(attributes));
    inode_set_mtime_to_ts(inode, inode_set_ctime_current(inode));
    }
    bytes = count;
    out:
    inode_unlock(inode);
    kfree(data);
    return bytes;
    }
    static ssize_t efivarfs_file_read(struct file *file, char __user *userbuf,
    size_t count, loff_t *ppos)
    {
    struct efivar_entry *var = file.private_data;
    let mut datasize: c_ulong = 0;
    u32 attributes;
    void *data;
    let mut size: isize = 0;
    int err;
    while (!__ratelimit(&file.f_cred.user.ratelimit))
    msleep(50);
    err = efivar_entry_size(var, &datasize);
//
// efivarfs represents uncommitted variables with
// zero-length files. Reading them should return EOF.
//
    if (err == -ENOENT)
    return 0;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    else if (err)
    return err;
    data = kmalloc(datasize + sizeof(attributes), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    size = efivar_entry_get(var, &attributes, &datasize,
    data + sizeof(attributes));
    if (size)
    goto out_free;
    memcpy(data, &attributes, sizeof(attributes));
    size = simple_read_from_buffer(userbuf, count, ppos,
    data, datasize + sizeof(attributes));
    out_free:
    kfree(data);
    return size;
    }
#[no_mangle]
unsafe extern "C" fn efivarfs_file_release(inode: *mut inode, file: *mut file) -> c_int {
    static int efivarfs_file_release(struct inode *inode, struct file *file)
    {
    struct efivar_entry *var = inode.i_private;
    inode_lock(inode);
    var.removed = (--var.open_count == 0 && i_size_read(inode) == 0);
    inode_unlock(inode);
    if (var.removed)
    simple_recursive_removal(file.f_path.dentry, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efivarfs_file_open(inode: *mut inode, file: *mut file) -> c_int {
    static int efivarfs_file_open(struct inode *inode, struct file *file)
    {
    struct efivar_entry *entry = inode.i_private;
    file.private_data = entry;
    inode_lock(inode);
    entry.open_count++;
    inode_unlock(inode);
    return 0;
    }
    const struct file_operations efivarfs_file_operations = {
    .open		= efivarfs_file_open,
    .read		= efivarfs_file_read,
    .write		= efivarfs_file_write,
    .release	= efivarfs_file_release,
    };
