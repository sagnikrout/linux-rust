//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds2406.c
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
// w1_ds2406.c - w1 family 12 (DS2406) driver
// based on w1_ds2413.c by Mariusz Bialonczyk <manio@skyboo.net>
//
// Copyright (c) 2014 Scott Alfter <scott@alfter.us>
//

pub const W1_FAMILY_DS2406: c_uint = 0x12;
pub const W1_F12_FUNC_READ_STATUS: c_uint = 0xAA;
pub const W1_F12_FUNC_WRITE_STATUS: c_uint = 0x55;
    static ssize_t w1_f12_read_state(
    struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    u8 w1_buf[6] = {W1_F12_FUNC_READ_STATUS, 7, 0, 0, 0, 0};
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    let mut rtnval: isize = 1;
    if (off != 0)
    return 0;
    if (!buf)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_reset_select_slave(sl)) {
    mutex_unlock(&sl.master.bus_mutex);
    return -EIO;
    }
    w1_write_block(sl.master, w1_buf, 3);
    w1_read_block(sl.master, w1_buf+3, 3);
    if (crc16(0, w1_buf, sizeof(w1_buf)) == 0xb001) /* good read? */
// buf = ((w1_buf[3]>>5)&3)|0x30;
    else
    rtnval = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return rtnval;
    }
    static ssize_t w1_f12_write_output(
    struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    u8 w1_buf[6] = {W1_F12_FUNC_WRITE_STATUS, 7, 0, 0, 0, 0};
    let mut rtnval: isize = 1;
    if (count != 1 || off != 0)
    return -EFAULT;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_reset_select_slave(sl)) {
    mutex_unlock(&sl.master.bus_mutex);
    return -EIO;
    }
    w1_buf[3] = (((*buf)&3)<<5)|0x1F;
    w1_write_block(sl.master, w1_buf, 4);
    w1_read_block(sl.master, w1_buf+4, 2);
    if (crc16(0, w1_buf, sizeof(w1_buf)) == 0xb001) /* good read? */
    w1_write_8(sl.master, 0xFF);
    else
    rtnval = -EIO;
    mutex_unlock(&sl.master.bus_mutex);
    return rtnval;
    }
pub const NB_SYSFS_BIN_FILES: c_int = 2;
    static const struct bin_attribute w1_f12_sysfs_bin_files[NB_SYSFS_BIN_FILES] = {
    {
    .attr = {
    .name = "state",
    .mode = 0444,
    },
    .size = 1,
    .read = w1_f12_read_state,
    },
    {
    .attr = {
    .name = "output",
    .mode = 0664,
    },
    .size = 1,
    .write = w1_f12_write_output,
    }
    };
#[no_mangle]
unsafe extern "C" fn w1_f12_add_slave(sl: *mut w1_slave) -> c_int {
    static int w1_f12_add_slave(struct w1_slave *sl)
    {
    let mut err: c_int = 0;
    int i;
    for (i = 0; i < NB_SYSFS_BIN_FILES && !err; ++i)
    err = sysfs_create_bin_file(
    &sl.dev.kobj,
    &(w1_f12_sysfs_bin_files[i]));
    if (err)
    while (--i >= 0)
    sysfs_remove_bin_file(&sl.dev.kobj,
    &(w1_f12_sysfs_bin_files[i]));
    return err;
    }
#[no_mangle]
unsafe extern "C" fn w1_f12_remove_slave(sl: *mut w1_slave) {
    static void w1_f12_remove_slave(struct w1_slave *sl)
    {
    int i;
    for (i = NB_SYSFS_BIN_FILES - 1; i >= 0; --i)
    sysfs_remove_bin_file(&sl.dev.kobj,
    &(w1_f12_sysfs_bin_files[i]));
    }
    static const struct w1_family_ops w1_f12_fops = {
    .add_slave      = w1_f12_add_slave,
    .remove_slave   = w1_f12_remove_slave,
    };
    static struct w1_family w1_family_12 = {
    .fid = W1_FAMILY_DS2406,
    .fops = &w1_f12_fops,
    };
    module_w1_family(w1_family_12);
    MODULE_AUTHOR("Scott Alfter <scott@alfter.us>");
    MODULE_DESCRIPTION("w1 family 12 driver for DS2406 2 Pin IO");
    MODULE_LICENSE("GPL");
