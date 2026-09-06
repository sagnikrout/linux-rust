//! Automatically rewritten from C to Rust
//! Source: drivers/nvme/host/hwmon.c
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
// NVM Express hardware monitoring support
// Copyright (c) 2019, Guenter Roeck
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_hwmon_data {
    pub ctrl: *mut nvme_ctrl,
    pub log: *mut nvme_smart_log,
    pub read_lock: mutex,
}

    static int nvme_get_temp_thresh(struct nvme_ctrl *ctrl, int sensor, bool under,
    long *temp)
    {
    let mut threshold: c_uint = sensor << NVME_TEMP_THRESH_SELECT_SHIFT;
    u32 status;
    int ret;
    if (under)
    threshold |= NVME_TEMP_THRESH_TYPE_UNDER;
    ret = nvme_get_features(ctrl, NVME_FEAT_TEMP_THRESH, threshold, core::ptr::null_mut(), 0,
    &status);
    if (ret > 0)
    return -EIO;
    if (ret < 0)
    return ret;
// temp = kelvin_to_millicelsius(status & NVME_TEMP_THRESH_MASK);
    return 0;
    }
    static int nvme_set_temp_thresh(struct nvme_ctrl *ctrl, int sensor, bool under,
    long temp)
    {
    let mut threshold: c_uint = sensor << NVME_TEMP_THRESH_SELECT_SHIFT;
    int ret;
    temp = millicelsius_to_kelvin(temp);
    threshold |= clamp_val(temp, 0, NVME_TEMP_THRESH_MASK);
    if (under)
    threshold |= NVME_TEMP_THRESH_TYPE_UNDER;
    ret = nvme_set_features(ctrl, NVME_FEAT_TEMP_THRESH, threshold, core::ptr::null_mut(), 0,
    core::ptr::null_mut());
    if (ret > 0)
    return -EIO;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nvme_hwmon_get_smart_log(data: *mut nvme_hwmon_data) -> c_int {
    static int nvme_hwmon_get_smart_log(struct nvme_hwmon_data *data)
    {
    return nvme_get_log(data.ctrl, NVME_NSID_ALL, NVME_LOG_SMART, 0,
    NVME_CSI_NVM, data.log, sizeof(*data.log), 0);
    }
    static int nvme_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct nvme_hwmon_data *data = dev_get_drvdata(dev);
    struct nvme_smart_log *log = data.log;
    int temp;
    int err;
//
// First handle attributes which don't require us to read
// the smart log.
//
    switch (attr) {
    case hwmon_temp_max:
    return nvme_get_temp_thresh(data.ctrl, channel, false, val);
    case hwmon_temp_min:
    return nvme_get_temp_thresh(data.ctrl, channel, true, val);
    case hwmon_temp_crit:
// val = kelvin_to_millicelsius(data->ctrl->cctemp);
    return 0;
    default:
    break;
    }
    mutex_lock(&data.read_lock);
    err = nvme_hwmon_get_smart_log(data);
    if (err)
    goto unlock;
    switch (attr) {
    case hwmon_temp_input:
    if (!channel)
    temp = get_unaligned_le16(log.temperature);
    else
    temp = le16_to_cpu(log.temp_sensor[channel - 1]);
// val = kelvin_to_millicelsius(temp);
    break;
    case hwmon_temp_alarm:
// val = !!(log->critical_warning & NVME_SMART_CRIT_TEMPERATURE);
    break;
    default:
    err = -EOPNOTSUPP;
    break;
    }
    unlock:
    mutex_unlock(&data.read_lock);
    return err;
    }
    static int nvme_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct nvme_hwmon_data *data = dev_get_drvdata(dev);
    switch (attr) {
    case hwmon_temp_max:
    return nvme_set_temp_thresh(data.ctrl, channel, false, val);
    case hwmon_temp_min:
    return nvme_set_temp_thresh(data.ctrl, channel, true, val);
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static const char * const nvme_hwmon_sensor_names[] = {
    "Composite",
    "Sensor 1",
    "Sensor 2",
    "Sensor 3",
    "Sensor 4",
    "Sensor 5",
    "Sensor 6",
    "Sensor 7",
    "Sensor 8",
    };
    static int nvme_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
// str = nvme_hwmon_sensor_names[channel];
    return 0;
    }
    static umode_t nvme_hwmon_is_visible(const void *_data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct nvme_hwmon_data *data = _data;
    switch (attr) {
    case hwmon_temp_crit:
    if (!channel && data.ctrl.cctemp)
    return 0444;
    break;
    case hwmon_temp_max:
    case hwmon_temp_min:
    if ((!channel && data.ctrl.wctemp) ||
    (channel && data.log.temp_sensor[channel - 1] &&
    !(data.ctrl.quirks &
    NVME_QUIRK_NO_SECONDARY_TEMP_THRESH))) {
    if (data.ctrl.quirks &
    NVME_QUIRK_NO_TEMP_THRESH_CHANGE)
    return 0444;
    return 0644;
    }
    break;
    case hwmon_temp_alarm:
    if (!channel)
    return 0444;
    break;
    case hwmon_temp_input:
    case hwmon_temp_label:
    if (!channel || data.log.temp_sensor[channel - 1])
    return 0444;
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info *const nvme_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_CRIT | HWMON_T_LABEL | HWMON_T_ALARM,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops nvme_hwmon_ops = {
    .is_visible	= nvme_hwmon_is_visible,
    .read		= nvme_hwmon_read,
    .read_string	= nvme_hwmon_read_string,
    .write		= nvme_hwmon_write,
    };
    static const struct hwmon_chip_info nvme_hwmon_chip_info = {
    .ops	= &nvme_hwmon_ops,
    .info	= nvme_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn nvme_hwmon_init(ctrl: *mut nvme_ctrl) -> c_int {
    int nvme_hwmon_init(struct nvme_ctrl *ctrl)
    {
    struct device *dev = ctrl.device;
    struct nvme_hwmon_data *data;
    struct device *hwmon;
    int err;
    data = kzalloc_obj(*data);
    if (!data)
    return -ENOMEM;
    data.log = kzalloc_obj(*data.log);
    if (!data.log) {
    err = -ENOMEM;
    goto err_free_data;
    }
    data.ctrl = ctrl;
    mutex_init(&data.read_lock);
    err = nvme_hwmon_get_smart_log(data);
    if (err) {
    dev_warn(dev, "Failed to read smart log (error %d)\n", err);
    goto err_free_log;
    }
    hwmon = hwmon_device_register_with_info(dev, "nvme",
    data, &nvme_hwmon_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon)) {
    dev_warn(dev, "Failed to instantiate hwmon device\n");
    err = PTR_ERR(hwmon);
    goto err_free_log;
    }
    ctrl.hwmon_device = hwmon;
    return 0;
    err_free_log:
    kfree(data.log);
    err_free_data:
    kfree(data);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn nvme_hwmon_exit(ctrl: *mut nvme_ctrl) {
    void nvme_hwmon_exit(struct nvme_ctrl *ctrl)
    {
    if (ctrl.hwmon_device) {
    struct nvme_hwmon_data *data =
    dev_get_drvdata(ctrl.hwmon_device);
    hwmon_device_unregister(ctrl.hwmon_device);
    ctrl.hwmon_device = core::ptr::null_mut();
    kfree(data.log);
    kfree(data);
    }
    }
