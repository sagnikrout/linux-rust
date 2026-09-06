//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adc128d818.c
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
// Driver for TI ADC128D818 System Monitor with Temperature Sensor
//
// Copyright (c) 2014 Guenter Roeck
//
// Derived from lm80.c
// Copyright (C) 1998, 1999  Frodo Looijaard <frodol@dds.nl>
// and Philip Edelbrock <phil@netroedge.com>
//

// Addresses to scan
// The chip also supports addresses 0x35..0x37. Don't scan those addresses
// since they are also used by some EEPROMs, which may result in false
// positives.
//
    static const unsigned short normal_i2c[] = {
    0x1d, 0x1e, 0x1f, 0x2d, 0x2e, 0x2f, I2C_CLIENT_END };
// registers

pub const ADC128_REG_TEMP: c_uint = 0x27;
pub const ADC128_REG_TEMP_MAX: c_uint = 0x38;
pub const ADC128_REG_TEMP_HYST: c_uint = 0x39;
pub const ADC128_REG_CONFIG: c_uint = 0x00;
pub const ADC128_REG_ALARM: c_uint = 0x01;
pub const ADC128_REG_MASK: c_uint = 0x03;
pub const ADC128_REG_CONV_RATE: c_uint = 0x07;
pub const ADC128_REG_ONESHOT: c_uint = 0x09;
pub const ADC128_REG_SHUTDOWN: c_uint = 0x0a;
pub const ADC128_REG_CONFIG_ADV: c_uint = 0x0b;
pub const ADC128_REG_BUSY_STATUS: c_uint = 0x0c;
pub const ADC128_REG_MAN_ID: c_uint = 0x3e;
pub const ADC128_REG_DEV_ID: c_uint = 0x3f;
// No. of voltage entries in adc128_attrs

// Voltage inputs visible per operation mode
    static const u8 num_inputs[] = { 7, 8, 4, 6 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adc128_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut int vref; / Reference voltage in mV,
    pub update_lock: mutex,
    pub /: *mut *mut u8 mode; / Operation mode,
    pub /: *mut *mut bool valid; / true if following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub bit: *mut *mut u16 in[3][8]; / Register value, normalized to 12,
// 0: input voltage
// 1: min limit
// 2: max limit
//
    pub bit: *mut *mut s16 temp[3]; / Register value, normalized to 9,
// 0: sensor 1: limit 2: hyst
//
    pub /: *mut *mut u8 alarms; / alarm register value,
}

    static struct adc128_data *adc128_update_device(struct device *dev)
    {
    struct adc128_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct adc128_data *ret = data;
    int i, rv;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ) || !data.valid) {
    for (i = 0; i < num_inputs[data.mode]; i++) {
    rv = i2c_smbus_read_word_swapped(client,
    ADC128_REG_IN(i));
    if (rv < 0)
    goto abort;
    data.in[0][i] = rv >> 4;
    rv = i2c_smbus_read_byte_data(client,
    ADC128_REG_IN_MIN(i));
    if (rv < 0)
    goto abort;
    data.in[1][i] = rv << 4;
    rv = i2c_smbus_read_byte_data(client,
    ADC128_REG_IN_MAX(i));
    if (rv < 0)
    goto abort;
    data.in[2][i] = rv << 4;
    }
    if (data.mode != 1) {
    rv = i2c_smbus_read_word_swapped(client,
    ADC128_REG_TEMP);
    if (rv < 0)
    goto abort;
    data.temp[0] = rv >> 7;
    rv = i2c_smbus_read_byte_data(client,
    ADC128_REG_TEMP_MAX);
    if (rv < 0)
    goto abort;
    data.temp[1] = rv << 1;
    rv = i2c_smbus_read_byte_data(client,
    ADC128_REG_TEMP_HYST);
    if (rv < 0)
    goto abort;
    data.temp[2] = rv << 1;
    }
    rv = i2c_smbus_read_byte_data(client, ADC128_REG_ALARM);
    if (rv < 0)
    goto abort;
    data.alarms |= rv;
    data.last_updated = jiffies;
    data.valid = true;
    }
    goto done;
    abort:
    ret = ERR_PTR(rv);
    data.valid = false;
    done:
    mutex_unlock(&data.update_lock);
    return ret;
    }
    static ssize_t adc128_in_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adc128_data *data = adc128_update_device(dev);
    let mut index: c_int = to_sensor_dev_attr_2(attr).index;
    let mut nr: c_int = to_sensor_dev_attr_2(attr).nr;
    int val;
    if (IS_ERR(data))
    return PTR_ERR(data);
    val = DIV_ROUND_CLOSEST(data.in[index][nr] * data.vref, 4095);
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t adc128_in_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    struct adc128_data *data = dev_get_drvdata(dev);
    let mut index: c_int = to_sensor_dev_attr_2(attr).index;
    let mut nr: c_int = to_sensor_dev_attr_2(attr).nr;
    u8 reg, regval;
    long val;
    int err;
    err = kstrtol(buf, 10, &val);
    if (err < 0)
    return err;
    mutex_lock(&data.update_lock);
// 10 mV LSB on limit registers
    regval = DIV_ROUND_CLOSEST(clamp_val(val, 0, 2550), 10);
    data.in[index][nr] = regval << 4;
    reg = index == 1 ? ADC128_REG_IN_MIN(nr) : ADC128_REG_IN_MAX(nr);
    i2c_smbus_write_byte_data(data.client, reg, regval);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t adc128_temp_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adc128_data *data = adc128_update_device(dev);
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    int temp;
    if (IS_ERR(data))
    return PTR_ERR(data);
    temp = sign_extend32(data.temp[index], 8);
    return sprintf(buf, "%d\n", temp * 500);/* 0.5 degrees C resolution */
    }
    static ssize_t adc128_temp_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adc128_data *data = dev_get_drvdata(dev);
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    long val;
    int err;
    s8 regval;
    err = kstrtol(buf, 10, &val);
    if (err < 0)
    return err;
    mutex_lock(&data.update_lock);
    regval = DIV_ROUND_CLOSEST(clamp_val(val, -128000, 127000), 1000);
    data.temp[index] = regval << 1;
    i2c_smbus_write_byte_data(data.client,
    index == 1 ? ADC128_REG_TEMP_MAX
    : ADC128_REG_TEMP_HYST,
    regval);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t adc128_alarm_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adc128_data *data = adc128_update_device(dev);
    let mut mask: c_int = 1 << to_sensor_dev_attr(attr).index;
    u8 alarms;
    if (IS_ERR(data))
    return PTR_ERR(data);
//
// Clear an alarm after reporting it to user space. If it is still
// active, the next update sequence will set the alarm bit again.
//
    alarms = data.alarms;
    data.alarms &= ~mask;
    return sprintf(buf, "%u\n", !!(alarms & mask));
    }
    static umode_t adc128_is_visible(struct kobject *kobj,
    struct attribute *attr, int index)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct adc128_data *data = dev_get_drvdata(dev);
    if (index < ADC128_ATTR_NUM_VOLT) {
// Voltage, visible according to num_inputs[]
    if (index >= num_inputs[data.mode] * 4)
    return 0;
    } else {
// Temperature, visible if not in mode 1
    if (data.mode == 1)
    return 0;
    }
    return attr.mode;
    }
    static SENSOR_DEVICE_ATTR_2_RO(in0_input, adc128_in, 0, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in0_min, adc128_in, 0, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in0_max, adc128_in, 0, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in1_input, adc128_in, 1, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in1_min, adc128_in, 1, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in1_max, adc128_in, 1, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in2_input, adc128_in, 2, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in2_min, adc128_in, 2, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in2_max, adc128_in, 2, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in3_input, adc128_in, 3, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in3_min, adc128_in, 3, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in3_max, adc128_in, 3, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in4_input, adc128_in, 4, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in4_min, adc128_in, 4, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in4_max, adc128_in, 4, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in5_input, adc128_in, 5, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in5_min, adc128_in, 5, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in5_max, adc128_in, 5, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in6_input, adc128_in, 6, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in6_min, adc128_in, 6, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in6_max, adc128_in, 6, 2);
    static SENSOR_DEVICE_ATTR_2_RO(in7_input, adc128_in, 7, 0);
    static SENSOR_DEVICE_ATTR_2_RW(in7_min, adc128_in, 7, 1);
    static SENSOR_DEVICE_ATTR_2_RW(in7_max, adc128_in, 7, 2);
    static SENSOR_DEVICE_ATTR_RO(temp1_input, adc128_temp, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_max, adc128_temp, 1);
    static SENSOR_DEVICE_ATTR_RW(temp1_max_hyst, adc128_temp, 2);
    static SENSOR_DEVICE_ATTR_RO(in0_alarm, adc128_alarm, 0);
    static SENSOR_DEVICE_ATTR_RO(in1_alarm, adc128_alarm, 1);
    static SENSOR_DEVICE_ATTR_RO(in2_alarm, adc128_alarm, 2);
    static SENSOR_DEVICE_ATTR_RO(in3_alarm, adc128_alarm, 3);
    static SENSOR_DEVICE_ATTR_RO(in4_alarm, adc128_alarm, 4);
    static SENSOR_DEVICE_ATTR_RO(in5_alarm, adc128_alarm, 5);
    static SENSOR_DEVICE_ATTR_RO(in6_alarm, adc128_alarm, 6);
    static SENSOR_DEVICE_ATTR_RO(in7_alarm, adc128_alarm, 7);
    static SENSOR_DEVICE_ATTR_RO(temp1_max_alarm, adc128_alarm, 7);
    static struct attribute *adc128_attrs[] = {
    &sensor_dev_attr_in0_alarm.dev_attr.attr,
    &sensor_dev_attr_in0_input.dev_attr.attr,
    &sensor_dev_attr_in0_max.dev_attr.attr,
    &sensor_dev_attr_in0_min.dev_attr.attr,
    &sensor_dev_attr_in1_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_max.dev_attr.attr,
    &sensor_dev_attr_in1_min.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_in2_max.dev_attr.attr,
    &sensor_dev_attr_in2_min.dev_attr.attr,
    &sensor_dev_attr_in3_alarm.dev_attr.attr,
    &sensor_dev_attr_in3_input.dev_attr.attr,
    &sensor_dev_attr_in3_max.dev_attr.attr,
    &sensor_dev_attr_in3_min.dev_attr.attr,
    &sensor_dev_attr_in4_alarm.dev_attr.attr,
    &sensor_dev_attr_in4_input.dev_attr.attr,
    &sensor_dev_attr_in4_max.dev_attr.attr,
    &sensor_dev_attr_in4_min.dev_attr.attr,
    &sensor_dev_attr_in5_alarm.dev_attr.attr,
    &sensor_dev_attr_in5_input.dev_attr.attr,
    &sensor_dev_attr_in5_max.dev_attr.attr,
    &sensor_dev_attr_in5_min.dev_attr.attr,
    &sensor_dev_attr_in6_alarm.dev_attr.attr,
    &sensor_dev_attr_in6_input.dev_attr.attr,
    &sensor_dev_attr_in6_max.dev_attr.attr,
    &sensor_dev_attr_in6_min.dev_attr.attr,
    &sensor_dev_attr_in7_alarm.dev_attr.attr,
    &sensor_dev_attr_in7_input.dev_attr.attr,
    &sensor_dev_attr_in7_max.dev_attr.attr,
    &sensor_dev_attr_in7_min.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_max_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_max_hyst.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group adc128_group = {
    .attrs = adc128_attrs,
    .is_visible = adc128_is_visible,
    };
    __ATTRIBUTE_GROUPS(adc128);
#[no_mangle]
unsafe extern "C" fn adc128_detect(client: *mut i2c_client, info: *mut i2c_board_info) -> c_int {
    static int adc128_detect(struct i2c_client *client, struct i2c_board_info *info)
    {
    int man_id, dev_id;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA))
    return -ENODEV;
    man_id = i2c_smbus_read_byte_data(client, ADC128_REG_MAN_ID);
    dev_id = i2c_smbus_read_byte_data(client, ADC128_REG_DEV_ID);
    if (man_id != 0x01 || dev_id != 0x09)
    return -ENODEV;
// Check unused bits for confirmation
    if (i2c_smbus_read_byte_data(client, ADC128_REG_CONFIG) & 0xf4)
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, ADC128_REG_CONV_RATE) & 0xfe)
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, ADC128_REG_ONESHOT) & 0xfe)
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, ADC128_REG_SHUTDOWN) & 0xfe)
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, ADC128_REG_CONFIG_ADV) & 0xf8)
    return -ENODEV;
    if (i2c_smbus_read_byte_data(client, ADC128_REG_BUSY_STATUS) & 0xfc)
    return -ENODEV;
    strscpy(info.type, "adc128d818", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc128_init_client(data: *mut adc128_data, external_vref: bool) -> c_int {
    static int adc128_init_client(struct adc128_data *data, bool external_vref)
    {
    struct i2c_client *client = data.client;
    int err;
    let mut regval: u8 = 0x0;
//
// Reset chip to defaults.
// This makes most other initializations unnecessary.
//
    err = i2c_smbus_write_byte_data(client, ADC128_REG_CONFIG, 0x80);
    if (err)
    return err;
// Set operation mode, if non-default
    if (data.mode != 0)
    regval |= data.mode << 1;
// If external vref is selected, configure the chip to use it
    if (external_vref)
    regval |= 0x01;
// Write advanced configuration register
    if (regval != 0x0) {
    err = i2c_smbus_write_byte_data(client, ADC128_REG_CONFIG_ADV,
    regval);
    if (err)
    return err;
    }
// Start monitoring
    err = i2c_smbus_write_byte_data(client, ADC128_REG_CONFIG, 0x01);
    if (err)
    return err;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adc128_probe(client: *mut i2c_client) -> c_int {
    static int adc128_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct adc128_data *data;
    bool external_vref;
    int err, vref;
    data = devm_kzalloc(dev, sizeof(struct adc128_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
// vref is optional. If specified, is used as chip reference voltage
    vref = devm_regulator_get_enable_read_voltage(dev, "vref");
    if (vref == -ENODEV) {
    external_vref = false;
    data.vref = 2560;	/* 2.56V, in mV */
    } else if (vref < 0) {
    return vref;
    } else {
    external_vref = true;
    data.vref = DIV_ROUND_CLOSEST(vref, 1000);
    }
// Operation mode is optional. If unspecified, keep current mode
    if (of_property_read_u8(dev.of_node, "ti,mode", &data.mode) == 0) {
    if (data.mode > 3) {
    dev_err(dev, "invalid operation mode %d\n",
    data.mode);
    return -EINVAL;
    }
    } else {
    err = i2c_smbus_read_byte_data(client, ADC128_REG_CONFIG_ADV);
    if (err < 0)
    return err;
    data.mode = (err >> 1) & ADC128_REG_MASK;
    }
    data.client = client;
    i2c_set_clientdata(client, data);
    mutex_init(&data.update_lock);
// Initialize the chip
    err = adc128_init_client(data, external_vref);
    if (err < 0)
    return err;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, adc128_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id adc128_id[] = {
    { .name = "adc128d818" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adc128_id);
    static const struct of_device_id __maybe_unused adc128_of_match[] = {
    { .compatible = "ti,adc128d818" },
    { },
    };
    MODULE_DEVICE_TABLE(of, adc128_of_match);
    static struct i2c_driver adc128_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name	= "adc128d818",
    .of_match_table = of_match_ptr(adc128_of_match),
    },
    .probe		= adc128_probe,
    .id_table	= adc128_id,
    .detect		= adc128_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(adc128_driver);
    MODULE_AUTHOR("Guenter Roeck");
    MODULE_DESCRIPTION("Driver for ADC128D818");
    MODULE_LICENSE("GPL");
