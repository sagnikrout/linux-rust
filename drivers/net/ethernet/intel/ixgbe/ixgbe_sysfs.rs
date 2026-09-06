//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_sysfs.c
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
// Copyright(c) 1999 - 2018 Intel Corporation.

// hwmon callback functions
    static ssize_t ixgbe_hwmon_show_location(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *ixgbe_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    return sprintf(buf, "loc%u\n",
    ixgbe_attr.sensor.location);
    }
    static ssize_t ixgbe_hwmon_show_temp(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *ixgbe_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    unsigned int value;
// reset the temp field
    ixgbe_attr.hw.mac.ops.get_thermal_sensor_data(ixgbe_attr.hw);
    value = ixgbe_attr.sensor.temp;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
    static ssize_t ixgbe_hwmon_show_cautionthresh(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *ixgbe_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    let mut value: c_uint = ixgbe_attr.sensor.caution_thresh;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
    static ssize_t ixgbe_hwmon_show_maxopthresh(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hwmon_attr *ixgbe_attr = container_of(attr, struct hwmon_attr,
    dev_attr);
    let mut value: c_uint = ixgbe_attr.sensor.max_op_thresh;
// display millidegree
    value *= 1000;
    return sprintf(buf, "%u\n", value);
    }
//
// ixgbe_add_hwmon_attr - Create hwmon attr table for a hwmon sysfs file.
// @adapter: pointer to the adapter structure
// @offset: offset in the eeprom sensor data table
// @type: type of sensor data to display
//
// For each file we want in hwmon's sysfs interface we need a device_attribute
// This is included in our hwmon_attr struct that contains the references to
// the data structures we need to get the data to display.
//
    static int ixgbe_add_hwmon_attr(struct ixgbe_adapter *adapter,
    unsigned int offset, int type) {
    int rc;
    unsigned int n_attr;
    struct hwmon_attr *ixgbe_attr;
    n_attr = adapter.ixgbe_hwmon_buff.n_hwmon;
    ixgbe_attr = &adapter.ixgbe_hwmon_buff.hwmon_list[n_attr];
    switch (type) {
    case IXGBE_HWMON_TYPE_LOC:
    ixgbe_attr.dev_attr.show = ixgbe_hwmon_show_location;
    snprintf(ixgbe_attr.name, sizeof(ixgbe_attr.name),
    "temp%u_label", offset + 1);
    break;
    case IXGBE_HWMON_TYPE_TEMP:
    ixgbe_attr.dev_attr.show = ixgbe_hwmon_show_temp;
    snprintf(ixgbe_attr.name, sizeof(ixgbe_attr.name),
    "temp%u_input", offset + 1);
    break;
    case IXGBE_HWMON_TYPE_CAUTION:
    ixgbe_attr.dev_attr.show = ixgbe_hwmon_show_cautionthresh;
    snprintf(ixgbe_attr.name, sizeof(ixgbe_attr.name),
    "temp%u_max", offset + 1);
    break;
    case IXGBE_HWMON_TYPE_MAX:
    ixgbe_attr.dev_attr.show = ixgbe_hwmon_show_maxopthresh;
    snprintf(ixgbe_attr.name, sizeof(ixgbe_attr.name),
    "temp%u_crit", offset + 1);
    break;
    default:
    rc = -EPERM;
    return rc;
    }
// These always the same regardless of type
    ixgbe_attr.sensor =
    &adapter.hw.mac.thermal_sensor_data.sensor[offset];
    ixgbe_attr.hw = &adapter.hw;
    ixgbe_attr.dev_attr.store = core::ptr::null_mut();
    ixgbe_attr.dev_attr.attr.mode = 0444;
    ixgbe_attr.dev_attr.attr.name = ixgbe_attr.name;
    sysfs_attr_init(&ixgbe_attr.dev_attr.attr);
    adapter.ixgbe_hwmon_buff.attrs[n_attr] = &ixgbe_attr.dev_attr.attr;
    ++adapter.ixgbe_hwmon_buff.n_hwmon;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ixgbe_sysfs_del_adapter(adapter: *mut ixgbe_adapter) {
    static void ixgbe_sysfs_del_adapter(struct ixgbe_adapter *adapter)
    {
    }
// called from ixgbe_main.c
#[no_mangle]
pub unsafe extern "C" fn ixgbe_sysfs_exit(adapter: *mut ixgbe_adapter) {
    void ixgbe_sysfs_exit(struct ixgbe_adapter *adapter)
    {
    ixgbe_sysfs_del_adapter(adapter);
    }
// called from ixgbe_main.c
#[no_mangle]
pub unsafe extern "C" fn ixgbe_sysfs_init(adapter: *mut ixgbe_adapter) -> c_int {
    int ixgbe_sysfs_init(struct ixgbe_adapter *adapter)
    {
    struct hwmon_buff *ixgbe_hwmon;
    struct device *hwmon_dev;
    unsigned int i;
    let mut rc: c_int = 0;
// If this method isn't defined we don't support thermals
    if (adapter.hw.mac.ops.init_thermal_sensor_thresh == core::ptr::null_mut()) {
    goto exit;
    }
// Don't create thermal hwmon interface if no sensors present
    if (adapter.hw.mac.ops.init_thermal_sensor_thresh(&adapter.hw))
    goto exit;
    ixgbe_hwmon = devm_kzalloc(&adapter.pdev.dev, sizeof(*ixgbe_hwmon),
    GFP_KERNEL);
    if (ixgbe_hwmon == core::ptr::null_mut()) {
    rc = -ENOMEM;
    goto exit;
    }
    adapter.ixgbe_hwmon_buff = ixgbe_hwmon;
    for (i = 0; i < IXGBE_MAX_SENSORS; i++) {
//
// Only create hwmon sysfs entries for sensors that have
// meaningful data for.
//
    if (adapter.hw.mac.thermal_sensor_data.sensor[i].location == 0)
    continue;
// Bail if any hwmon attr struct fails to initialize
    rc = ixgbe_add_hwmon_attr(adapter, i, IXGBE_HWMON_TYPE_CAUTION);
    if (rc)
    goto exit;
    rc = ixgbe_add_hwmon_attr(adapter, i, IXGBE_HWMON_TYPE_LOC);
    if (rc)
    goto exit;
    rc = ixgbe_add_hwmon_attr(adapter, i, IXGBE_HWMON_TYPE_TEMP);
    if (rc)
    goto exit;
    rc = ixgbe_add_hwmon_attr(adapter, i, IXGBE_HWMON_TYPE_MAX);
    if (rc)
    goto exit;
    }
    ixgbe_hwmon.groups[0] = &ixgbe_hwmon.group;
    ixgbe_hwmon.group.attrs = ixgbe_hwmon.attrs;
    hwmon_dev = devm_hwmon_device_register_with_groups(&adapter.pdev.dev,
    "ixgbe",
    ixgbe_hwmon,
    ixgbe_hwmon.groups);
    if (IS_ERR(hwmon_dev))
    rc = PTR_ERR(hwmon_dev);
    exit:
    return rc;
    }
