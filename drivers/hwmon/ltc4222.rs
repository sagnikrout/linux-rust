//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc4222.c
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
// Driver for Linear Technology LTC4222 Dual Hot Swap controller
//
// Copyright (c) 2014 Guenter Roeck
//

// chip registers
pub const LTC4222_CONTROL1: c_uint = 0xd0;
pub const LTC4222_ALERT1: c_uint = 0xd1;
pub const LTC4222_STATUS1: c_uint = 0xd2;
pub const LTC4222_FAULT1: c_uint = 0xd3;
pub const LTC4222_CONTROL2: c_uint = 0xd4;
pub const LTC4222_ALERT2: c_uint = 0xd5;
pub const LTC4222_STATUS2: c_uint = 0xd6;
pub const LTC4222_FAULT2: c_uint = 0xd7;
pub const LTC4222_SOURCE1: c_uint = 0xd8;
pub const LTC4222_SOURCE2: c_uint = 0xda;
pub const LTC4222_ADIN1: c_uint = 0xdc;
pub const LTC4222_ADIN2: c_uint = 0xde;
pub const LTC4222_SENSE1: c_uint = 0xe0;
pub const LTC4222_SENSE2: c_uint = 0xe2;
pub const LTC4222_ADC_CONTROL: c_uint = 0xe4;
//
// Fault register bits
//

// Return the voltage from the given register in mV or mA
#[no_mangle]
unsafe extern "C" fn ltc4222_get_value(dev: *mut device, reg: u8) -> c_int {
    static int ltc4222_get_value(struct device *dev, u8 reg)
    {
    struct regmap *regmap = dev_get_drvdata(dev);
    unsigned int val;
    u8 buf[2];
    int ret;
    ret = regmap_bulk_read(regmap, reg, buf, 2);
    if (ret < 0)
    return ret;
    val = ((buf[0] << 8) + buf[1]) >> 6;
    switch (reg) {
    case LTC4222_ADIN1:
    case LTC4222_ADIN2:
// 1.25 mV resolution. Convert to mV.
    val = DIV_ROUND_CLOSEST(val * 5, 4);
    break;
    case LTC4222_SOURCE1:
    case LTC4222_SOURCE2:
// 31.25 mV resolution. Convert to mV.
    val = DIV_ROUND_CLOSEST(val * 125, 4);
    break;
    case LTC4222_SENSE1:
    case LTC4222_SENSE2:
//
// 62.5 uV resolution. Convert to current as measured with
// an 1 mOhm sense resistor, in mA. If a different sense
// resistor is installed, calculate the actual current by
// dividing the reported current by the sense resistor value
// in mOhm.
//
    val = DIV_ROUND_CLOSEST(val * 125, 2);
    break;
    default:
    return -EINVAL;
    }
    return val;
    }
    static ssize_t ltc4222_value_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    int value;
    value = ltc4222_get_value(dev, attr.index);
    if (value < 0)
    return value;
    return sysfs_emit(buf, "%d\n", value);
    }
    static ssize_t ltc4222_bool_show(struct device *dev,
    struct device_attribute *da, char *buf)
    {
    struct sensor_device_attribute_2 *attr = to_sensor_dev_attr_2(da);
    struct regmap *regmap = dev_get_drvdata(dev);
    unsigned int fault;
    int ret;
    ret = regmap_read(regmap, attr.nr, &fault);
    if (ret < 0)
    return ret;
    fault &= attr.index;
    if (fault)		/* Clear reported faults in chip register */
    regmap_update_bits(regmap, attr.nr, attr.index, 0);
    return sysfs_emit(buf, "%d\n", !!fault);
    }
// Voltages
    static SENSOR_DEVICE_ATTR_RO(in1_input, ltc4222_value, LTC4222_SOURCE1);
    static SENSOR_DEVICE_ATTR_RO(in2_input, ltc4222_value, LTC4222_ADIN1);
    static SENSOR_DEVICE_ATTR_RO(in3_input, ltc4222_value, LTC4222_SOURCE2);
    static SENSOR_DEVICE_ATTR_RO(in4_input, ltc4222_value, LTC4222_ADIN2);
//
// Voltage alarms
// UV/OV faults are associated with the input voltage, and power bad and fet
// faults are associated with the output voltage.
//
    static SENSOR_DEVICE_ATTR_2_RO(in1_min_alarm, ltc4222_bool, LTC4222_FAULT1,
    FAULT_UV);
    static SENSOR_DEVICE_ATTR_2_RO(in1_max_alarm, ltc4222_bool, LTC4222_FAULT1,
    FAULT_OV);
    static SENSOR_DEVICE_ATTR_2_RO(in2_alarm, ltc4222_bool, LTC4222_FAULT1,
    FAULT_POWER_BAD | FAULT_FET_BAD);
    static SENSOR_DEVICE_ATTR_2_RO(in3_min_alarm, ltc4222_bool, LTC4222_FAULT2,
    FAULT_UV);
    static SENSOR_DEVICE_ATTR_2_RO(in3_max_alarm, ltc4222_bool, LTC4222_FAULT2,
    FAULT_OV);
    static SENSOR_DEVICE_ATTR_2_RO(in4_alarm, ltc4222_bool, LTC4222_FAULT2,
    FAULT_POWER_BAD | FAULT_FET_BAD);
// Current (via sense resistor)
    static SENSOR_DEVICE_ATTR_RO(curr1_input, ltc4222_value, LTC4222_SENSE1);
    static SENSOR_DEVICE_ATTR_RO(curr2_input, ltc4222_value, LTC4222_SENSE2);
// Overcurrent alarm
    static SENSOR_DEVICE_ATTR_2_RO(curr1_max_alarm, ltc4222_bool, LTC4222_FAULT1,
    FAULT_OC);
    static SENSOR_DEVICE_ATTR_2_RO(curr2_max_alarm, ltc4222_bool, LTC4222_FAULT2,
    FAULT_OC);
    static struct attribute *ltc4222_attrs[] = {
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_min_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in3_min_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_max_alarm.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in4_alarm.dev_attr.attr,
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    &sensor_dev_attr_curr1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_curr2_input.dev_attr.attr,
    &sensor_dev_attr_curr2_max_alarm.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ltc4222);
    static const struct regmap_config ltc4222_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = LTC4222_ADC_CONTROL,
    };
#[no_mangle]
unsafe extern "C" fn ltc4222_probe(client: *mut i2c_client) -> c_int {
    static int ltc4222_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &ltc4222_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to allocate register map\n");
    return PTR_ERR(regmap);
    }
// Clear faults
    regmap_write(regmap, LTC4222_FAULT1, 0x00);
    regmap_write(regmap, LTC4222_FAULT2, 0x00);
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    regmap,
    ltc4222_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id ltc4222_id[] = {
    { .name = "ltc4222" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc4222_id);
    static struct i2c_driver ltc4222_driver = {
    .driver = {
    .name = "ltc4222",
    },
    .probe = ltc4222_probe,
    .id_table = ltc4222_id,
    };
    module_i2c_driver(ltc4222_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("LTC4222 driver");
    MODULE_LICENSE("GPL");
