//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/nsa320-hwmon.c
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
// drivers/hwmon/nsa320-hwmon.c
//
// ZyXEL NSA320 Media Servers
// hardware monitoring
//
// Copyright (C) 2016 Adam Baker <linux@baker-net.org.uk>
// based on a board file driver
// Copyright (C) 2012 Peter Schildmann <linux@schildmann.info>
//

// Tests for error return values rely upon this value being < 0x80
pub const MAGIC_NUMBER: c_uint = 0x55;
//
// The Zyxel hwmon MCU is a Holtek HT46R065 that is factory programmed
// to perform temperature and fan speed monitoring. It is read by taking
// the active pin low. The 32 bit output word is then clocked onto the
// data line. The MSB of the data word is a magic nuber to indicate it
// has been read correctly, the next byte is the fan speed (in hundreds
// of RPM) and the last two bytes are the temperature (in tenths of a
// degree)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsa320_hwmon {
    pub /: *mut *mut mutex update_lock; / lock GPIO operations,
    pub /: *mut *mut unsigned long last_updated; / jiffies,
    pub mcu_data: c_ulong,
    pub act: *mut gpio_desc,
    pub clk: *mut gpio_desc,
    pub data: *mut gpio_desc,
}

    enum nsa320_inputs {
    NSA320_TEMP = 0,
    NSA320_FAN = 1,
    };
    static const char * const nsa320_input_names[] = {
    [NSA320_TEMP] = "System Temperature",
    [NSA320_FAN] = "Chassis Fan",
    };
//
// Although this protocol looks similar to SPI the long delay
// between the active (aka chip select) signal and the shorter
// delay between clock pulses are needed for reliable operation.
// The delays provided are taken from the manufacturer kernel,
// testing suggest they probably incorporate a reasonable safety
// margin. (The single device tested became unreliable if the
// delay was reduced to 1/10th of this value.)
//
#[no_mangle]
unsafe extern "C" fn nsa320_hwmon_update(dev: *mut device) -> i32 {
    static s32 nsa320_hwmon_update(struct device *dev)
    {
    u32 mcu_data;
    u32 mask;
    struct nsa320_hwmon *hwmon = dev_get_drvdata(dev);
    mutex_lock(&hwmon.update_lock);
    mcu_data = hwmon.mcu_data;
    if (time_after(jiffies, hwmon.last_updated + HZ) || mcu_data == 0) {
    gpiod_set_value(hwmon.act, 1);
    msleep(100);
    mcu_data = 0;
    for (mask = BIT(31); mask; mask >>= 1) {
    gpiod_set_value(hwmon.clk, 0);
    usleep_range(100, 200);
    gpiod_set_value(hwmon.clk, 1);
    usleep_range(100, 200);
    if (gpiod_get_value(hwmon.data))
    mcu_data |= mask;
    }
    gpiod_set_value(hwmon.act, 0);
    dev_dbg(dev, "Read raw MCU data %08x\n", mcu_data);
    if ((mcu_data >> 24) != MAGIC_NUMBER) {
    dev_dbg(dev, "Read invalid MCU data %08x\n", mcu_data);
    mcu_data = -EIO;
    } else {
    hwmon.mcu_data = mcu_data;
    hwmon.last_updated = jiffies;
    }
    }
    mutex_unlock(&hwmon.update_lock);
    return mcu_data;
    }
    static ssize_t label_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut channel: c_int = to_sensor_dev_attr(attr).index;
    return sprintf(buf, "%s\n", nsa320_input_names[channel]);
    }
    static ssize_t temp1_input_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut mcu_data: i32 = nsa320_hwmon_update(dev);
    if (mcu_data < 0)
    return mcu_data;
    return sprintf(buf, "%d\n", (mcu_data & 0xffff) * 100);
    }
    static ssize_t fan1_input_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut mcu_data: i32 = nsa320_hwmon_update(dev);
    if (mcu_data < 0)
    return mcu_data;
    return sprintf(buf, "%d\n", ((mcu_data & 0xff0000) >> 16) * 100);
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_label, label, NSA320_TEMP);
    static DEVICE_ATTR_RO(temp1_input);
    static SENSOR_DEVICE_ATTR_RO(fan1_label, label, NSA320_FAN);
    static DEVICE_ATTR_RO(fan1_input);
    static struct attribute *nsa320_attrs[] = {
    &sensor_dev_attr_temp1_label.dev_attr.attr,
    &dev_attr_temp1_input.attr,
    &sensor_dev_attr_fan1_label.dev_attr.attr,
    &dev_attr_fan1_input.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(nsa320);
    static const struct of_device_id of_nsa320_hwmon_match[] = {
    { .compatible = "zyxel,nsa320-mcu", },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_nsa320_hwmon_match);
#[no_mangle]
unsafe extern "C" fn nsa320_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int nsa320_hwmon_probe(struct platform_device *pdev)
    {
    struct nsa320_hwmon	*hwmon;
    struct device		*classdev;
    hwmon = devm_kzalloc(&pdev.dev, sizeof(*hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
// Look up the GPIO pins to use
    hwmon.act = devm_gpiod_get(&pdev.dev, "act", GPIOD_OUT_LOW);
    if (IS_ERR(hwmon.act))
    return PTR_ERR(hwmon.act);
    hwmon.clk = devm_gpiod_get(&pdev.dev, "clk", GPIOD_OUT_HIGH);
    if (IS_ERR(hwmon.clk))
    return PTR_ERR(hwmon.clk);
    hwmon.data = devm_gpiod_get(&pdev.dev, "data", GPIOD_IN);
    if (IS_ERR(hwmon.data))
    return PTR_ERR(hwmon.data);
    mutex_init(&hwmon.update_lock);
    classdev = devm_hwmon_device_register_with_groups(&pdev.dev,
    "nsa320", hwmon, nsa320_groups);
    return PTR_ERR_OR_ZERO(classdev);
    }
// All allocations use devres so remove() is not needed.
    static struct platform_driver nsa320_hwmon_driver = {
    .probe = nsa320_hwmon_probe,
    .driver = {
    .name = "nsa320-hwmon",
    .of_match_table = of_nsa320_hwmon_match,
    },
    };
    module_platform_driver(nsa320_hwmon_driver);
    MODULE_AUTHOR("Peter Schildmann <linux@schildmann.info>");
    MODULE_AUTHOR("Adam Baker <linux@baker-net.org.uk>");
    MODULE_DESCRIPTION("NSA320 Hardware Monitoring");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:nsa320-hwmon");
