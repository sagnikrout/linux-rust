//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/ec_sys.c
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
// ec_sys.c
//
// Copyright (C) 2010 SUSE Products GmbH/Novell
// Author:
// Thomas Renninger <trenn@suse.de>
//

    MODULE_AUTHOR("Thomas Renninger <trenn@suse.de>");
    MODULE_DESCRIPTION("ACPI EC sysfs access driver");
    MODULE_LICENSE("GPL");
    static bool write_support;
    module_param_hw(write_support, bool, other, 0644);
    MODULE_PARM_DESC(write_support, "Dangerous, reboot and removal of battery may "
    "be needed.");
pub const EC_SPACE_SIZE: c_int = 256;
    static struct dentry *acpi_ec_debugfs_dir;
    static ssize_t acpi_ec_read_io(struct file *f, char __user *buf,
    size_t count, loff_t *off)
    {
// Use this if support reading/writing multiple ECs exists in ec.c:
// struct acpi_ec *ec = ((struct seq_file *)f->private_data)->private;
//
    let mut size: c_uint = EC_SPACE_SIZE;
    let mut init_off: loff_t = *off;
    let mut err: c_int = 0;
    if (*off >= size)
    return 0;
    if (*off + count >= size) {
    size -= *off;
    count = size;
    } else
    size = count;
    while (size) {
    u8 byte_read;
    err = ec_read(*off, &byte_read);
    if (err)
    return err;
    if (put_user(byte_read, buf + *off - init_off)) {
    if (*off - init_off)
    return *off - init_off; /* partial read */
    return -EFAULT;
    }
// off += 1;
    size--;
    }
    return count;
    }
    static ssize_t acpi_ec_write_io(struct file *f, const char __user *buf,
    size_t count, loff_t *off)
    {
// Use this if support reading/writing multiple ECs exists in ec.c:
// struct acpi_ec *ec = ((struct seq_file *)f->private_data)->private;
//
    let mut size: c_uint = count;
    let mut init_off: loff_t = *off;
    let mut err: c_int = 0;
    if (!write_support)
    return -EINVAL;
    if (*off >= EC_SPACE_SIZE)
    return 0;
    if (*off + count >= EC_SPACE_SIZE) {
    size = EC_SPACE_SIZE - *off;
    count = size;
    }
    while (size) {
    u8 byte_write;
    if (get_user(byte_write, buf + *off - init_off)) {
    if (*off - init_off)
    return *off - init_off; /* partial write */
    return -EFAULT;
    }
    err = ec_write(*off, byte_write);
    if (err)
    return err;
// off += 1;
    size--;
    }
    return count;
    }
    static const struct file_operations acpi_ec_io_ops = {
    .owner = THIS_MODULE,
    .open  = simple_open,
    .read  = acpi_ec_read_io,
    .write = acpi_ec_write_io,
    .llseek = default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn acpi_ec_add_debugfs(ec: *mut acpi_ec, ec_device_count: c_uint) {
    static void acpi_ec_add_debugfs(struct acpi_ec *ec, unsigned int ec_device_count)
    {
    struct dentry *dev_dir;
    char name[64];
    let mut mode: umode_t = 0400;
    if (ec_device_count == 0)
    acpi_ec_debugfs_dir = debugfs_create_dir("ec", core::ptr::null_mut());
    sprintf(name, "ec%u", ec_device_count);
    dev_dir = debugfs_create_dir(name, acpi_ec_debugfs_dir);
    debugfs_create_x32("gpe", 0444, dev_dir, &first_ec.gpe);
    debugfs_create_bool("use_global_lock", 0444, dev_dir,
    &first_ec.global_lock);
    if (write_support)
    mode = 0600;
    debugfs_create_file("io", mode, dev_dir, ec, &acpi_ec_io_ops);
    }
#[no_mangle]
unsafe extern "C" fn acpi_ec_sys_init() -> int __init {
    static int __init acpi_ec_sys_init(void)
    {
    if (first_ec)
    acpi_ec_add_debugfs(first_ec, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_ec_sys_exit() -> void __exit {
    static void __exit acpi_ec_sys_exit(void)
    {
    debugfs_remove_recursive(acpi_ec_debugfs_dir);
    }
    module_init(acpi_ec_sys_init);
    module_exit(acpi_ec_sys_exit);
