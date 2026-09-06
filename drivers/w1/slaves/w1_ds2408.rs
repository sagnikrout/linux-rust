//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds2408.c
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
// w1_ds2408.c - w1 family 29 (DS2408) driver
//
// Copyright (c) 2010 Jean-Francois Dagenais <dagenaisj@sonatest.com>
//

pub const W1_FAMILY_DS2408: c_uint = 0x29;
pub const W1_F29_RETRIES: c_int = 3;
pub const W1_F29_REG_LOGIG_STATE: c_uint = 0x88 /* R */;
pub const W1_F29_REG_OUTPUT_LATCH_STATE: c_uint = 0x89 /* R */;
pub const W1_F29_REG_ACTIVITY_LATCH_STATE: c_uint = 0x8A /* R */;
pub const W1_F29_REG_COND_SEARCH_SELECT_MASK: c_uint = 0x8B /* RW */;
pub const W1_F29_REG_COND_SEARCH_POL_SELECT: c_uint = 0x8C /* RW */;
pub const W1_F29_REG_CONTROL_AND_STATUS: c_uint = 0x8D /* RW */;
pub const W1_F29_FUNC_READ_PIO_REGS: c_uint = 0xF0;
pub const W1_F29_FUNC_CHANN_ACCESS_READ: c_uint = 0xF5;
pub const W1_F29_FUNC_CHANN_ACCESS_WRITE: c_uint = 0x5A;
// also used to write the control/status reg (0x8D):
pub const W1_F29_FUNC_WRITE_COND_SEARCH_REG: c_uint = 0xCC;
pub const W1_F29_FUNC_RESET_ACTIVITY_LATCHES: c_uint = 0xC3;
pub const W1_F29_SUCCESS_CONFIRM_BYTE: c_uint = 0xAA;
#[no_mangle]
unsafe extern "C" fn _read_reg(sl: *mut w1_slave, address: u8, buf: *mut c_uchar) -> c_int {
    static int _read_reg(struct w1_slave *sl, u8 address, unsigned char *buf)
    {
    u8 wrbuf[3];
    dev_dbg(&sl.dev, "Reading with slave: %p, reg addr: %0#4x, buff addr: %p",
    sl, (unsigned int)address, buf);
    if (!buf)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    dev_dbg(&sl.dev, "mutex locked");
    if (w1_reset_select_slave(sl)) {
    mutex_unlock(&sl.master.bus_mutex);
    return -EIO;
    }
    wrbuf[0] = W1_F29_FUNC_READ_PIO_REGS;
    wrbuf[1] = address;
    wrbuf[2] = 0;
    w1_write_block(sl.master, wrbuf, 3);
// buf = w1_read_8(sl->master);
    mutex_unlock(&sl.master.bus_mutex);
    dev_dbg(&sl.dev, "mutex unlocked");
    return 1;
    }
    static ssize_t state_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    dev_dbg(&kobj_to_w1_slave(kobj).dev,
    "Reading %s kobj: %p, off: %0#10x, count: %zu, buff addr: %p",
    bin_attr.attr.name, kobj, (unsigned int)off, count, buf);
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj), W1_F29_REG_LOGIG_STATE, buf);
    }
    static ssize_t output_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    dev_dbg(&kobj_to_w1_slave(kobj).dev,
    "Reading %s kobj: %p, off: %0#10x, count: %zu, buff addr: %p",
    bin_attr.attr.name, kobj, (unsigned int)off, count, buf);
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj),
    W1_F29_REG_OUTPUT_LATCH_STATE, buf);
    }
    static ssize_t activity_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    dev_dbg(&kobj_to_w1_slave(kobj).dev,
    "Reading %s kobj: %p, off: %0#10x, count: %zu, buff addr: %p",
    bin_attr.attr.name, kobj, (unsigned int)off, count, buf);
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj),
    W1_F29_REG_ACTIVITY_LATCH_STATE, buf);
    }
    static ssize_t cond_search_mask_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    dev_dbg(&kobj_to_w1_slave(kobj).dev,
    "Reading %s kobj: %p, off: %0#10x, count: %zu, buff addr: %p",
    bin_attr.attr.name, kobj, (unsigned int)off, count, buf);
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj),
    W1_F29_REG_COND_SEARCH_SELECT_MASK, buf);
    }
    static ssize_t cond_search_polarity_read(struct file *filp,
    struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj),
    W1_F29_REG_COND_SEARCH_POL_SELECT, buf);
    }
    static ssize_t status_control_read(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    if (count != 1 || off != 0)
    return -EFAULT;
    return _read_reg(kobj_to_w1_slave(kobj),
    W1_F29_REG_CONTROL_AND_STATUS, buf);
    }

#[no_mangle]
unsafe extern "C" fn optional_read_back_valid(sl: *mut w1_slave, expected: u8) -> bool {
    static bool optional_read_back_valid(struct w1_slave *sl, u8 expected)
    {
    u8 w1_buf[3];
    if (w1_reset_resume_command(sl.master))
    return false;
    w1_buf[0] = W1_F29_FUNC_READ_PIO_REGS;
    w1_buf[1] = W1_F29_REG_OUTPUT_LATCH_STATE;
    w1_buf[2] = 0;
    w1_write_block(sl.master, w1_buf, 3);
    return (w1_read_8(sl.master) == expected);
    }

#[no_mangle]
unsafe extern "C" fn optional_read_back_valid(sl: *mut w1_slave, expected: u8) -> bool {
    static bool optional_read_back_valid(struct w1_slave *sl, u8 expected)
    {
    return true;
    }

    static ssize_t output_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    u8 w1_buf[3];
    let mut retries: c_uint = W1_F29_RETRIES;
    let mut bytes_written: isize = -EIO;
    if (count != 1 || off != 0)
    return -EFAULT;
    dev_dbg(&sl.dev, "locking mutex for write_output");
    mutex_lock(&sl.master.bus_mutex);
    dev_dbg(&sl.dev, "mutex locked");
    if (w1_reset_select_slave(sl))
    goto out;
    do {
    w1_buf[0] = W1_F29_FUNC_CHANN_ACCESS_WRITE;
    w1_buf[1] = *buf;
    w1_buf[2] = ~(*buf);
    w1_write_block(sl.master, w1_buf, 3);
    if (w1_read_8(sl.master) == W1_F29_SUCCESS_CONFIRM_BYTE &&
    optional_read_back_valid(sl, *buf)) {
    bytes_written = 1;
    goto out;
    }
    if (w1_reset_resume_command(sl.master))
    goto out; /* unrecoverable error */
// try again, the slave is ready for a command
    } while (--retries);
    out:
    mutex_unlock(&sl.master.bus_mutex);
    dev_dbg(&sl.dev, "%s, mutex unlocked retries:%d\n",
    (bytes_written > 0) ? "succeeded" : "error", retries);
    return bytes_written;
    }
//
// Writing to the activity file resets the activity latches.
//
    static ssize_t activity_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr, char *buf,
    loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    let mut retries: c_uint = W1_F29_RETRIES;
    if (count != 1 || off != 0)
    return -EFAULT;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_reset_select_slave(sl))
    goto error;
    while (retries--) {
    w1_write_8(sl.master, W1_F29_FUNC_RESET_ACTIVITY_LATCHES);
    if (w1_read_8(sl.master) == W1_F29_SUCCESS_CONFIRM_BYTE) {
    mutex_unlock(&sl.master.bus_mutex);
    return 1;
    }
    if (w1_reset_resume_command(sl.master))
    goto error;
    }
    error:
    mutex_unlock(&sl.master.bus_mutex);
    return -EIO;
    }
    static ssize_t status_control_write(struct file *filp, struct kobject *kobj,
    const struct bin_attribute *bin_attr,
    char *buf, loff_t off, size_t count)
    {
    struct w1_slave *sl = kobj_to_w1_slave(kobj);
    u8 w1_buf[4];
    let mut retries: c_uint = W1_F29_RETRIES;
    if (count != 1 || off != 0)
    return -EFAULT;
    mutex_lock(&sl.master.bus_mutex);
    if (w1_reset_select_slave(sl))
    goto error;
    while (retries--) {
    w1_buf[0] = W1_F29_FUNC_WRITE_COND_SEARCH_REG;
    w1_buf[1] = W1_F29_REG_CONTROL_AND_STATUS;
    w1_buf[2] = 0;
    w1_buf[3] = *buf;
    w1_write_block(sl.master, w1_buf, 4);
    if (w1_reset_resume_command(sl.master))
    goto error;
    w1_buf[0] = W1_F29_FUNC_READ_PIO_REGS;
    w1_buf[1] = W1_F29_REG_CONTROL_AND_STATUS;
    w1_buf[2] = 0;
    w1_write_block(sl.master, w1_buf, 3);
    if (w1_read_8(sl.master) == *buf) {
// success!
    mutex_unlock(&sl.master.bus_mutex);
    return 1;
    }
    }
    error:
    mutex_unlock(&sl.master.bus_mutex);
    return -EIO;
    }
//
// This is a special sequence we must do to ensure the P0 output is not stuck
// in test mode. This is described in rev 2 of the ds2408's datasheet
// (http://datasheets.maximintegrated.com/en/ds/DS2408.pdf) under
// "APPLICATION INFORMATION/Power-up timing".
//
#[no_mangle]
unsafe extern "C" fn w1_f29_disable_test_mode(sl: *mut w1_slave) -> c_int {
    static int w1_f29_disable_test_mode(struct w1_slave *sl)
    {
    int res;
    u8 magic[10] = {0x96, };
    let mut rn: u64 = le64_to_cpu(*((u64 *)&sl.reg_num));
    memcpy(&magic[1], &rn, 8);
    magic[9] = 0x3C;
    mutex_lock(&sl.master.bus_mutex);
    res = w1_reset_bus(sl.master);
    if (res)
    goto out;
    w1_write_block(sl.master, magic, ARRAY_SIZE(magic));
    res = w1_reset_bus(sl.master);
    out:
    mutex_unlock(&sl.master.bus_mutex);
    return res;
    }
    static const BIN_ATTR_RO(state, 1);
    static const BIN_ATTR_RW(output, 1);
    static const BIN_ATTR_RW(activity, 1);
    static const BIN_ATTR_RO(cond_search_mask, 1);
    static const BIN_ATTR_RO(cond_search_polarity, 1);
    static const BIN_ATTR_RW(status_control, 1);
    static const struct bin_attribute *const w1_f29_bin_attrs[] = {
    &bin_attr_state,
    &bin_attr_output,
    &bin_attr_activity,
    &bin_attr_cond_search_mask,
    &bin_attr_cond_search_polarity,
    &bin_attr_status_control,
    core::ptr::null_mut(),
    };
    static const struct attribute_group w1_f29_group = {
    .bin_attrs = w1_f29_bin_attrs,
    };
    static const struct attribute_group *w1_f29_groups[] = {
    &w1_f29_group,
    core::ptr::null_mut(),
    };
    static const struct w1_family_ops w1_f29_fops = {
    .add_slave      = w1_f29_disable_test_mode,
    .groups		= w1_f29_groups,
    };
    static struct w1_family w1_family_29 = {
    .fid = W1_FAMILY_DS2408,
    .fops = &w1_f29_fops,
    };
    module_w1_family(w1_family_29);
    MODULE_AUTHOR("Jean-Francois Dagenais <dagenaisj@sonatest.com>");
    MODULE_DESCRIPTION("w1 family 29 driver for DS2408 8 Pin IO");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_DS2408));
