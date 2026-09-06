//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/thmc50.c
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
// thmc50.c - Part of lm_sensors, Linux kernel modules for hardware
// monitoring
// Copyright (C) 2007 Krzysztof Helt <krzysztof.h1@wp.pl>
// Based on 2.4 driver by Frodo Looijaard <frodol@dds.nl> and
// Philip Edelbrock <phil@netroedge.com>
//

    MODULE_LICENSE("GPL");
// Addresses to scan
    static const unsigned short normal_i2c[] = { 0x2c, 0x2d, 0x2e, I2C_CLIENT_END };
// Insmod parameters
    enum chips { thmc50, adm1022 };
    static unsigned short adm1022_temp3[16];
    static unsigned int adm1022_temp3_num;
    module_param_array(adm1022_temp3, ushort, &adm1022_temp3_num, 0);
    MODULE_PARM_DESC(adm1022_temp3,
    "List of adapter,address pairs to enable 3rd temperature (ADM1022 only)");
// Many THMC50 constants specified below
// The THMC50 registers
pub const THMC50_REG_CONF: c_uint = 0x40;
pub const THMC50_REG_COMPANY_ID: c_uint = 0x3E;
pub const THMC50_REG_DIE_CODE: c_uint = 0x3F;
pub const THMC50_REG_ANALOG_OUT: c_uint = 0x19;
//
// The mirror status register cannot be used as
// reading it does not clear alarms.
//
pub const THMC50_REG_INTR: c_uint = 0x41;
    static const u8 THMC50_REG_TEMP[] = { 0x27, 0x26, 0x20 };
    static const u8 THMC50_REG_TEMP_MIN[] = { 0x3A, 0x38, 0x2C };
    static const u8 THMC50_REG_TEMP_MAX[] = { 0x39, 0x37, 0x2B };
    static const u8 THMC50_REG_TEMP_CRITICAL[] = { 0x13, 0x14, 0x14 };
    static const u8 THMC50_REG_TEMP_DEFAULT[] = { 0x17, 0x18, 0x18 };
pub const THMC50_REG_CONF_nFANOFF: c_uint = 0x20;
pub const THMC50_REG_CONF_PROGRAMMED: c_uint = 0x08;
// Each client has this additional data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thmc50_data {
    pub client: *mut i2c_client,
    pub groups: [*const attribute_group; 3],
    pub update_lock: mutex,
    pub type: enum chips,
    pub /: *mut *mut unsigned long last_updated; / In jiffies,
    pub /: *mut *mut char has_temp3; / !=0 if it is ADM1022 in temp3 mode,
    pub /: *mut *mut bool valid; / true if following fields are valid,
// Register values
    pub temp_input: [i8; 3],
    pub temp_max: [i8; 3],
    pub temp_min: [i8; 3],
    pub temp_critical: [i8; 3],
    pub analog_out: u8,
    pub alarms: u8,
}

    static struct thmc50_data *thmc50_update_device(struct device *dev)
    {
    struct thmc50_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    let mut timeout: c_int = HZ / 5 + (data.type == thmc50 ? HZ : 0);
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + timeout)
    || !data.valid) {
    let mut temps: c_int = data.has_temp3 ? 3 : 2;
    int i;
    let mut prog: c_int = i2c_smbus_read_byte_data(client, THMC50_REG_CONF);
    prog &= THMC50_REG_CONF_PROGRAMMED;
    for (i = 0; i < temps; i++) {
    data.temp_input[i] = i2c_smbus_read_byte_data(client,
    THMC50_REG_TEMP[i]);
    data.temp_max[i] = i2c_smbus_read_byte_data(client,
    THMC50_REG_TEMP_MAX[i]);
    data.temp_min[i] = i2c_smbus_read_byte_data(client,
    THMC50_REG_TEMP_MIN[i]);
    data.temp_critical[i] =
    i2c_smbus_read_byte_data(client,
    prog ? THMC50_REG_TEMP_CRITICAL[i]
    : THMC50_REG_TEMP_DEFAULT[i]);
    }
    data.analog_out =
    i2c_smbus_read_byte_data(client, THMC50_REG_ANALOG_OUT);
    data.alarms =
    i2c_smbus_read_byte_data(client, THMC50_REG_INTR);
    data.last_updated = jiffies;
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
    static ssize_t analog_out_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%d\n", data.analog_out);
    }
    static ssize_t analog_out_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct thmc50_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int config;
    unsigned long tmp;
    int err;
    err = kstrtoul(buf, 10, &tmp);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.analog_out = clamp_val(tmp, 0, 255);
    i2c_smbus_write_byte_data(client, THMC50_REG_ANALOG_OUT,
    data.analog_out);
    config = i2c_smbus_read_byte_data(client, THMC50_REG_CONF);
    if (data.analog_out == 0)
    config &= ~THMC50_REG_CONF_nFANOFF;
    else
    config |= THMC50_REG_CONF_nFANOFF;
    i2c_smbus_write_byte_data(client, THMC50_REG_CONF, config);
    mutex_unlock(&data.update_lock);
    return count;
    }
// There is only one PWM mode = DC
    static ssize_t pwm_mode_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return sprintf(buf, "0\n");
    }
// Temperatures
    static ssize_t temp_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%d\n", data.temp_input[nr] * 1000);
    }
    static ssize_t temp_min_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%d\n", data.temp_min[nr] * 1000);
    }
    static ssize_t temp_min_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long val;
    int err;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.temp_min[nr] = clamp_val(val / 1000, -128, 127);
    i2c_smbus_write_byte_data(client, THMC50_REG_TEMP_MIN[nr],
    data.temp_min[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%d\n", data.temp_max[nr] * 1000);
    }
    static ssize_t temp_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf,
    size_t count)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    long val;
    int err;
    err = kstrtol(buf, 10, &val);
    if (err)
    return err;
    mutex_lock(&data.update_lock);
    data.temp_max[nr] = clamp_val(val / 1000, -128, 127);
    i2c_smbus_write_byte_data(client, THMC50_REG_TEMP_MAX[nr],
    data.temp_max[nr]);
    mutex_unlock(&data.update_lock);
    return count;
    }
    static ssize_t temp_critical_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    let mut nr: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%d\n", data.temp_critical[nr] * 1000);
    }
    static ssize_t alarm_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    let mut index: c_int = to_sensor_dev_attr(attr).index;
    struct thmc50_data *data = thmc50_update_device(dev);
    return sprintf(buf, "%u\n", (data.alarms >> index) & 1);
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_min, temp_min, 0);
    static SENSOR_DEVICE_ATTR_RW(temp1_max, temp_max, 0);
    static SENSOR_DEVICE_ATTR_RO(temp1_crit, temp_critical, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_input, temp, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_min, temp_min, 1);
    static SENSOR_DEVICE_ATTR_RW(temp2_max, temp_max, 1);
    static SENSOR_DEVICE_ATTR_RO(temp2_crit, temp_critical, 1);
    static SENSOR_DEVICE_ATTR_RO(temp3_input, temp, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_min, temp_min, 2);
    static SENSOR_DEVICE_ATTR_RW(temp3_max, temp_max, 2);
    static SENSOR_DEVICE_ATTR_RO(temp3_crit, temp_critical, 2);
    static SENSOR_DEVICE_ATTR_RO(temp1_alarm, alarm, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_alarm, alarm, 5);
    static SENSOR_DEVICE_ATTR_RO(temp3_alarm, alarm, 1);
    static SENSOR_DEVICE_ATTR_RO(temp2_fault, alarm, 7);
    static SENSOR_DEVICE_ATTR_RO(temp3_fault, alarm, 2);
    static SENSOR_DEVICE_ATTR_RW(pwm1, analog_out, 0);
    static SENSOR_DEVICE_ATTR_RO(pwm1_mode, pwm_mode, 0);
    static struct attribute *thmc50_attributes[] = {
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_crit.dev_attr.attr,
    &sensor_dev_attr_temp1_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp2_min.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_crit.dev_attr.attr,
    &sensor_dev_attr_temp2_alarm.dev_attr.attr,
    &sensor_dev_attr_temp2_fault.dev_attr.attr,
    &sensor_dev_attr_pwm1.dev_attr.attr,
    &sensor_dev_attr_pwm1_mode.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group thmc50_group = {
    .attrs = thmc50_attributes,
    };
// for ADM1022 3rd temperature mode
    static struct attribute *temp3_attributes[] = {
    &sensor_dev_attr_temp3_max.dev_attr.attr,
    &sensor_dev_attr_temp3_min.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp3_crit.dev_attr.attr,
    &sensor_dev_attr_temp3_alarm.dev_attr.attr,
    &sensor_dev_attr_temp3_fault.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group temp3_group = {
    .attrs = temp3_attributes,
    };
// Return 0 if detection is successful, -ENODEV otherwise
    static int thmc50_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    unsigned company;
    unsigned revision;
    unsigned config;
    struct i2c_adapter *adapter = client.adapter;
    const char *type_name;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA)) {
    pr_debug("thmc50: detect failed, smbus byte data not supported!\n");
    return -ENODEV;
    }
    pr_debug("thmc50: Probing for THMC50 at 0x%2X on bus %d\n",
    client.addr, i2c_adapter_id(client.adapter));
    company = i2c_smbus_read_byte_data(client, THMC50_REG_COMPANY_ID);
    revision = i2c_smbus_read_byte_data(client, THMC50_REG_DIE_CODE);
    config = i2c_smbus_read_byte_data(client, THMC50_REG_CONF);
    if (revision < 0xc0 || (config & 0x10))
    return -ENODEV;
    if (company == 0x41) {
    let mut id: c_int = i2c_adapter_id(client.adapter);
    int i;
    type_name = "adm1022";
    for (i = 0; i + 1 < adm1022_temp3_num; i += 2)
    if (adm1022_temp3[i] == id &&
    adm1022_temp3[i + 1] == client.addr) {
// enable 2nd remote temp
    config |= (1 << 7);
    i2c_smbus_write_byte_data(client,
    THMC50_REG_CONF,
    config);
    break;
    }
    } else if (company == 0x49) {
    type_name = "thmc50";
    } else {
    pr_debug("thmc50: Detection of THMC50/ADM1022 failed\n");
    return -ENODEV;
    }
    pr_debug("thmc50: Detected %s (version %x, revision %x)\n",
    type_name, (revision >> 4) - 0xc, revision & 0xf);
    strscpy(info.type, type_name, I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn thmc50_init_client(data: *mut thmc50_data) {
    static void thmc50_init_client(struct thmc50_data *data)
    {
    struct i2c_client *client = data.client;
    int config;
    data.analog_out = i2c_smbus_read_byte_data(client,
    THMC50_REG_ANALOG_OUT);
// set up to at least 1
    if (data.analog_out == 0) {
    data.analog_out = 1;
    i2c_smbus_write_byte_data(client, THMC50_REG_ANALOG_OUT,
    data.analog_out);
    }
    config = i2c_smbus_read_byte_data(client, THMC50_REG_CONF);
    config |= 0x1;	/* start the chip if it is in standby mode */
    if (data.type == adm1022 && (config & (1 << 7)))
    data.has_temp3 = 1;
    i2c_smbus_write_byte_data(client, THMC50_REG_CONF, config);
    }
#[no_mangle]
unsafe extern "C" fn thmc50_probe(client: *mut i2c_client) -> c_int {
    static int thmc50_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct thmc50_data *data;
    struct device *hwmon_dev;
    let mut idx: c_int = 0;
    data = devm_kzalloc(dev, sizeof(struct thmc50_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    data.type = (uintptr_t)i2c_get_match_data(client);
    mutex_init(&data.update_lock);
    thmc50_init_client(data);
// sysfs hooks
    data.groups[idx++] = &thmc50_group;
// Register additional ADM1022 sysfs hooks
    if (data.has_temp3)
    data.groups[idx++] = &temp3_group;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, data.groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id thmc50_id[] = {
    { .name = "adm1022", .driver_data = adm1022 },
    { .name = "thmc50", .driver_data = thmc50 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, thmc50_id);
    static struct i2c_driver thmc50_driver = {
    .class = I2C_CLASS_HWMON,
    .driver = {
    .name = "thmc50",
    },
    .probe = thmc50_probe,
    .id_table = thmc50_id,
    .detect = thmc50_detect,
    .address_list = normal_i2c,
    };
    module_i2c_driver(thmc50_driver);
    MODULE_AUTHOR("Krzysztof Helt <krzysztof.h1@wp.pl>");
    MODULE_DESCRIPTION("THMC50 driver");
