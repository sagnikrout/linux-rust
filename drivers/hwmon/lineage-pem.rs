//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/lineage-pem.c
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
// Driver for Lineage Compact Power Line series of power entry modules.
//
// Copyright (C) 2010, 2011 Ericsson AB.
//
// Documentation:
// http://www.lineagepower.com/oem/pdf/CPLI2C.pdf
//

//
// This driver supports various Lineage Compact Power Line DC/DC and AC/DC
// converters such as CP1800, CP2000AC, CP2000DC, CP2100DC, and others.
//
// The devices are nominally PMBus compliant. However, most standard PMBus
// commands are not supported. Specifically, all hardware monitoring and
// status reporting commands are non-standard. For this reason, a standard
// PMBus driver can not be used.
//
// All Lineage CPL devices have a built-in I2C bus master selector (PCA9541).
// To ensure device access, this driver should only be used as client driver
// to the pca9541 I2C master selector driver.
//
// Command codes
pub const PEM_OPERATION: c_uint = 0x01;
pub const PEM_CLEAR_INFO_FLAGS: c_uint = 0x03;
pub const PEM_VOUT_COMMAND: c_uint = 0x21;
pub const PEM_VOUT_OV_FAULT_LIMIT: c_uint = 0x40;
pub const PEM_READ_DATA_STRING: c_uint = 0xd0;
pub const PEM_READ_INPUT_STRING: c_uint = 0xdc;
pub const PEM_READ_FIRMWARE_REV: c_uint = 0xdd;
pub const PEM_READ_RUN_TIMER: c_uint = 0xde;
pub const PEM_FAN_HI_SPEED: c_uint = 0xdf;
pub const PEM_FAN_NORMAL_SPEED: c_uint = 0xe0;
pub const PEM_READ_FAN_SPEED: c_uint = 0xe1;
// offsets in data string
pub const PEM_DATA_STATUS_2: c_int = 0;
pub const PEM_DATA_STATUS_1: c_int = 1;
pub const PEM_DATA_ALARM_2: c_int = 2;
pub const PEM_DATA_ALARM_1: c_int = 3;
pub const PEM_DATA_VOUT_LSB: c_int = 4;
pub const PEM_DATA_VOUT_MSB: c_int = 5;
pub const PEM_DATA_CURRENT: c_int = 6;
pub const PEM_DATA_TEMP: c_int = 7;
// Virtual entries, to report constants
pub const PEM_DATA_TEMP_MAX: c_int = 10;
pub const PEM_DATA_TEMP_CRIT: c_int = 11;
// offsets in input string
pub const PEM_INPUT_VOLTAGE: c_int = 0;
pub const PEM_INPUT_POWER_LSB: c_int = 1;
pub const PEM_INPUT_POWER_MSB: c_int = 2;
// offsets in fan data
pub const PEM_FAN_ADJUSTMENT: c_int = 0;
pub const PEM_FAN_FAN1: c_int = 1;
pub const PEM_FAN_FAN2: c_int = 2;
pub const PEM_FAN_FAN3: c_int = 3;
// Status register bits

// Alarm register bits

pub const FIRMWARE_REV_LEN: c_int = 4;
pub const DATA_STRING_LEN: c_int = 9;

pub const FAN_SPEED_LEN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pem_data {
    pub client: *mut i2c_client,
    pub groups: [*const attribute_group; 4],
    pub update_lock: mutex,
    pub valid: bool,
    pub fans_supported: bool,
    pub input_length: c_int,
    pub /: *mut *mut unsigned long last_updated; / in jiffies,
    pub firmware_rev: [u8; FIRMWARE_REV_LEN],
    pub data_string: [u8; DATA_STRING_LEN],
    pub input_string: [u8; INPUT_STRING_LEN],
    pub fan_speed: [u8; FAN_SPEED_LEN],
}

    static int pem_read_block(struct i2c_client *client, u8 command, u8 *data,
    int data_len)
    {
    u8 block_buffer[I2C_SMBUS_BLOCK_MAX];
    int result;
    result = i2c_smbus_read_block_data(client, command, block_buffer);
    if (unlikely(result < 0))
    goto abort;
    if (unlikely(result == 0xff || result != data_len)) {
    result = -EIO;
    goto abort;
    }
    memcpy(data, block_buffer, data_len);
    result = 0;
    abort:
    return result;
    }
    static struct pem_data *pem_update_device(struct device *dev)
    {
    struct pem_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    struct pem_data *ret = data;
    mutex_lock(&data.update_lock);
    if (time_after(jiffies, data.last_updated + HZ) || !data.valid) {
    int result;
// Read data string
    result = pem_read_block(client, PEM_READ_DATA_STRING,
    data.data_string,
    sizeof(data.data_string));
    if (unlikely(result < 0)) {
    ret = ERR_PTR(result);
    goto abort;
    }
// Read input string
    if (data.input_length) {
    result = pem_read_block(client, PEM_READ_INPUT_STRING,
    data.input_string,
    data.input_length);
    if (unlikely(result < 0)) {
    ret = ERR_PTR(result);
    goto abort;
    }
    }
// Read fan speeds
    if (data.fans_supported) {
    result = pem_read_block(client, PEM_READ_FAN_SPEED,
    data.fan_speed,
    sizeof(data.fan_speed));
    if (unlikely(result < 0)) {
    ret = ERR_PTR(result);
    goto abort;
    }
    }
    i2c_smbus_write_byte(client, PEM_CLEAR_INFO_FLAGS);
    data.last_updated = jiffies;
    data.valid = true;
    }
    abort:
    mutex_unlock(&data.update_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pem_get_data(data: *mut u8, len: c_int, index: c_int) -> c_long {
    static long pem_get_data(u8 *data, int len, int index)
    {
    long val;
    switch (index) {
    case PEM_DATA_VOUT_LSB:
    val = (data[index] + (data[index+1] << 8)) * 5 / 2;
    break;
    case PEM_DATA_CURRENT:
    val = data[index] * 200;
    break;
    case PEM_DATA_TEMP:
    val = data[index] * 1000;
    break;
    case PEM_DATA_TEMP_MAX:
    val = 97 * 1000;	/* 97 degrees C per datasheet */
    break;
    case PEM_DATA_TEMP_CRIT:
    val = 107 * 1000;	/* 107 degrees C per datasheet */
    break;
    default:
    WARN_ON_ONCE(1);
    val = 0;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pem_get_input(data: *mut u8, len: c_int, index: c_int) -> c_long {
    static long pem_get_input(u8 *data, int len, int index)
    {
    long val;
    switch (index) {
    case PEM_INPUT_VOLTAGE:
    if (len == INPUT_STRING_LEN)
    val = (data[index] + (data[index+1] << 8) - 75) * 1000;
    else
    val = (data[index] - 75) * 1000;
    break;
    case PEM_INPUT_POWER_LSB:
    if (len == INPUT_STRING_LEN)
    index++;
    val = (data[index] + (data[index+1] << 8)) * 1000000L;
    break;
    default:
    WARN_ON_ONCE(1);
    val = 0;
    }
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pem_get_fan(data: *mut u8, len: c_int, index: c_int) -> c_long {
    static long pem_get_fan(u8 *data, int len, int index)
    {
    long val;
    switch (index) {
    case PEM_FAN_FAN1:
    case PEM_FAN_FAN2:
    case PEM_FAN_FAN3:
    val = data[index] * 100;
    break;
    default:
    WARN_ON_ONCE(1);
    val = 0;
    }
    return val;
    }
//
// Show boolean, either a fault or an alarm.
// .nr points to the register, .index is the bit mask to check
//
    static ssize_t pem_bool_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute_2 *attr = to_sensor_dev_attr_2(da);
    struct pem_data *data = pem_update_device(dev);
    u8 status;
    if (IS_ERR(data))
    return PTR_ERR(data);
    status = data.data_string[attr.nr] & attr.index;
    return sysfs_emit(buf, "%d\n", !!status);
    }
    static ssize_t pem_data_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct pem_data *data = pem_update_device(dev);
    long value;
    if (IS_ERR(data))
    return PTR_ERR(data);
    value = pem_get_data(data.data_string, sizeof(data.data_string),
    attr.index);
    return sysfs_emit(buf, "%ld\n", value);
    }
    static ssize_t pem_input_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct pem_data *data = pem_update_device(dev);
    long value;
    if (IS_ERR(data))
    return PTR_ERR(data);
    value = pem_get_input(data.input_string, sizeof(data.input_string),
    attr.index);
    return sysfs_emit(buf, "%ld\n", value);
    }
    static ssize_t pem_fan_show(struct device *dev, struct device_attribute *da,
    char *buf)
    {
    struct sensor_device_attribute *attr = to_sensor_dev_attr(da);
    struct pem_data *data = pem_update_device(dev);
    long value;
    if (IS_ERR(data))
    return PTR_ERR(data);
    value = pem_get_fan(data.fan_speed, sizeof(data.fan_speed),
    attr.index);
    return sysfs_emit(buf, "%ld\n", value);
    }
// Voltages
    static SENSOR_DEVICE_ATTR_RO(in1_input, pem_data, PEM_DATA_VOUT_LSB);
    static SENSOR_DEVICE_ATTR_2_RO(in1_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_VOUT_OUT_LIMIT);
    static SENSOR_DEVICE_ATTR_2_RO(in1_crit_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_OV_VOLT_SHUTDOWN);
    static SENSOR_DEVICE_ATTR_RO(in2_input, pem_input, PEM_INPUT_VOLTAGE);
    static SENSOR_DEVICE_ATTR_2_RO(in2_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_VIN_OUT_LIMIT | ALRM1_PRIMARY_FAULT);
// Currents
    static SENSOR_DEVICE_ATTR_RO(curr1_input, pem_data, PEM_DATA_CURRENT);
    static SENSOR_DEVICE_ATTR_2_RO(curr1_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_VIN_OVERCURRENT);
// Power
    static SENSOR_DEVICE_ATTR_RO(power1_input, pem_input, PEM_INPUT_POWER_LSB);
    static SENSOR_DEVICE_ATTR_2_RO(power1_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_POWER_LIMIT);
// Fans
    static SENSOR_DEVICE_ATTR_RO(fan1_input, pem_fan, PEM_FAN_FAN1);
    static SENSOR_DEVICE_ATTR_RO(fan2_input, pem_fan, PEM_FAN_FAN2);
    static SENSOR_DEVICE_ATTR_RO(fan3_input, pem_fan, PEM_FAN_FAN3);
    static SENSOR_DEVICE_ATTR_2_RO(fan1_alarm, pem_bool, PEM_DATA_ALARM_2,
    ALRM2_FAN_FAULT);
// Temperatures
    static SENSOR_DEVICE_ATTR_RO(temp1_input, pem_data, PEM_DATA_TEMP);
    static SENSOR_DEVICE_ATTR_RO(temp1_max, pem_data, PEM_DATA_TEMP_MAX);
    static SENSOR_DEVICE_ATTR_RO(temp1_crit, pem_data, PEM_DATA_TEMP_CRIT);
    static SENSOR_DEVICE_ATTR_2_RO(temp1_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_TEMP_WARNING);
    static SENSOR_DEVICE_ATTR_2_RO(temp1_crit_alarm, pem_bool, PEM_DATA_ALARM_1,
    ALRM1_TEMP_SHUTDOWN);
    static SENSOR_DEVICE_ATTR_2_RO(temp1_fault, pem_bool, PEM_DATA_ALARM_2,
    ALRM2_TEMP_FAULT);
    static struct attribute *pem_attributes[] = {
    &sensor_dev_attr_in1_input.dev_attr.attr,
    &sensor_dev_attr_in1_alarm.dev_attr.attr,
    &sensor_dev_attr_in1_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_in2_alarm.dev_attr.attr,
    &sensor_dev_attr_curr1_alarm.dev_attr.attr,
    &sensor_dev_attr_power1_alarm.dev_attr.attr,
    &sensor_dev_attr_fan1_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    &sensor_dev_attr_temp1_max.dev_attr.attr,
    &sensor_dev_attr_temp1_crit.dev_attr.attr,
    &sensor_dev_attr_temp1_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_crit_alarm.dev_attr.attr,
    &sensor_dev_attr_temp1_fault.dev_attr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group pem_group = {
    .attrs = pem_attributes,
    };
    static struct attribute *pem_input_attributes[] = {
    &sensor_dev_attr_in2_input.dev_attr.attr,
    &sensor_dev_attr_curr1_input.dev_attr.attr,
    &sensor_dev_attr_power1_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group pem_input_group = {
    .attrs = pem_input_attributes,
    };
    static struct attribute *pem_fan_attributes[] = {
    &sensor_dev_attr_fan1_input.dev_attr.attr,
    &sensor_dev_attr_fan2_input.dev_attr.attr,
    &sensor_dev_attr_fan3_input.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group pem_fan_group = {
    .attrs = pem_fan_attributes,
    };
#[no_mangle]
unsafe extern "C" fn pem_probe(client: *mut i2c_client) -> c_int {
    static int pem_probe(struct i2c_client *client)
    {
    struct i2c_adapter *adapter = client.adapter;
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct pem_data *data;
    int ret, idx = 0;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BLOCK_DATA
    | I2C_FUNC_SMBUS_WRITE_BYTE))
    return -ENODEV;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    mutex_init(&data.update_lock);
//
// We use the next two commands to determine if the device is really
// there.
//
    ret = pem_read_block(client, PEM_READ_FIRMWARE_REV,
    data.firmware_rev, sizeof(data.firmware_rev));
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte(client, PEM_CLEAR_INFO_FLAGS);
    if (ret < 0)
    return ret;
    dev_info(dev, "Firmware revision %d.%d.%d\n",
    data.firmware_rev[0], data.firmware_rev[1],
    data.firmware_rev[2]);
// sysfs hooks
    data.groups[idx++] = &pem_group;
//
// Check if input readings are supported.
// This is the case if we can read input data,
// and if the returned data is not all zeros.
// Note that input alarms are always supported.
//
    ret = pem_read_block(client, PEM_READ_INPUT_STRING,
    data.input_string,
    sizeof(data.input_string) - 1);
    if (!ret && (data.input_string[0] || data.input_string[1] ||
    data.input_string[2]))
    data.input_length = sizeof(data.input_string) - 1;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
// Input string is one byte longer for some devices
    ret = pem_read_block(client, PEM_READ_INPUT_STRING,
    data.input_string,
    sizeof(data.input_string));
    if (!ret && (data.input_string[0] || data.input_string[1] ||
    data.input_string[2] || data.input_string[3]))
    data.input_length = sizeof(data.input_string);
    }
    if (data.input_length)
    data.groups[idx++] = &pem_input_group;
//
// Check if fan speed readings are supported.
// This is the case if we can read fan speed data,
// and if the returned data is not all zeros.
// Note that the fan alarm is always supported.
//
    ret = pem_read_block(client, PEM_READ_FAN_SPEED,
    data.fan_speed,
    sizeof(data.fan_speed));
    if (!ret && (data.fan_speed[0] || data.fan_speed[1] ||
    data.fan_speed[2] || data.fan_speed[3])) {
    data.fans_supported = true;
    data.groups[idx++] = &pem_fan_group;
    }
    hwmon_dev = devm_hwmon_device_register_with_groups(dev, client.name,
    data, data.groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id pem_id[] = {
    { .name = "lineage_pem" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pem_id);
    static struct i2c_driver pem_driver = {
    .driver = {
    .name = "lineage_pem",
    },
    .probe = pem_probe,
    .id_table = pem_id,
    };
    module_i2c_driver(pem_driver);
    MODULE_AUTHOR("Guenter Roeck <linux@roeck-us.net>");
    MODULE_DESCRIPTION("Lineage CPL PEM hardware monitoring driver");
    MODULE_LICENSE("GPL");
