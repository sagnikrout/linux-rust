//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc4261.c
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
// Driver for Linear Technology LTC4261 I2C Negative Voltage Hot Swap Controller
//
// Copyright (C) 2010 Ericsson AB.
//
// Derived from:
//
// Driver for Linear Technology LTC4245 I2C Multiple Supply Hot Swap Controller
// Copyright (C) 2008 Ira W. Snyder <iws@ovro.caltech.edu>
//
// Datasheet: http://cds.linear.com/docs/Datasheet/42612fb.pdf
//

// chip registers
pub const LTC4261_STATUS: c_uint = 0x00	/* readonly */;
pub const LTC4261_FAULT: c_uint = 0x01;
pub const LTC4261_ALERT: c_uint = 0x02;
pub const LTC4261_CONTROL: c_uint = 0x03;
pub const LTC4261_SENSE_H: c_uint = 0x04;
pub const LTC4261_SENSE_L: c_uint = 0x05;
pub const LTC4261_ADIN2_H: c_uint = 0x06;
pub const LTC4261_ADIN2_L: c_uint = 0x07;
pub const LTC4261_ADIN_H: c_uint = 0x08;
pub const LTC4261_ADIN_L: c_uint = 0x09;
//
// Fault register bits
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc4261_data {
    pub client: *mut i2c_client,
    pub update_lock: mutex,
    pub valid: bool,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
// Registers
    pub regs: [u8; 10],
}

    static struct ltc4261_data *ltc4261_update_device(struct device *dev)
    {
    struct ltc4261_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct ltc4261_data *ret = data;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ / 4) || !data.valid) {
    int i;
// Read registers -- 0x00 to 0x09
    for (i = 0; i < ARRAY_SIZE(data.regs); i++) {
    int val;
    val = i2c_smbus_read_byte_data(client, i);
    if (unlikely(val < 0)) {
    dev_dbg(dev,
    "Failed to read ADC value: error %d\n",
    val);
    ret = ERR_PTR(val);
    data.valid = false;
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
// Return the voltage from the given register in mV or mA
#[no_mangle]
unsafe extern "C" fn ltc4261_get_value(data: *mut ltc4261_data, reg: u8) -> c_int {
    static int ltc4261_get_value(struct ltc4261_data *data, u8 reg)
    {
    u32 val;
    val = (data.regs[reg] << 2) + (data.regs[reg + 1] >> 6);
    switch (reg) {
    case LTC4261_ADIN_H:
    case LTC4261_ADIN2_H:
// 2.5mV resolution. Convert to mV.
    val = val * 25 / 10;
    break;
    case LTC4261_SENSE_H:
//
// 62.5uV resolution. Convert to current as measured with
// an 1 mOhm sense resistor, in mA. If a different sense
// resistor is installed, calculate the actual current by
// dividing the reported current by the sense resistor value
// in mOhm.
//
    val = val * 625 / 10;
    break;
    default:
// If we get here, the developer messed up
    WARN_ON_ONCE(1);
    val = 0;
    break;
    }
    return val;
    }
    static ssize_t ltc4261_value_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ltc4261_data *data = ltc4261_update_device(dev);
    int value;
    if (IS_ERR(data))
    return PTR_ERR(data);
    value = ltc4261_get_value(data, attr.index);
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t ltc4261_bool_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct ltc4261_data *data = ltc4261_update_device(dev);
    u8 fault;
    if (IS_ERR(data))
    return PTR_ERR(data);
    fault = data.regs[LTC4261_FAULT] & attr.index;
    if (fault)		/* Clear reported faults in chip register */
    i2c_smbus_write_byte_data(data.client, LTC4261_FAULT, ~fault);
    return sysfs_emit(buf, "%d\n", fault ? 1 : 0);
    }
//
// Input voltages.
//
    static SENSOR_DEVICE_ATTR_RO(in1_input, ltc4261_value, LTC4261_ADIN_H);
    static SENSOR_DEVICE_ATTR_RO(in2_input, ltc4261_value, LTC4261_ADIN2_H);
//
// Voltage alarms. The chip has only one set of voltage alarm status bits,
// triggered by input voltage alarms. In many designs, those alarms are
// associated with the ADIN2 sensor, due to the proximity of the ADIN2 pin
// to the OV pin. ADIN2 is, however, not available on all chip variants.
// To ensure that the alarm condition is reported to the user, report it
// with both voltage sensors.
//
    static SENSOR_DEVICE_ATTR_RO(in1_min_alarm, ltc4261_bool, FAULT_UV);
    static SENSOR_DEVICE_ATTR_RO(in1_max_alarm, ltc4261_bool, FAULT_OV);
    static SENSOR_DEVICE_ATTR_RO(in2_min_alarm, ltc4261_bool, FAULT_UV);
    static SENSOR_DEVICE_ATTR_RO(in2_max_alarm, ltc4261_bool, FAULT_OV);
// Currents (via sense resistor)
    static SENSOR_DEVICE_ATTR_RO(curr1_input, ltc4261_value, LTC4261_SENSE_H);
// Overcurrent alarm
    static SENSOR_DEVICE_ATTR_RO(curr1_max_alarm, ltc4261_bool, FAULT_OC);
    static struct attribute *ltc4261_attrs[] = {
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_min_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_max_alarm.dev_attr.attr,
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    &sensor_dev_attr_curr1_max_alarm.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ltc4261);
#[no_mangle]
unsafe extern "C" fn ltc4261_probe(client: *mut i2c_client) -> c_int {
    static int ltc4261_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct ltc4261_data *data;
    struct device *hwmon_dev;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, LTC4261_STATUS) < 0) {
    dev_err(dev, "Failed to read status register\n");
    return -ENODEV;
    }
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
// Clear faults
    i2c_smbus_write_byte_data(client, LTC4261_FAULT, 0x00);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    ltc4261_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id ltc4261_id[] = {
    { .name = "ltc4261" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc4261_id);
// This is the driver that will be inserted
    static struct i2c_driver ltc4261_driver = {
    .driver = {
    .name = "ltc4261",
    },
    .probe = ltc4261_probe,
    .id_table = ltc4261_id,
    };
    module_i2c_driver(ltc4261_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("LTC4261 driver");
    MODULE_LICENSE("GPL");
