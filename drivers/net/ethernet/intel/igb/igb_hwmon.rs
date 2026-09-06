//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/igb/igb_hwmon.c
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
// Copyright(c) 2007 - 2018 Intel Corporation.

    static struct i2c_board_info i350_sensor_info = {
    I2C_BOARD_INFO("i350bb", (0Xf8 >> 1)),
    };
// hwmon callback functions
    static ssize_t igb_hwmon_show_location(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *igb_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    return sprintf(buf, "loc%u\n",
    igb_attr.sensor.location);
    }
    static ssize_t igb_hwmon_show_temp(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *igb_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    unsigned int value;
// reset the temp field
    igb_attr.hw.mac.ops.get_thermal_sensor_data(igb_attr.hw);
    value = igb_attr.sensor.temp;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
    static ssize_t igb_hwmon_show_cautionthresh(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *igb_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    let mut value: c_uint = igb_attr.sensor.caution_thresh;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
    static ssize_t igb_hwmon_show_maxopthresh(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *igb_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    let mut value: c_uint = igb_attr.sensor.max_op_thresh;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
// igb_add_hwmon_attr - Create hwmon attr table for a hwmon sysfs file.
// @ adapter: pointer to the adapter structure
// @ offset: offset in the eeprom sensor data table
// @ type: type of sensor data to display
//
// For each file we want in hwmon's sysfs interface we need a device_attribute
// This is included in our hwmon_attr struct that contains the references to
// the data structures we need to get the data to display.
//
    static int igb_add_hwmon_attr(struct igb_adapter *adapter,
    unsigned int offset, int type)
    {
    int rc;
    unsigned int n_attr;
    struct hwmon_attr *igb_attr;
    n_attr = adapter.igb_hwmon_buff.n_hwmon;
    igb_attr = &adapter.igb_hwmon_buff.hwmon_list[n_attr];
    switch (type) {
    case IGB_HWMON_TYPE_LOC:
    igb_attr.dev_attr.show = igb_hwmon_show_location;
    snprintf(igb_attr.name, sizeof(igb_attr.name),
    "temp%u_label", offset + 1);
    break;
    case IGB_HWMON_TYPE_TEMP:
    igb_attr.dev_attr.show = igb_hwmon_show_temp;
    snprintf(igb_attr.name, sizeof(igb_attr.name),
    "temp%u_input", offset + 1);
    break;
    case IGB_HWMON_TYPE_CAUTION:
    igb_attr.dev_attr.show = igb_hwmon_show_cautionthresh;
    snprintf(igb_attr.name, sizeof(igb_attr.name),
    "temp%u_max", offset + 1);
    break;
    case IGB_HWMON_TYPE_MAX:
    igb_attr.dev_attr.show = igb_hwmon_show_maxopthresh;
    snprintf(igb_attr.name, sizeof(igb_attr.name),
    "temp%u_crit", offset + 1);
    break;
    default:
    rc = -EPERM;
    return rc;
    }
// These always the same regardless of type
    igb_attr.sensor =
    &adapter.hw.mac.thermal_sensor_data.sensor[offset];
    igb_attr.hw = &adapter.hw;
    igb_attr.dev_attr.store = core::ptr::null_mut();
    igb_attr.dev_attr.attr.mode = 0444;
    igb_attr.dev_attr.attr.name = igb_attr.name;
    sysfs_attr_init(&igb_attr.dev_attr.attr);
    adapter.igb_hwmon_buff.attrs[n_attr] = &igb_attr.dev_attr.attr;
    ++adapter.igb_hwmon_buff.n_hwmon;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn igb_sysfs_del_adapter(adapter: *mut igb_adapter) {
    static void igb_sysfs_del_adapter(struct igb_adapter *adapter)
    {
    }
// called from igb_main.c
#[no_mangle]
pub unsafe extern "C" fn igb_sysfs_exit(adapter: *mut igb_adapter) {
    void igb_sysfs_exit(struct igb_adapter *adapter)
    {
    igb_sysfs_del_adapter(adapter);
    }
// called from igb_main.c
#[no_mangle]
pub unsafe extern "C" fn igb_sysfs_init(adapter: *mut igb_adapter) -> c_int {
    int igb_sysfs_init(struct igb_adapter *adapter)
    {
    struct hwmon_buff *igb_hwmon;
    struct i2c_client *client;
    struct device *hwmon_dev;
    unsigned int i;
    let mut rc: c_int = 0;
// If this method isn't defined we don't support thermals
    if (adapter.hw.mac.ops.init_thermal_sensor_thresh == core::ptr::null_mut())
    goto exit;
// Don't create thermal hwmon interface if no sensors present
    rc = (adapter.hw.mac.ops.init_thermal_sensor_thresh(&adapter.hw));
    if (rc)
    goto exit;
    igb_hwmon = devm_kzalloc(&adapter.pdev.dev, sizeof(*igb_hwmon),
    GFP_KERNEL);
    if (!igb_hwmon) {
    rc = -ENOMEM;
    goto exit;
    }
    adapter.igb_hwmon_buff = igb_hwmon;
    for (i = 0; i < E1000_MAX_SENSORS; i++) {
// Only create hwmon sysfs entries for sensors that have
// meaningful data.
//
    if (adapter.hw.mac.thermal_sensor_data.sensor[i].location == 0)
    continue;
// Bail if any hwmon attr struct fails to initialize
    rc = igb_add_hwmon_attr(adapter, i, IGB_HWMON_TYPE_CAUTION);
    if (rc)
    goto exit;
    rc = igb_add_hwmon_attr(adapter, i, IGB_HWMON_TYPE_LOC);
    if (rc)
    goto exit;
    rc = igb_add_hwmon_attr(adapter, i, IGB_HWMON_TYPE_TEMP);
    if (rc)
    goto exit;
    rc = igb_add_hwmon_attr(adapter, i, IGB_HWMON_TYPE_MAX);
    if (rc)
    goto exit;
    }
// init i2c_client
    client = i2c_new_client_device(&adapter.i2c_adap, &i350_sensor_info);
    if (IS_ERR(client)) {
    dev_info(&adapter.pdev.dev,
    "Failed to create new i2c device.\n");
    rc = PTR_ERR(client);
    goto exit;
    }
    adapter.i2c_client = client;
    igb_hwmon.groups[0] = &igb_hwmon.group;
    igb_hwmon.group.attrs = igb_hwmon.attrs;
    hwmon_dev = devm_hwmon_device_register_with_groups(&adapter.pdev.dev,
    client.name,
    igb_hwmon,
    igb_hwmon.groups);
    if (IS_ERR(hwmon_dev)) {
    rc = PTR_ERR(hwmon_dev);
    goto err;
    }
    goto exit;
    err:
    igb_sysfs_del_adapter(adapter);
    exit:
    return rc;
    }
