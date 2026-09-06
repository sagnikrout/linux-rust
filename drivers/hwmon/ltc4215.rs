//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc4215.c
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
// Driver for Linear Technology LTC4215 I2C Hot Swap Controller
//
// Copyright (C) 2009 Ira W. Snyder <iws@ovro.caltech.edu>
//
// Datasheet:
// http://www.linear.com/pc/downloadDocument.do?navId=H0,C1,C1003,C1006,C1163,P17572,D12697
//

// Here are names of the chip's registers (a.k.a. commands)
    enum ltc4215_cmd {
    LTC4215_CONTROL			= 0x00, /* rw */
    LTC4215_ALERT			= 0x01, /* rw */
    LTC4215_STATUS			= 0x02, /* ro */
    LTC4215_FAULT			= 0x03, /* rw */
    LTC4215_SENSE			= 0x04, /* rw */
    LTC4215_SOURCE			= 0x05, /* rw */
    LTC4215_ADIN			= 0x06, /* rw */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc4215_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub valid: bool,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
// Registers
    pub regs: [u8; 7],
}

    static struct ltc4215_data *ltc4215_update_device(struct device *dev)
    {
    struct ltc4215_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    s32 val;
    int i;
    mutex_lock(&data.update_lock);
// The chip's A/D updates 10 times per second
    if (time_after(jiffies, data.last_updated + HZ / 10) || !data.valid) {
    dev_dbg(&client.dev, "Starting ltc4215 update\n");
// Read all registers
    for (i = 0; i < ARRAY_SIZE(data.regs); i++) {
    val = i2c_smbus_read_byte_data(client, i);
    if (unlikely(val < 0))
    data.regs[i] = 0;
    else
    data.regs[i] = val;
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
// Return the voltage from the given register in millivolts
#[no_mangle]
unsafe extern "C" fn ltc4215_get_voltage(dev: *mut device, reg: u8) -> c_int {
    static int ltc4215_get_voltage(struct device *dev, u8 reg)
    {
    struct ltc4215_data *data = ltc4215_update_device(dev);
    let mut regval: u8 = data.regs[reg];
    let mut voltage: u32 = 0;
    switch (reg) {
    case LTC4215_SENSE:
// 151 uV per increment
    voltage = regval * 151 / 1000;
    break;
    case LTC4215_SOURCE:
// 60.5 mV per increment
    voltage = regval * 605 / 10;
    break;
    case LTC4215_ADIN:
//
// The ADIN input is divided by 12.5, and has 4.82 mV
// per increment, so we have the additional multiply
//
    voltage = regval * 482 * 125 / 1000;
    break;
    default:
// If we get here, the developer messed up
    WARN_ON_ONCE(1);
    break;
    }
    return voltage;
    }
// Return the current from the sense resistor in mA
#[no_mangle]
unsafe extern "C" fn ltc4215_get_current(dev: *mut device) -> c_uint {
    static unsigned int ltc4215_get_current(struct device *dev)
    {
    struct ltc4215_data *data = ltc4215_update_device(dev);
//
// The strange looking conversions that follow are fixed-point
// math, since we cannot do floating point in the kernel.
//
// Step 1: convert sense register to microVolts
// Step 2: convert voltage to milliAmperes
//
// If you play around with the V=IR equation, you come up with
// the following: X uV / Y mOhm == Z mA
//
// With the resistors that are fractions of a milliOhm, we multiply
// the voltage and resistance by 10, to shift the decimal point.
// Now we can use the normal division operator again.
//
// Calculate voltage in microVolts (151 uV per increment)
    let mut voltage: c_uint = data.regs[LTC4215_SENSE] * 151;
// Calculate current in milliAmperes (4 milliOhm sense resistor)
    let mut curr: c_uint = voltage / 4;
    return curr;
    }
    static ssize_t ltc4215_voltage_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    let mut voltage: c_int = ltc4215_get_voltage(dev, attr.index);
    return sysfs_emit(buf, "%d\n", voltage);
    }
    static ssize_t ltc4215_current_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    let mut curr: c_uint = ltc4215_get_current(dev);
    return sysfs_emit(buf, "%u\n", curr);
    }
    static ssize_t ltc4215_power_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    let mut curr: c_uint = ltc4215_get_current(dev);
    let mut output_voltage: c_int = ltc4215_get_voltage(dev, LTC4215_ADIN);
// current in mA * voltage in mV == power in uW
    let mut power: c_uint = abs(output_voltage * curr);
    return sysfs_emit(buf, "%u\n", power);
    }
    static ssize_t ltc4215_alarm_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ltc4215_data *data = ltc4215_update_device(dev);
    let mut reg: u8 = data.regs[LTC4215_STATUS];
    let mut mask: u32 = attr.index;
    return sysfs_emit(buf, "%u\n", !!(reg & mask));
    }
//
// These macros are used below in constructing device attribute objects
// for use with sysfs_create_group() to make a sysfs device file
// for each register.
//
// Construct a sensor_device_attribute structure for each register
// Current
    static SENSOR_DEVICE_ATTR_RO(curr1_input, ltc4215_current, 0);
    static SENSOR_DEVICE_ATTR_RO(curr1_max_alarm, ltc4215_alarm, 1 << 2);
// Power (virtual)
    static SENSOR_DEVICE_ATTR_RO(power1_input, ltc4215_power, 0);
// Input Voltage
    static SENSOR_DEVICE_ATTR_RO(in1_input, ltc4215_voltage, LTC4215_ADIN);
    static SENSOR_DEVICE_ATTR_RO(in1_max_alarm, ltc4215_alarm, 1 << 0);
    static SENSOR_DEVICE_ATTR_RO(in1_min_alarm, ltc4215_alarm, 1 << 1);
// Output Voltage
    static SENSOR_DEVICE_ATTR_RO(in2_input, ltc4215_voltage, LTC4215_SOURCE);
    static SENSOR_DEVICE_ATTR_RO(in2_min_alarm, ltc4215_alarm, 1 << 3);
//
// Finally, construct an array of pointers to members of the above objects,
// as required for sysfs_create_group()
//
    static struct attribute *ltc4215_attrs[] = {
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    &sensor_dev_attr_curr1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_power1_input.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_min_alarm.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ltc4215);
#[no_mangle]
unsafe extern "C" fn ltc4215_probe(client: *mut i2c_client) -> c_int {
    static int ltc4215_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct ltc4215_data *data;
    struct device *hwmon_dev;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
// Initialize the LTC4215 chip
    i2c_smbus_write_byte_data(client, LTC4215_FAULT, 0x00);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    ltc4215_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id ltc4215_id[] = {
    { .name = "ltc4215" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc4215_id);
// This is the driver that will be inserted
    static struct i2c_driver ltc4215_driver = {
    .driver = {
    .name	= "ltc4215",
    },
    .probe		= ltc4215_probe,
    .id_table	= ltc4215_id,
    };
    module_i2c_driver(ltc4215_driver);
    MODULE_AUTHOR("Ira W. Snyder <iws@ovro.caltech.edu>");
    MODULE_DESCRIPTION("LTC4215 driver");
    MODULE_LICENSE("GPL");
