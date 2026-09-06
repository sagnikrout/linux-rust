//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/occ/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright IBM Corp 2019

// OCC status register

// OCC extended status register

    static ssize_t occ_active_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    int rc;
    bool active;
    struct occ *occ = dev_get_drvdata(dev);
    rc = kstrtobool(buf, &active);
    if (rc)
    return rc;
    rc = occ_active(occ, active);
    if (rc)
    return rc;
    return count;
    }
    static ssize_t occ_sysfs_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int rc;
    let mut val: c_int = 0;
    struct occ *occ = dev_get_drvdata(dev);
    struct occ_poll_response_header *header;
    struct sensor_device_attribute *sattr = to_sensor_dev_attr(attr);
    if (occ.active) {
    rc = occ_update_response(occ);
    if (rc)
    return rc;
    header = (struct occ_poll_response_header *)occ.resp.data;
    switch (sattr.index) {
    case 0:
    val = !!(header.status & OCC_STAT_MASTER);
    break;
    case 1:
    val = 1;
    break;
    case 2:
    val = !!(header.ext_status & OCC_EXT_STAT_DVFS_OT);
    break;
    case 3:
    val = !!(header.ext_status & OCC_EXT_STAT_DVFS_POWER);
    break;
    case 4:
    val = !!(header.ext_status &
    OCC_EXT_STAT_MEM_THROTTLE);
    break;
    case 5:
    val = !!(header.ext_status & OCC_EXT_STAT_QUICK_DROP);
    break;
    case 6:
    val = header.occ_state;
    break;
    case 7:
    if (header.status & OCC_STAT_MASTER)
    val = hweight8(header.occs_present);
    else
    val = 1;
    break;
    case 8:
    val = header.ips_status;
    break;
    case 9:
    val = header.mode;
    break;
    case 10:
    val = !!(header.ext_status & OCC_EXT_STAT_DVFS_VDD);
    break;
    case 11:
    val = header.ext_status & OCC_EXT_STAT_GPU_THROTTLE;
    break;
    default:
    return -EINVAL;
    }
    } else {
    if (sattr.index == 1)
    val = 0;
#[no_mangle]
pub unsafe extern "C" fn if(11: sattr->index <=) -> else {
    else if (sattr.index <= 11)
    val = -ENODATA;
    else
    return -EINVAL;
    }
    return sysfs_emit(buf, "%d\n", val);
    }
    static ssize_t occ_error_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct occ *occ = dev_get_drvdata(dev);
    occ_update_response(occ);
    return sysfs_emit(buf, "%d\n", occ.error);
    }
    static SENSOR_DEVICE_ATTR(occ_master, 0444, occ_sysfs_show, core::ptr::null_mut(), 0);
    static SENSOR_DEVICE_ATTR(occ_active, 0644, occ_sysfs_show, occ_active_store,
    1);
    static SENSOR_DEVICE_ATTR(occ_dvfs_overtemp, 0444, occ_sysfs_show, core::ptr::null_mut(), 2);
    static SENSOR_DEVICE_ATTR(occ_dvfs_power, 0444, occ_sysfs_show, core::ptr::null_mut(), 3);
    static SENSOR_DEVICE_ATTR(occ_mem_throttle, 0444, occ_sysfs_show, core::ptr::null_mut(), 4);
    static SENSOR_DEVICE_ATTR(occ_quick_pwr_drop, 0444, occ_sysfs_show, core::ptr::null_mut(), 5);
    static SENSOR_DEVICE_ATTR(occ_state, 0444, occ_sysfs_show, core::ptr::null_mut(), 6);
    static SENSOR_DEVICE_ATTR(occs_present, 0444, occ_sysfs_show, core::ptr::null_mut(), 7);
    static SENSOR_DEVICE_ATTR(occ_ips_status, 0444, occ_sysfs_show, core::ptr::null_mut(), 8);
    static SENSOR_DEVICE_ATTR(occ_mode, 0444, occ_sysfs_show, core::ptr::null_mut(), 9);
    static SENSOR_DEVICE_ATTR(occ_dvfs_vdd, 0444, occ_sysfs_show, core::ptr::null_mut(), 10);
    static SENSOR_DEVICE_ATTR(occ_gpu_throttle, 0444, occ_sysfs_show, core::ptr::null_mut(), 11);
    static DEVICE_ATTR_RO(occ_error);
    static struct attribute *occ_attributes[] = {
    &sensor_dev_attr_occ_master.dev_attr.attr,
    &sensor_dev_attr_occ_active.dev_attr.attr,
    &sensor_dev_attr_occ_dvfs_overtemp.dev_attr.attr,
    &sensor_dev_attr_occ_dvfs_power.dev_attr.attr,
    &sensor_dev_attr_occ_mem_throttle.dev_attr.attr,
    &sensor_dev_attr_occ_quick_pwr_drop.dev_attr.attr,
    &sensor_dev_attr_occ_state.dev_attr.attr,
    &sensor_dev_attr_occs_present.dev_attr.attr,
    &sensor_dev_attr_occ_ips_status.dev_attr.attr,
    &sensor_dev_attr_occ_mode.dev_attr.attr,
    &sensor_dev_attr_occ_dvfs_vdd.dev_attr.attr,
    &sensor_dev_attr_occ_gpu_throttle.dev_attr.attr,
    &dev_attr_occ_error.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group occ_sysfs = {
    .attrs = occ_attributes,
    };
#[no_mangle]
pub unsafe extern "C" fn occ_sysfs_poll_done(occ: *mut occ) {
    void occ_sysfs_poll_done(struct occ *occ)
    {
    const char *name;
    struct occ_poll_response_header *header =
    (struct occ_poll_response_header *)occ.resp.data;
//
// On the first poll response, we haven't yet created the sysfs
// attributes, so don't make any notify calls.
//
    if (!occ.active)
    goto done;
    if ((header.status & OCC_STAT_MASTER) !=
    (occ.prev_stat & OCC_STAT_MASTER)) {
    name = sensor_dev_attr_occ_master.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_DVFS_OT) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_DVFS_OT)) {
    name = sensor_dev_attr_occ_dvfs_overtemp.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_DVFS_POWER) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_DVFS_POWER)) {
    name = sensor_dev_attr_occ_dvfs_power.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_MEM_THROTTLE) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_MEM_THROTTLE)) {
    name = sensor_dev_attr_occ_mem_throttle.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_QUICK_DROP) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_QUICK_DROP)) {
    name = sensor_dev_attr_occ_quick_pwr_drop.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_DVFS_VDD) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_DVFS_VDD)) {
    name = sensor_dev_attr_occ_dvfs_vdd.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.ext_status & OCC_EXT_STAT_GPU_THROTTLE) !=
    (occ.prev_ext_stat & OCC_EXT_STAT_GPU_THROTTLE)) {
    name = sensor_dev_attr_occ_gpu_throttle.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if ((header.status & OCC_STAT_MASTER) &&
    header.occs_present != occ.prev_occs_present) {
    name = sensor_dev_attr_occs_present.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if (header.ips_status != occ.prev_ips_status) {
    name = sensor_dev_attr_occ_ips_status.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if (header.mode != occ.prev_mode) {
    name = sensor_dev_attr_occ_mode.dev_attr.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
    if (occ.error && occ.error != occ.prev_error) {
    name = dev_attr_occ_error.attr.name;
    sysfs_notify(&occ.bus_dev.kobj, core::ptr::null_mut(), name);
    }
// no notifications for OCC state; doesn't indicate error condition
    done:
    occ.prev_error = occ.error;
    occ.prev_stat = header.status;
    occ.prev_ext_stat = header.ext_status;
    occ.prev_occs_present = header.occs_present;
    occ.prev_ips_status = header.ips_status;
    occ.prev_mode = header.mode;
    }
#[no_mangle]
pub unsafe extern "C" fn occ_setup_sysfs(occ: *mut occ) -> c_int {
    int occ_setup_sysfs(struct occ *occ)
    {
    return sysfs_create_group(&occ.bus_dev.kobj, &occ_sysfs);
    }
#[no_mangle]
pub unsafe extern "C" fn occ_shutdown_sysfs(occ: *mut occ) {
    void occ_shutdown_sysfs(struct occ *occ)
    {
    sysfs_remove_group(&occ.bus_dev.kobj, &occ_sysfs);
    }
