//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/pmbus/inspur-ipsps.c
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
// Copyright 2019 Inspur Corp.
//

pub const IPSPS_REG_VENDOR_ID: c_uint = 0x99;
pub const IPSPS_REG_MODEL: c_uint = 0x9A;
pub const IPSPS_REG_FW_VERSION: c_uint = 0x9B;
pub const IPSPS_REG_PN: c_uint = 0x9C;
pub const IPSPS_REG_SN: c_uint = 0x9E;
pub const IPSPS_REG_HW_VERSION: c_uint = 0xB0;
pub const IPSPS_REG_MODE: c_uint = 0xFC;
pub const MODE_ACTIVE: c_uint = 0x55;
pub const MODE_STANDBY: c_uint = 0x0E;
pub const MODE_REDUNDANCY: c_uint = 0x00;

    enum ipsps_index {
    vendor,
    model,
    fw_version,
    part_number,
    serial_number,
    hw_version,
    mode,
    num_regs,
    };
    static const u8 ipsps_regs[num_regs] = {
    [vendor] = IPSPS_REG_VENDOR_ID,
    [model] = IPSPS_REG_MODEL,
    [fw_version] = IPSPS_REG_FW_VERSION,
    [part_number] = IPSPS_REG_PN,
    [serial_number] = IPSPS_REG_SN,
    [hw_version] = IPSPS_REG_HW_VERSION,
    [mode] = IPSPS_REG_MODE,
    };
    static ssize_t ipsps_string_show(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    u8 reg;
    int rc;
    char *p;
    char data[I2C_SMBUS_BLOCK_MAX + 1];
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    reg = ipsps_regs[attr.index];
    rc = i2c_smbus_read_block_data(client, reg, data);
    if (rc < 0)
    return rc;
// filled with printable characters, ending with #
    p = memscan(data, '#', rc);
// p = '\0';
    return sysfs_emit(buf, "%s\n", data);
    }
    static ssize_t ipsps_fw_version_show(struct device *dev,
    struct device_attribute *devattr,
    char *buf)
    {
    u8 reg;
    int rc;
    u8 data[I2C_SMBUS_BLOCK_MAX] = { 0 };
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    reg = ipsps_regs[attr.index];
    rc = i2c_smbus_read_block_data(client, reg, data);
    if (rc < 0)
    return rc;
    if (rc != 6)
    return -EPROTO;
    return sysfs_emit(buf, "%u.%02u%u-%u.%02u\n",
    data[1], data[2]/* < 100 */, data[3]/*< 10*/,
    data[4], data[5]/* < 100 */);
    }
    static ssize_t ipsps_mode_show(struct device *dev,
    struct device_attribute *devattr, char *buf)
    {
    u8 reg;
    int rc;
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    reg = ipsps_regs[attr.index];
    rc = i2c_smbus_read_byte_data(client, reg);
    if (rc < 0)
    return rc;
    switch (rc) {
    case MODE_ACTIVE:
    return sysfs_emit(buf, "[%s] %s %s\n",
    MODE_ACTIVE_STRING,
    MODE_STANDBY_STRING, MODE_REDUNDANCY_STRING);
    case MODE_STANDBY:
    return sysfs_emit(buf, "%s [%s] %s\n",
    MODE_ACTIVE_STRING,
    MODE_STANDBY_STRING, MODE_REDUNDANCY_STRING);
    case MODE_REDUNDANCY:
    return sysfs_emit(buf, "%s %s [%s]\n",
    MODE_ACTIVE_STRING,
    MODE_STANDBY_STRING, MODE_REDUNDANCY_STRING);
    default:
    return sysfs_emit(buf, "unspecified\n");
    }
    }
    static ssize_t ipsps_mode_store(struct device *dev,
    struct device_attribute *devattr,
    const char *buf, size_t count)
    {
    u8 reg;
    int rc;
    struct i2c_client *client = to_i2c_client(dev.parent);
    struct sensor_device_attribute *attr = to_sensor_dev_attr(devattr);
    reg = ipsps_regs[attr.index];
    if (sysfs_streq(MODE_STANDBY_STRING, buf)) {
    rc = i2c_smbus_write_byte_data(client, reg,
    MODE_STANDBY);
    if (rc < 0)
    return rc;
    return count;
    } else if (sysfs_streq(MODE_ACTIVE_STRING, buf)) {
    rc = i2c_smbus_write_byte_data(client, reg,
    MODE_ACTIVE);
    if (rc < 0)
    return rc;
    return count;
    }
    return -EINVAL;
    }
    static SENSOR_DEVICE_ATTR_RO(vendor, ipsps_string, vendor);
    static SENSOR_DEVICE_ATTR_RO(model, ipsps_string, model);
    static SENSOR_DEVICE_ATTR_RO(part_number, ipsps_string, part_number);
    static SENSOR_DEVICE_ATTR_RO(serial_number, ipsps_string, serial_number);
    static SENSOR_DEVICE_ATTR_RO(hw_version, ipsps_string, hw_version);
    static SENSOR_DEVICE_ATTR_RO(fw_version, ipsps_fw_version, fw_version);
    static SENSOR_DEVICE_ATTR_RW(mode, ipsps_mode, mode);
    static struct attribute *ipsps_attrs[] = {
    &sensor_dev_attr_vendor.dev_attr.attr,
    &sensor_dev_attr_model.dev_attr.attr,
    &sensor_dev_attr_part_number.dev_attr.attr,
    &sensor_dev_attr_serial_number.dev_attr.attr,
    &sensor_dev_attr_hw_version.dev_attr.attr,
    &sensor_dev_attr_fw_version.dev_attr.attr,
    &sensor_dev_attr_mode.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(ipsps);
    static struct pmbus_driver_info ipsps_info = {
    .pages = 1,
    .func[0] = PMBUS_HAVE_VIN | PMBUS_HAVE_VOUT | PMBUS_HAVE_IOUT |
    PMBUS_HAVE_IIN | PMBUS_HAVE_POUT | PMBUS_HAVE_PIN |
    PMBUS_HAVE_FAN12 | PMBUS_HAVE_TEMP | PMBUS_HAVE_TEMP2 |
    PMBUS_HAVE_TEMP3 | PMBUS_HAVE_STATUS_VOUT |
    PMBUS_HAVE_STATUS_IOUT | PMBUS_HAVE_STATUS_INPUT |
    PMBUS_HAVE_STATUS_TEMP | PMBUS_HAVE_STATUS_FAN12,
    .groups = ipsps_groups,
    };
    static struct pmbus_platform_data ipsps_pdata = {
    .flags = PMBUS_SKIP_STATUS_CHECK,
    };
#[no_mangle]
unsafe extern "C" fn ipsps_probe(client: *mut i2c_client) -> c_int {
    static int ipsps_probe(struct i2c_client *client)
    {
    client.dev.platform_data = &ipsps_pdata;
    return pmbus_do_probe(client, &ipsps_info);
    }
    static const struct i2c_device_id ipsps_id[] = {
    { .name = "ipsps1" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ipsps_id);

    static const struct of_device_id ipsps_of_match[] = {
    { .compatible = "inspur,ipsps1" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ipsps_of_match);

    static struct i2c_driver ipsps_driver = {
    .driver = {
    .name = "inspur-ipsps",
    .of_match_table = of_match_ptr(ipsps_of_match),
    },
    .probe = ipsps_probe,
    .id_table = ipsps_id,
    };
    module_i2c_driver(ipsps_driver);
    MODULE_AUTHOR("John Wang");
    MODULE_DESCRIPTION("PMBus driver for Inspur Power System power supplies");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("PMBUS");
