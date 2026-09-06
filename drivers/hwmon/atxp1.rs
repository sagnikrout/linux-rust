//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/atxp1.c
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
// atxp1.c - kernel module for setting CPU VID and general purpose
// I/Os using the Attansic ATXP1 chip.
//
// The ATXP1 can reside on I2C addresses 0x37 or 0x4e. The chip is
// not auto-detected by the driver and must be instantiated explicitly.
// See Documentation/i2c/instantiating-devices.rst for more information.
//

    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("System voltages control via Attansic ATXP1");
    MODULE_VERSION("0.6.3");
    MODULE_AUTHOR("Sebastian Witt <se.witt@gmx.net>");
pub const ATXP1_VID: c_uint = 0x00;
pub const ATXP1_CVID: c_uint = 0x01;
pub const ATXP1_GPIO1: c_uint = 0x06;
pub const ATXP1_GPIO2: c_uint = 0x0a;
pub const ATXP1_VIDENA: c_uint = 0x20;
pub const ATXP1_VIDMASK: c_uint = 0x1f;
pub const ATXP1_GPIO1MASK: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atxp1_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub last_updated: c_ulong,
    pub valid: bool,
    struct {
    pub /: *mut *mut u8 vid; / VID output register,
    pub /: *mut *mut u8 cpu_vid; / VID input from CPU,
    pub /: *mut *mut u8 gpio1; / General purpose I/O register 1,
    pub /: *mut *mut u8 gpio2; / General purpose I/O register 2,
    pub reg: },
    pub /: *mut *mut u8 vrm; / Detected CPU VRM,
}

    static struct atxp1_data *atxp1_update_device(struct device *dev)
    {
    struct atxp1_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ) || !data.valid) {
// Update local register data
    data.reg.vid = i2c_smbus_read_byte_data(client, ATXP1_VID);
    data.reg.cpu_vid = i2c_smbus_read_byte_data(client,
    ATXP1_CVID);
    data.reg.gpio1 = i2c_smbus_read_byte_data(client, ATXP1_GPIO1);
    data.reg.gpio2 = i2c_smbus_read_byte_data(client, ATXP1_GPIO2);
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
// sys file functions for cpu0_vid
    static ssize_t cpu0_vid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int size;
    struct atxp1_data *data;
    data = atxp1_update_device(dev);
    size = sprintf(buf, "%d\n", vid_from_reg(data.reg.vid & ATXP1_VIDMASK,
    data.vrm));
    return size;
    }
    static ssize_t cpu0_vid_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct atxp1_data *data = atxp1_update_device(dev);
    struct i2c_client *client = data.client;
    int vid, cvid;
    unsigned long vcore;
    int err;
    err = kstrtoul(buf, 10, &vcore);
    if (err)
    return err;
    vcore /= 25;
    vcore *= 25;
// Calculate VID
    vid = vid_to_reg(vcore, data.vrm);
    if (vid < 0) {
    dev_err(dev, "VID calculation failed.\n");
    return vid;
    }
//
// If output enabled, use control register value.
// Otherwise original CPU VID
//
    if (data.reg.vid & ATXP1_VIDENA)
    cvid = data.reg.vid & ATXP1_VIDMASK;
    else
    cvid = data.reg.cpu_vid;
// Nothing changed, aborting
    if (vid == cvid)
    return count;
    dev_dbg(dev, "Setting VCore to %d mV (0x%02x)\n", (int)vcore, vid);
// Write every 25 mV step to increase stability
    if (cvid > vid) {
    for (; cvid >= vid; cvid--)
    i2c_smbus_write_byte_data(client,
    ATXP1_VID, cvid | ATXP1_VIDENA);
    } else {
    for (; cvid <= vid; cvid++)
    i2c_smbus_write_byte_data(client,
    ATXP1_VID, cvid | ATXP1_VIDENA);
    }
    data.valid = false;
    return count;
    }
//
// CPU core reference voltage
// unit: millivolt
//
    static DEVICE_ATTR_RW(cpu0_vid);
// sys file functions for GPIO1
    static ssize_t gpio1_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    int size;
    struct atxp1_data *data;
    data = atxp1_update_device(dev);
    size = sprintf(buf, "0x%02x\n", data.reg.gpio1 & ATXP1_GPIO1MASK);
    return size;
    }
    static ssize_t gpio1_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct atxp1_data *data = atxp1_update_device(dev);
    struct i2c_client *client = data.client;
    unsigned long value;
    int err;
    err = kstrtoul(buf, 16, &value);
    if (err)
    return err;
    value &= ATXP1_GPIO1MASK;
    if (value != (data.reg.gpio1 & ATXP1_GPIO1MASK)) {
    dev_info(dev, "Writing 0x%x to GPIO1.\n", (unsigned int)value);
    i2c_smbus_write_byte_data(client, ATXP1_GPIO1, value);
    data.valid = false;
    }
    return count;
    }
//
// GPIO1 data register
// unit: Four bit as hex (e.g. 0x0f)
//
    static DEVICE_ATTR_RW(gpio1);
// sys file functions for GPIO2
    static ssize_t gpio2_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    int size;
    struct atxp1_data *data;
    data = atxp1_update_device(dev);
    size = sprintf(buf, "0x%02x\n", data.reg.gpio2);
    return size;
    }
    static ssize_t gpio2_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct atxp1_data *data = atxp1_update_device(dev);
    struct i2c_client *client = data.client;
    unsigned long value;
    int err;
    err = kstrtoul(buf, 16, &value);
    if (err)
    return err;
    value &= 0xff;
    if (value != data.reg.gpio2) {
    dev_info(dev, "Writing 0x%x to GPIO1.\n", (unsigned int)value);
    i2c_smbus_write_byte_data(client, ATXP1_GPIO2, value);
    data.valid = false;
    }
    return count;
    }
//
// GPIO2 data register
// unit: Eight bit as hex (e.g. 0xff)
//
    static DEVICE_ATTR_RW(gpio2);
    static struct attribute *atxp1_attrs[] = {
    &dev_attr_gpio1.attr,
    &dev_attr_gpio2.attr,
    &dev_attr_cpu0_vid.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(atxp1);
#[no_mangle]
unsafe extern "C" fn atxp1_probe(client: *mut i2c_client) -> c_int {
    static int atxp1_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct atxp1_data *data;
    struct device *hwmon_dev;
    data = devm_kzalloc(dev, sizeof(struct atxp1_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
// Get VRM
    data.vrm = vid_which_vrm();
    if (data.vrm != 90 && data.vrm != 91) {
    dev_err(dev, "atxp1: Not supporting VRM %d.%d\n",
    data.vrm / 10, data.vrm % 10);
    return -ENODEV;
    }
    data.client = client;
    mutex_init(&data.update_lock);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    atxp1_groups);
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    dev_info(dev, "Using VRM: %d.%d\n", data.vrm / 10, data.vrm % 10);
    return 0;
    };
    static const struct i2c_device_id atxp1_id[] = {
    { .name = "atxp1" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, atxp1_id);
    static struct i2c_driver atxp1_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "atxp1",
    },
    .probe		= atxp1_probe,
    .id_table	= atxp1_id,
    };
    module_i2c_driver(atxp1_driver);
