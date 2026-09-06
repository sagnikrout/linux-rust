//! Automatically rewritten from C to Rust
//! Source: drivers/w1/slaves/w1_ds2405.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// w1_ds2405.c
//
// Copyright (c) 2017 Maciej S. Szmigiero <mail@maciej.szmigiero.name>
// Based on w1_therm.c copyright (c) 2004 Evgeniy Polyakov <zbr@ioremap.net>
//

pub const W1_FAMILY_DS2405: c_uint = 0x05;
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Maciej S. Szmigiero <mail@maciej.szmigiero.name>");
    MODULE_DESCRIPTION("Driver for 1-wire Dallas DS2405 PIO.");
    MODULE_ALIAS("w1-family-" __stringify(W1_FAMILY_DS2405));
#[no_mangle]
unsafe extern "C" fn w1_ds2405_select(sl: *mut w1_slave, only_active: bool) -> c_int {
    static int w1_ds2405_select(struct w1_slave *sl, bool only_active)
    {
    struct w1_master *dev = sl.master;
    let mut dev_addr: u64 = le64_to_cpu(*(u64 *)&sl.reg_num);
    unsigned int bit_ctr;
    if (w1_reset_bus(dev) != 0)
    return 0;
//
// We cannot use a normal Match ROM command
// since doing so would toggle PIO state
//
    w1_write_8(dev, only_active ? W1_ALARM_SEARCH : W1_SEARCH);
    for (bit_ctr = 0; bit_ctr < 64; bit_ctr++) {
    let mut bit2send: c_int = !!(dev_addr & BIT(bit_ctr));
    u8 ret;
    ret = w1_triplet(dev, bit2send);
    if ((ret & (BIT(0) | BIT(1))) ==
    (BIT(0) | BIT(1))) /* no devices found */
    return 0;
    if (!!(ret & BIT(2)) != bit2send)
// wrong direction taken - no such device
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn w1_ds2405_read_pio(sl: *mut w1_slave) -> c_int {
    static int w1_ds2405_read_pio(struct w1_slave *sl)
    {
    if (w1_ds2405_select(sl, true))
    return 0; /* "active" means PIO is low */
    if (w1_ds2405_select(sl, false))
    return 1;
    return -ENODEV;
    }
    static ssize_t state_show(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct w1_slave *sl = dev_to_w1_slave(device);
    struct w1_master *dev = sl.master;
    int ret;
    ssize_t f_retval;
    u8 state;
    ret = mutex_lock_interruptible(&dev.bus_mutex);
    if (ret)
    return ret;
    if (!w1_ds2405_select(sl, false)) {
    f_retval = -ENODEV;
    goto out_unlock;
    }
    state = w1_read_8(dev);
    if (state != 0 &&
    state != 0xff) {
    dev_err(device, "non-consistent state %x\n", state);
    f_retval = -EIO;
    goto out_unlock;
    }
// buf = state ? '1' : '0';
    f_retval = 1;
    out_unlock:
    w1_reset_bus(dev);
    mutex_unlock(&dev.bus_mutex);
    return f_retval;
    }
    static ssize_t output_show(struct device *device,
    struct device_attribute *attr, char *buf)
    {
    struct w1_slave *sl = dev_to_w1_slave(device);
    struct w1_master *dev = sl.master;
    int ret;
    ssize_t f_retval;
    ret = mutex_lock_interruptible(&dev.bus_mutex);
    if (ret)
    return ret;
    ret = w1_ds2405_read_pio(sl);
    if (ret < 0) {
    f_retval = ret;
    goto out_unlock;
    }
// buf = ret ? '1' : '0';
    f_retval = 1;
    out_unlock:
    w1_reset_bus(dev);
    mutex_unlock(&dev.bus_mutex);
    return f_retval;
    }
    static ssize_t output_store(struct device *device,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct w1_slave *sl = dev_to_w1_slave(device);
    struct w1_master *dev = sl.master;
    int ret, current_pio;
    unsigned int val;
    ssize_t f_retval;
    if (count < 1)
    return -EINVAL;
    if (sscanf(buf, " %u%n", &val, &ret) < 1)
    return -EINVAL;
    if (val != 0 && val != 1)
    return -EINVAL;
    f_retval = ret;
    ret = mutex_lock_interruptible(&dev.bus_mutex);
    if (ret)
    return ret;
    current_pio = w1_ds2405_read_pio(sl);
    if (current_pio < 0) {
    f_retval = current_pio;
    goto out_unlock;
    }
    if (current_pio == val)
    goto out_unlock;
    if (w1_reset_bus(dev) != 0) {
    f_retval = -ENODEV;
    goto out_unlock;
    }
//
// can't use w1_reset_select_slave() here since it uses Skip ROM if
// there is only one device on bus
//
    do {
    let mut dev_addr: u64 = le64_to_cpu(*(u64 *)&sl.reg_num);
    u8 cmd[9];
    cmd[0] = W1_MATCH_ROM;
    memcpy(&cmd[1], &dev_addr, sizeof(dev_addr));
    w1_write_block(dev, cmd, sizeof(cmd));
    } while (0);
    out_unlock:
    w1_reset_bus(dev);
    mutex_unlock(&dev.bus_mutex);
    return f_retval;
    }
    static DEVICE_ATTR_RO(state);
    static DEVICE_ATTR_RW(output);
    static struct attribute *w1_ds2405_attrs[] = {
    &dev_attr_state.attr,
    &dev_attr_output.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(w1_ds2405);
    static const struct w1_family_ops w1_ds2405_fops = {
    .groups = w1_ds2405_groups
    };
    static struct w1_family w1_family_ds2405 = {
    .fid = W1_FAMILY_DS2405,
    .fops = &w1_ds2405_fops
    };
    module_w1_family(w1_family_ds2405);
