//! Automatically rewritten from C to Rust
//! Source: drivers/ufs/core/ufs-hwmon.c
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
// UFS hardware monitoring support
// Copyright (c) 2021, Western Digital Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ufs_hwmon_data {
    pub hba: *mut ufs_hba,
    pub mask: u8,
}

#[no_mangle]
unsafe extern "C" fn ufs_read_temp_enable(hba: *mut ufs_hba, mask: u8, val: *mut c_long) -> c_int {
    static int ufs_read_temp_enable(struct ufs_hba *hba, u8 mask, long *val)
    {
    u32 ee_mask;
    int err;
    err = ufshcd_query_attr(hba, UPIU_QUERY_OPCODE_READ_ATTR, QUERY_ATTR_IDN_EE_CONTROL, 0, 0,
    &ee_mask);
    if (err)
    return err;
// val = (mask & ee_mask & MASK_EE_TOO_HIGH_TEMP) || (mask & ee_mask & MASK_EE_TOO_LOW_TEMP);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ufs_get_temp(hba: *mut ufs_hba, idn: enum attr_idn, val: *mut c_long) -> c_int {
    static int ufs_get_temp(struct ufs_hba *hba, enum attr_idn idn, long *val)
    {
    u32 value;
    int err;
    err = ufshcd_query_attr(hba, UPIU_QUERY_OPCODE_READ_ATTR, idn, 0, 0, &value);
    if (err)
    return err;
    if (value == 0)
    return -ENODATA;
// val = ((long)value - 80) * MILLIDEGREE_PER_DEGREE;
    return 0;
    }
    static int ufs_hwmon_read(struct device *dev, enum hwmon_sensor_types type, u32 attr, int channel,
    long *val)
    {
    struct ufs_hwmon_data *data = dev_get_drvdata(dev);
    struct ufs_hba *hba = data.hba;
    int err;
    down(&hba.host_sem);
    if (!ufshcd_is_user_access_allowed(hba)) {
    up(&hba.host_sem);
    return -EBUSY;
    }
    ufshcd_rpm_get_sync(hba);
    switch (attr) {
    case hwmon_temp_enable:
    err = ufs_read_temp_enable(hba, data.mask, val);
    break;
    case hwmon_temp_crit:
    err = ufs_get_temp(hba, QUERY_ATTR_IDN_HIGH_TEMP_BOUND, val);
    break;
    case hwmon_temp_lcrit:
    err = ufs_get_temp(hba, QUERY_ATTR_IDN_LOW_TEMP_BOUND, val);
    break;
    case hwmon_temp_input:
    err = ufs_get_temp(hba, QUERY_ATTR_IDN_CASE_ROUGH_TEMP, val);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    ufshcd_rpm_put_sync(hba);
    up(&hba.host_sem);
    return err;
    }
    static int ufs_hwmon_write(struct device *dev, enum hwmon_sensor_types type, u32 attr, int channel,
    long val)
    {
    struct ufs_hwmon_data *data = dev_get_drvdata(dev);
    struct ufs_hba *hba = data.hba;
    int err;
    if (attr != hwmon_temp_enable)
    return -EINVAL;
    if (val != 0 && val != 1)
    return -EINVAL;
    down(&hba.host_sem);
    if (!ufshcd_is_user_access_allowed(hba)) {
    up(&hba.host_sem);
    return -EBUSY;
    }
    ufshcd_rpm_get_sync(hba);
    if (val == 1)
    err = ufshcd_update_ee_usr_mask(hba, MASK_EE_URGENT_TEMP, 0);
    else
    err = ufshcd_update_ee_usr_mask(hba, 0, MASK_EE_URGENT_TEMP);
    ufshcd_rpm_put_sync(hba);
    up(&hba.host_sem);
    return err;
    }
    static umode_t ufs_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    if (type != hwmon_temp)
    return 0;
    switch (attr) {
    case hwmon_temp_enable:
    return 0644;
    case hwmon_temp_crit:
    case hwmon_temp_lcrit:
    case hwmon_temp_input:
    return 0444;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info *const ufs_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_ENABLE | HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_LCRIT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ufs_hwmon_ops = {
    .is_visible	= ufs_hwmon_is_visible,
    .read		= ufs_hwmon_read,
    .write		= ufs_hwmon_write,
    };
    static const struct hwmon_chip_info ufs_hwmon_hba_info = {
    .ops	= &ufs_hwmon_ops,
    .info	= ufs_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn ufs_hwmon_probe(hba: *mut ufs_hba, mask: u8) {
    void ufs_hwmon_probe(struct ufs_hba *hba, u8 mask)
    {
    struct device *dev = hba.dev;
    struct ufs_hwmon_data *data;
    struct device *hwmon;
    data = kzalloc_obj(*data);
    if (!data)
    return;
    data.hba = hba;
    data.mask = mask;
    hwmon = hwmon_device_register_with_info(dev, "ufs", data, &ufs_hwmon_hba_info, core::ptr::null_mut());
    if (IS_ERR(hwmon)) {
    dev_warn(dev, "Failed to instantiate hwmon device\n");
    kfree(data);
    return;
    }
    hba.hwmon_device = hwmon;
    }
#[no_mangle]
pub unsafe extern "C" fn ufs_hwmon_remove(hba: *mut ufs_hba) {
    void ufs_hwmon_remove(struct ufs_hba *hba)
    {
    struct ufs_hwmon_data *data;
    if (!hba.hwmon_device)
    return;
    data = dev_get_drvdata(hba.hwmon_device);
    hwmon_device_unregister(hba.hwmon_device);
    hba.hwmon_device = core::ptr::null_mut();
    kfree(data);
    }
#[no_mangle]
pub unsafe extern "C" fn ufs_hwmon_notify_event(hba: *mut ufs_hba, ee_mask: u8) {
    void ufs_hwmon_notify_event(struct ufs_hba *hba, u8 ee_mask)
    {
    if (!hba.hwmon_device)
    return;
    if (ee_mask & MASK_EE_TOO_HIGH_TEMP)
    hwmon_notify_event(hba.hwmon_device, hwmon_temp, hwmon_temp_max_alarm, 0);
    if (ee_mask & MASK_EE_TOO_LOW_TEMP)
    hwmon_notify_event(hba.hwmon_device, hwmon_temp, hwmon_temp_min_alarm, 0);
    }
