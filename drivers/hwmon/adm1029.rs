//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adm1029.c
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


// SPDX-License-Identifier: GPL-2.0
//
// adm1029.c - Part of lm_sensors, Linux kernel modules for hardware monitoring
//
// Copyright (C) 2006 Corentin LABBE <clabbe.montjoie@gmail.com>
//
// Based on LM83 Driver by Jean Delvare <jdelvare@suse.de>
//
// Give only processor, motherboard temperatures and fan tachs
// Very rare chip please let me know if you use it
//
// http://www.analog.com/UploadedFiles/Data_Sheets/ADM1029.pdf
//

//
// Addresses to scan
//
    static const unsigned short normal_i2c[] = { 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
    0x2e, 0x2f, I2C_CLIENT_END
    };
//
// The ADM1029 registers
// Manufacturer ID is 0x41 for Analog Devices
//
pub const ADM1029_REG_MAN_ID: c_uint = 0x0D;
pub const ADM1029_REG_CHIP_ID: c_uint = 0x0E;
pub const ADM1029_REG_CONFIG: c_uint = 0x01;
pub const ADM1029_REG_NB_FAN_SUPPORT: c_uint = 0x02;
pub const ADM1029_REG_TEMP_DEVICES_INSTALLED: c_uint = 0x06;
pub const ADM1029_REG_LOCAL_TEMP: c_uint = 0xA0;
pub const ADM1029_REG_REMOTE1_TEMP: c_uint = 0xA1;
pub const ADM1029_REG_REMOTE2_TEMP: c_uint = 0xA2;
pub const ADM1029_REG_LOCAL_TEMP_HIGH: c_uint = 0x90;
pub const ADM1029_REG_REMOTE1_TEMP_HIGH: c_uint = 0x91;
pub const ADM1029_REG_REMOTE2_TEMP_HIGH: c_uint = 0x92;
pub const ADM1029_REG_LOCAL_TEMP_LOW: c_uint = 0x98;
pub const ADM1029_REG_REMOTE1_TEMP_LOW: c_uint = 0x99;
pub const ADM1029_REG_REMOTE2_TEMP_LOW: c_uint = 0x9A;
pub const ADM1029_REG_FAN1: c_uint = 0x70;
pub const ADM1029_REG_FAN2: c_uint = 0x71;
pub const ADM1029_REG_FAN1_MIN: c_uint = 0x78;
pub const ADM1029_REG_FAN2_MIN: c_uint = 0x79;
pub const ADM1029_REG_FAN1_CONFIG: c_uint = 0x68;
pub const ADM1029_REG_FAN2_CONFIG: c_uint = 0x69;

// Registers to be checked by adm1029_update_device()
    static const u8 ADM1029_REG_TEMP[] = {
    ADM1029_REG_LOCAL_TEMP,
    ADM1029_REG_REMOTE1_TEMP,
    ADM1029_REG_REMOTE2_TEMP,
    ADM1029_REG_LOCAL_TEMP_HIGH,
    ADM1029_REG_REMOTE1_TEMP_HIGH,
    ADM1029_REG_REMOTE2_TEMP_HIGH,
    ADM1029_REG_LOCAL_TEMP_LOW,
    ADM1029_REG_REMOTE1_TEMP_LOW,
    ADM1029_REG_REMOTE2_TEMP_LOW,
    };
    static const u8 ADM1029_REG_FAN[] = {
    ADM1029_REG_FAN1,
    ADM1029_REG_FAN2,
    ADM1029_REG_FAN1_MIN,
    ADM1029_REG_FAN2_MIN,
    };
    static const u8 ADM1029_REG_FAN_DIV[] = {
    ADM1029_REG_FAN1_CONFIG,
    ADM1029_REG_FAN2_CONFIG,
    };
//
// Client data (each client gets its own)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm1029_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex update_lock; / protect register access,
    pub /: *mut *mut bool valid; / false until following fields are valid,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
// registers values, signed for temperature, unsigned for other stuff
    pub temp: [i8; ARRAY_SIZE(ADM1029_REG_TEMP)],
    pub fan: [u8; ARRAY_SIZE(ADM1029_REG_FAN)],
    pub fan_div: [u8; ARRAY_SIZE(ADM1029_REG_FAN_DIV)],
}

//
// function that update the status of the chips (temperature for example)
//
    static struct adm1029_data *adm1029_update_device(struct device *dev)
    {
    struct adm1029_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    mutex_lock(&data.update_lock);
//
// Use the "cache" Luke, don't recheck values
// if there are already checked not a long time later
//
    if (time_after(jiffies, data.last_updated + HZ * 2) || !data.valid) {
    int nr;
    dev_dbg(&client.dev, "Updating adm1029 data\n");
    for (nr = 0; nr < ARRAY_SIZE(ADM1029_REG_TEMP); nr++) {
    data.temp[nr] =
    i2c_smbus_read_byte_data(client,
    ADM1029_REG_TEMP[nr]);
    }
    for (nr = 0; nr < ARRAY_SIZE(ADM1029_REG_FAN); nr++) {
    data.fan[nr] =
    i2c_smbus_read_byte_data(client,
    ADM1029_REG_FAN[nr]);
    }
    for (nr = 0; nr < ARRAY_SIZE(ADM1029_REG_FAN_DIV); nr++) {
    data.fan_div[nr] =
    i2c_smbus_read_byte_data(client,
    ADM1029_REG_FAN_DIV[nr]);
    }
    data.last_updated = jiffies;
    data.valid = true;
    }
    mutex_unlock(&data.update_lock);
    return data;
    }
//
// Sysfs stuff
//
    static ssize_t
    temp_show(struct device *dev, struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adm1029_data *data = adm1029_update_device(dev);
    return sprintf(buf, "%d\n", TEMP_FROM_REG(data.temp[attr.index]));
    }
    static ssize_t
    fan_show(struct device *dev, struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adm1029_data *data = adm1029_update_device(dev);
    u16 val;
    mutex_lock(&data.update_lock);
    if (data.fan[attr.index] == 0 ||
    (data.fan_div[attr.index] & 0xC0) == 0 ||
    data.fan[attr.index] == 255) {
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "0\n");
    }
    val = 1880 * 120 / DIV_FROM_REG(data.fan_div[attr.index])
    / data.fan[attr.index];
    mutex_unlock(&data.update_lock);
    return sprintf(buf, "%d\n", val);
    }
    static ssize_t
    fan_div_show(struct device *dev, struct device_attribute *devattr, char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    struct adm1029_data *data = adm1029_update_device(dev);
    if ((data.fan_div[attr.index] & 0xC0) == 0)
    return sprintf(buf, "0\n");
    return sprintf(buf, "%d\n", DIV_FROM_REG(data.fan_div[attr.index]));
    }
    static ssize_t fan_div_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    struct adm1029_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    u8 reg;
    long val;
    let mut ret: c_int = kstrtol(buf, 10, &val);
    if (ret < 0)
    return ret;
    mutex_lock(&data.update_lock);
// Read actual config
    reg = i2c_smbus_read_byte_data(client,
    ADM1029_REG_FAN_DIV[attr.index]);
    switch (val) {
    case 1:
    val = 1;
    break;
    case 2:
    val = 2;
    break;
    case 4:
    val = 3;
    break;
    default:
    mutex_unlock(&data.update_lock);
    dev_err(&client.dev,
    "fan_div value %ld not supported. Choose one of 1, 2 or 4!\n",
    val);
    return -EINVAL;
    }
// Update the value
    reg = (reg & 0x3F) | (val << 6);
// Update the cache
    data.fan_div[attr.index] = reg;
// Write value
    i2c_smbus_write_byte_data(client,
    ADM1029_REG_FAN_DIV[attr.index], reg);
    mutex_unlock(&data.update_lock);
    return count;
    }
// Access rights on sysfs.
    static SENSOR_DEVICE_ATTR_RO(temp1_input, temp, 0);
    static SENSOR_DEVICE_ATTR_RO(temp2_input, temp, 1);
    static SENSOR_DEVICE_ATTR_RO(temp3_input, temp, 2);
    static SENSOR_DEVICE_ATTR_RO(temp1_max, temp, 3);
    static SENSOR_DEVICE_ATTR_RO(temp2_max, temp, 4);
    static SENSOR_DEVICE_ATTR_RO(temp3_max, temp, 5);
    static SENSOR_DEVICE_ATTR_RO(temp1_min, temp, 6);
    static SENSOR_DEVICE_ATTR_RO(temp2_min, temp, 7);
    static SENSOR_DEVICE_ATTR_RO(temp3_min, temp, 8);
    static SENSOR_DEVICE_ATTR_RO(fan1_input, fan, 0);
    static SENSOR_DEVICE_ATTR_RO(fan2_input, fan, 1);
    static SENSOR_DEVICE_ATTR_RO(fan1_min, fan, 2);
    static SENSOR_DEVICE_ATTR_RO(fan2_min, fan, 3);
    static SENSOR_DEVICE_ATTR_RW(fan1_div, fan_div, 0);
    static SENSOR_DEVICE_ATTR_RW(fan2_div, fan_div, 1);
    static struct attribute *adm1029_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_min.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp2_input.dev_attr.attr,
    &sensor_dev_attr_temp2_min.dev_attr.attr,
    &sensor_dev_attr_temp2_max.dev_attr.attr,
    &sensor_dev_attr_temp3_input.dev_attr.attr,
    &sensor_dev_attr_temp3_min.dev_attr.attr,
    &sensor_dev_attr_temp3_max.dev_attr.attr,
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan1_min.dev_attr.attr,
    &sensor_dev_attr_fan2_min.dev_attr.attr,
    &sensor_dev_attr_fan1_div.dev_attr.attr,
    &sensor_dev_attr_fan2_div.dev_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(adm1029);
//
// Real code
//
// Return 0 if detection is successful, -ENODEV otherwise
    static int adm1029_detect(struct i2c_client *client,
    struct i2c_board_info *info)
    {
    struct i2c_adapter *adapter = client.adapter;
    u8 man_id, chip_id, temp_devices_installed, nb_fan_support;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -ENODEV;
//
// ADM1029 doesn't have CHIP ID, check just MAN ID
// For better detection we check also ADM1029_TEMP_DEVICES_INSTALLED,
// ADM1029_REG_NB_FAN_SUPPORT and compare it with possible values
// documented
//
    man_id = i2c_smbus_read_byte_data(client, ADM1029_REG_MAN_ID);
    chip_id = i2c_smbus_read_byte_data(client, ADM1029_REG_CHIP_ID);
    temp_devices_installed = i2c_smbus_read_byte_data(client,
    ADM1029_REG_TEMP_DEVICES_INSTALLED);
    nb_fan_support = i2c_smbus_read_byte_data(client,
    ADM1029_REG_NB_FAN_SUPPORT);
// 0x41 is Analog Devices
    if (man_id != 0x41 || (temp_devices_installed & 0xf9) != 0x01 ||
    nb_fan_support != 0x03)
    return -ENODEV;
    if ((chip_id & 0xF0) != 0x00) {
//
// There are no "official" CHIP ID, so actually
// we use Major/Minor revision for that
//
    pr_info("Unknown major revision %x, please let us know\n",
    chip_id);
    return -ENODEV;
    }
    strscpy(info.type, "adm1029", I2C_NAME_SIZE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm1029_init_client(client: *mut i2c_client) -> c_int {
    static int adm1029_init_client(struct i2c_client *client)
    {
    u8 config;
    config = i2c_smbus_read_byte_data(client, ADM1029_REG_CONFIG);
    if ((config & 0x10) == 0) {
    i2c_smbus_write_byte_data(client, ADM1029_REG_CONFIG,
    config | 0x10);
    }
// recheck config
    config = i2c_smbus_read_byte_data(client, ADM1029_REG_CONFIG);
    if ((config & 0x10) == 0) {
    dev_err(&client.dev, "Initialization failed!\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn adm1029_probe(client: *mut i2c_client) -> c_int {
    static int adm1029_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct adm1029_data *data;
    struct device *hwmon_dev;
    data = devm_kzalloc(dev, sizeof(struct adm1029_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
//
// Initialize the ADM1029 chip
// Check config register
//
    if (adm1029_init_client(client) == 0)
    return -ENODEV;
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data,
    adm1029_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id adm1029_id[] = {
    { .name = "adm1029" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adm1029_id);
    static struct i2c_driver adm1029_driver = {
    .class		= I2C_CLASS_HWMON,
    .driver = {
    .name = "adm1029",
    },
    .probe		= adm1029_probe,
    .id_table	= adm1029_id,
    .detect		= adm1029_detect,
    .address_list	= normal_i2c,
    };
    module_i2c_driver(adm1029_driver);
    MODULE_AUTHOR("Corentin LABBE <clabbe.montjoie@gmail.com>");
    MODULE_DESCRIPTION("adm1029 driver");
    MODULE_LICENSE("GPL v2");
