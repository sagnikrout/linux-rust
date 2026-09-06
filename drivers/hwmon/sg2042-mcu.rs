//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/sg2042-mcu.c
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
// Copyright (c) 2024 Inochi Amaoto <inochiama@outlook.com>
//
// Sophgo power control mcu for SG2042
//

// fixed MCU registers
pub const REG_BOARD_TYPE: c_uint = 0x00;
pub const REG_MCU_FIRMWARE_VERSION: c_uint = 0x01;
pub const REG_PCB_VERSION: c_uint = 0x02;
pub const REG_PWR_CTRL: c_uint = 0x03;
pub const REG_SOC_TEMP: c_uint = 0x04;
pub const REG_BOARD_TEMP: c_uint = 0x05;
pub const REG_RST_COUNT: c_uint = 0x0a;
pub const REG_UPTIME: c_uint = 0x0b;
pub const REG_RESET_REASON: c_uint = 0x0d;
pub const REG_MCU_TYPE: c_uint = 0x18;
pub const REG_REPOWER_POLICY: c_uint = 0x65;
pub const REG_CRITICAL_TEMP: c_uint = 0x66;
pub const REG_REPOWER_TEMP: c_uint = 0x67;
pub const REPOWER_POLICY_REBOOT: c_int = 1;
pub const REPOWER_POLICY_KEEP_OFF: c_int = 2;
pub const MCU_POWER_MAX: c_uint = 0xff;

    static int _name##_show(struct seq_file *seqf,			\
    void *unused)			\
    {								\
    struct sg2042_mcu_data *mcu = seqf.private;		\
    int ret;						\
    ret = i2c_smbus_read_byte_data(mcu.client, (_reg));	\
    if (ret < 0)						\
    return ret;					\
    seq_printf(seqf, _format "\n", ret);			\
    return 0;						\
    }								\
    DEFINE_SHOW_ATTRIBUTE(_name)					\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sg2042_mcu_data {
    pub client: *mut i2c_client,
    pub mutex: mutex,
}

    static ssize_t reset_count_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    int ret;
    ret = i2c_smbus_read_byte_data(mcu.client, REG_RST_COUNT);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%d\n", ret);
    }
    static ssize_t uptime_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    u8 time_val[2];
    int ret;
    ret = i2c_smbus_read_i2c_block_data(mcu.client, REG_UPTIME,
    sizeof(time_val), time_val);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%d\n",
    (time_val[0]) | (time_val[1] << 8));
    }
    static ssize_t reset_reason_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    int ret;
    ret = i2c_smbus_read_byte_data(mcu.client, REG_RESET_REASON);
    if (ret < 0)
    return ret;
    return sprintf(buf, "0x%02x\n", ret);
    }
    static ssize_t repower_policy_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    int ret;
    const char *action;
    ret = i2c_smbus_read_byte_data(mcu.client, REG_REPOWER_POLICY);
    if (ret < 0)
    return ret;
    if (ret == REPOWER_POLICY_REBOOT)
    action = "repower";
#[no_mangle]
pub unsafe extern "C" fn if(REPOWER_POLICY_KEEP_OFF: ret ==) -> else {
    else if (ret == REPOWER_POLICY_KEEP_OFF)
    action = "keep";
    else
    action = "unknown";
    return sprintf(buf, "%s\n", action);
    }
    static ssize_t repower_policy_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    u8 value;
    int ret;
    if (sysfs_streq("repower", buf))
    value = REPOWER_POLICY_REBOOT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq("keep", _arg: buf)) -> else {
    else if (sysfs_streq("keep", buf))
    value = REPOWER_POLICY_KEEP_OFF;
    else
    return -EINVAL;
    ret = i2c_smbus_write_byte_data(mcu.client,
    REG_REPOWER_POLICY, value);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR_RO(reset_count);
    static DEVICE_ATTR_RO(uptime);
    static DEVICE_ATTR_RO(reset_reason);
    static DEVICE_ATTR_RW(repower_policy);
    DEFINE_MCU_DEBUG_ATTR(firmware_version, REG_MCU_FIRMWARE_VERSION, "0x%02x");
    DEFINE_MCU_DEBUG_ATTR(pcb_version, REG_PCB_VERSION, "0x%02x");
    DEFINE_MCU_DEBUG_ATTR(board_type, REG_BOARD_TYPE, "0x%02x");
    DEFINE_MCU_DEBUG_ATTR(mcu_type, REG_MCU_TYPE, "%d");
    static struct attribute *sg2042_mcu_attrs[] = {
    &dev_attr_reset_count.attr,
    &dev_attr_uptime.attr,
    &dev_attr_reset_reason.attr,
    &dev_attr_repower_policy.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group sg2042_mcu_attr_group = {
    .attrs	= sg2042_mcu_attrs,
    };
    static const struct attribute_group *sg2042_mcu_groups[] = {
    &sg2042_mcu_attr_group,
    core::ptr::null_mut()
    };
    static const struct hwmon_channel_info * const sg2042_mcu_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_CRIT |
    HWMON_T_CRIT_HYST,
    HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static int sg2042_mcu_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    int tmp;
    u8 reg;
    switch (attr) {
    case hwmon_temp_input:
    reg = channel ? REG_BOARD_TEMP : REG_SOC_TEMP;
    break;
    case hwmon_temp_crit:
    reg = REG_CRITICAL_TEMP;
    break;
    case hwmon_temp_crit_hyst:
    reg = REG_REPOWER_TEMP;
    break;
    default:
    return -EOPNOTSUPP;
    }
    tmp = i2c_smbus_read_byte_data(mcu.client, reg);
    if (tmp < 0)
    return tmp;
// val = tmp * 1000;
    return 0;
    }
    static int sg2042_mcu_write(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct sg2042_mcu_data *mcu = dev_get_drvdata(dev);
    let mut temp: c_int = val / 1000;
    int hyst_temp, crit_temp;
    u8 reg;
    temp = clamp_val(temp, 0, MCU_POWER_MAX);
    guard(mutex)(&mcu.mutex);
    switch (attr) {
    case hwmon_temp_crit:
    hyst_temp = i2c_smbus_read_byte_data(mcu.client,
    REG_REPOWER_TEMP);
    if (hyst_temp < 0)
    return hyst_temp;
    crit_temp = temp;
    reg = REG_CRITICAL_TEMP;
    break;
    case hwmon_temp_crit_hyst:
    crit_temp = i2c_smbus_read_byte_data(mcu.client,
    REG_CRITICAL_TEMP);
    if (crit_temp < 0)
    return crit_temp;
    hyst_temp = temp;
    reg = REG_REPOWER_TEMP;
    break;
    default:
    return -EOPNOTSUPP;
    }
//
// ensure hyst_temp is smaller to avoid MCU from
// keeping triggering repower event.
//
    if (crit_temp < hyst_temp)
    return -EINVAL;
    return i2c_smbus_write_byte_data(mcu.client, reg, temp);
    }
    static umode_t sg2042_mcu_is_visible(const void *_data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    return 0444;
    case hwmon_temp_crit:
    case hwmon_temp_crit_hyst:
    if (channel == 0)
    return 0644;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_ops sg2042_mcu_ops = {
    .is_visible = sg2042_mcu_is_visible,
    .read = sg2042_mcu_read,
    .write = sg2042_mcu_write,
    };
    static const struct hwmon_chip_info sg2042_mcu_chip_info = {
    .ops = &sg2042_mcu_ops,
    .info = sg2042_mcu_info,
    };
#[no_mangle]
unsafe extern "C" fn sg2042_mcu_debugfs_init(mcu: *mut sg2042_mcu_data) {
    static void sg2042_mcu_debugfs_init(struct sg2042_mcu_data *mcu)
    {
    debugfs_create_file("firmware_version", 0444, mcu.client.debugfs,
    mcu, &firmware_version_fops);
    debugfs_create_file("pcb_version", 0444, mcu.client.debugfs, mcu,
    &pcb_version_fops);
    debugfs_create_file("mcu_type", 0444, mcu.client.debugfs, mcu,
    &mcu_type_fops);
    debugfs_create_file("board_type", 0444, mcu.client.debugfs, mcu,
    &board_type_fops);
    }
#[no_mangle]
unsafe extern "C" fn sg2042_mcu_i2c_probe(client: *mut i2c_client) -> c_int {
    static int sg2042_mcu_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct sg2042_mcu_data *mcu;
    struct device *hwmon_dev;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_BLOCK_DATA))
    return -ENODEV;
    mcu = devm_kmalloc(dev, sizeof(*mcu), GFP_KERNEL);
    if (!mcu)
    return -ENOMEM;
    mutex_init(&mcu.mutex);
    mcu.client = client;
    i2c_set_clientdata(client, mcu);
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "sg2042_mcu",
    mcu,
    &sg2042_mcu_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return PTR_ERR(hwmon_dev);
    sg2042_mcu_debugfs_init(mcu);
    return 0;
    }
    static const struct i2c_device_id sg2042_mcu_id[] = {
    { .name = "sg2042-hwmon-mcu" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sg2042_mcu_id);
    static const struct of_device_id sg2042_mcu_of_id[] = {
    { .compatible = "sophgo,sg2042-hwmon-mcu" },
    {},
    };
    MODULE_DEVICE_TABLE(of, sg2042_mcu_of_id);
    static struct i2c_driver sg2042_mcu_driver = {
    .driver = {
    .name = "sg2042-mcu",
    .of_match_table = sg2042_mcu_of_id,
    .dev_groups = sg2042_mcu_groups,
    },
    .probe = sg2042_mcu_i2c_probe,
    .id_table = sg2042_mcu_id,
    };
    module_i2c_driver(sg2042_mcu_driver);
    MODULE_AUTHOR("Inochi Amaoto <inochiama@outlook.com>");
    MODULE_DESCRIPTION("MCU I2C driver for SG2042 soc platform");
    MODULE_LICENSE("GPL");
