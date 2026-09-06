//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc4151.c
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
// Driver for Linear Technology LTC4151 High Voltage I2C Current
// and Voltage Monitor
//
// Copyright (C) 2011 AppearTV AS
//
// Derived from:
//
// Driver for Linear Technology LTC4261 I2C Negative Voltage Hot
// Swap Controller
// Copyright (C) 2010 Ericsson AB.
//
// Datasheet: http://www.linear.com/docs/Datasheet/4151fc.pdf
//

// chip registers
pub const LTC4151_SENSE_H: c_uint = 0x00;
pub const LTC4151_SENSE_L: c_uint = 0x01;
pub const LTC4151_VIN_H: c_uint = 0x02;
pub const LTC4151_VIN_L: c_uint = 0x03;
pub const LTC4151_ADIN_H: c_uint = 0x04;
pub const LTC4151_ADIN_L: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc4151_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub valid: bool,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub /: *mut *mut unsigned int shunt; / in micro ohms,
// Registers
    pub regs: [u8; 6],
}

    static struct ltc4151_data *ltc4151_update_device(struct device *dev)
    {
    struct ltc4151_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct ltc4151_data *ret = data;
    mutex_lock(&data.update_lock);
//
// The chip's A/D updates 6 times per second
// (Conversion Rate 6 - 9 Hz)
//
    if (time_after(jiffies, data.last_updated + HZ / 6) || !data.valid) {
    int i;
    dev_dbg(&client.dev, "Starting ltc4151 update\n");
// Read all registers
    for (i = 0; i < ARRAY_SIZE(data.regs); i++) {
    int val;
    val = i2c_smbus_read_byte_data(client, i);
    if (unlikely(val < 0)) {
    dev_dbg(dev,
    "Failed to read ADC value: error %d\n",
    val);
    ret = ERR_PTR(val);
    goto abort;
    }
    data.regs[i] = val;
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    abort:
    mutex_unlock(&data.update_lock);
    return ret;
    }
// Return the voltage from the given register in mV
#[no_mangle]
unsafe extern "C" fn ltc4151_get_value(data: *mut ltc4151_data, reg: u8) -> c_int {
    static int ltc4151_get_value(struct ltc4151_data *data, u8 reg)
    {
    u32 val;
    val = (data.regs[reg] << 4) + (data.regs[reg + 1] >> 4);
    switch (reg) {
    case LTC4151_ADIN_H:
// 500uV resolution. Convert to mV.
    val = val * 500 / 1000;
    break;
    case LTC4151_SENSE_H:
//
// 20uV resolution. Convert to current as measured with
// a given sense resistor, in mA.
//
    val = val * 20 * 1000 / data.shunt;
    break;
    case LTC4151_VIN_H:
// 25 mV per increment
    val = val * 25;
    break;
    default:
// If we get here, the developer messed up
    WARN_ON_ONCE(1);
    val = 0;
    break;
    }
    return val;
    }
    static ssize_t ltc4151_value_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ltc4151_data *data = ltc4151_update_device(dev);
    int value;
    if (IS_ERR(data))
    return PTR_ERR(data);
    value = ltc4151_get_value(data, attr.index);
    return sysfs_emit(buf, "%d\n", value);
    }
//
// Input voltages.
//
    static SENSOR_DEVICE_ATTR_RO(in1_input, ltc4151_value, LTC4151_VIN_H);
    static SENSOR_DEVICE_ATTR_RO(in2_input, ltc4151_value, LTC4151_ADIN_H);
// Currents (via sense resistor)
    static SENSOR_DEVICE_ATTR_RO(curr1_input, ltc4151_value, LTC4151_SENSE_H);
//
// Finally, construct an array of pointers to members of the above objects,
// as required for sysfs_create_group()
//
    static struct attribute *ltc4151_attrs[] = {
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ltc4151);
#[no_mangle]
unsafe extern "C" fn ltc4151_probe(client: *mut i2c_client) -> c_int {
    static int ltc4151_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct ltc4151_data *data;
    struct device *hwmon_dev;
    u32 shunt;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    if (of_property_read_u32(client.dev.of_node,
    "shunt-resistor-micro-ohms", &shunt))
    shunt = 1000; /* 1 mOhm if not set via DT */
    if (shunt == 0)
    return -EINVAL;
    data.shunt = shunt;
    data.client = client;
    mutex_init(&data.update_lock);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    ltc4151_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id ltc4151_id[] = {
    { .name = "ltc4151" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc4151_id);
    static const struct of_device_id __maybe_unused ltc4151_match[] = {
    { .compatible = "lltc,ltc4151" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ltc4151_match);
// This is the driver that will be inserted
    static struct i2c_driver ltc4151_driver = {
    .driver = {
    .name	= "ltc4151",
    .of_match_table = of_match_ptr(ltc4151_match),
    },
    .probe		= ltc4151_probe,
    .id_table	= ltc4151_id,
    };
    module_i2c_driver(ltc4151_driver);
    MODULE_AUTHOR("Per Dalen <per.dalen@appeartv.com>");
    MODULE_DESCRIPTION("LTC4151 driver");
    MODULE_LICENSE("GPL");
