//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/surface_temp.c
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
//
// Thermal sensor subsystem driver for Surface System Aggregator Module (SSAM).
//
// Copyright (C) 2022-2023 Maximilian Luz <luzmaximilian@gmail.com>
//

// -- SAM interface. --------------------------------------------------------
//
// Available sensors are indicated by a 16-bit bitfield, where a 1 marks the
// presence of a sensor. So we have at most 16 possible sensors/channels.
//
pub const SSAM_TMP_SENSOR_MAX_COUNT: c_int = 16;
//
// All names observed so far are 6 characters long, but there's only
// zeros after the name, so perhaps they can be longer. This number reflects
// the maximum zero-padded space observed in the returned buffer.
//
pub const SSAM_TMP_SENSOR_NAME_LENGTH: c_int = 18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tmp_get_name_rsp {
    pub unknown1: __le16,
    pub unknown2: c_char,
    pub name: [c_char; SSAM_TMP_SENSOR_NAME_LENGTH],
    pub __packed: },
    pub 21): static_assert(sizeof(struct ssam_tmp_get_name_rsp) ==,
    SSAM_DEFINE_SYNC_REQUEST_CL_R(__ssam_tmp_get_available_sensors, __le16, {
    .target_category = SSAM_SSH_TC_TMP,
    .command_id      = 0x04,
    SSAM_DEFINE_SYNC_REQUEST_MD_R(__ssam_tmp_get_temperature, __le16, {
    .target_category = SSAM_SSH_TC_TMP,
    .command_id      = 0x01,
    SSAM_DEFINE_SYNC_REQUEST_MD_R(__ssam_tmp_get_name, struct ssam_tmp_get_name_rsp, {
    .target_category = SSAM_SSH_TC_TMP,
    .command_id      = 0x0e,
#[no_mangle]
unsafe extern "C" fn ssam_tmp_get_available_sensors(sdev: *mut ssam_device, sensors: *mut i16) -> c_int {
    static int ssam_tmp_get_available_sensors(struct ssam_device *sdev, s16 *sensors)
    {
    pub sensors_le: __le16,
    pub status: c_int,
    pub &sensors_le): status = __ssam_tmp_get_available_sensors(sdev,,
    if (status)
    pub status: return,
// sensors = le16_to_cpu(sensors_le);
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ssam_tmp_get_temperature(sdev: *mut ssam_device, iid: u8, temperature: *mut c_long) -> c_int {
    static int ssam_tmp_get_temperature(struct ssam_device *sdev, u8 iid, long *temperature)
    {
    pub temp_le: __le16,
    pub status: c_int,
    pub &temp_le): status = __ssam_tmp_get_temperature(sdev->ctrl, sdev->uid.target, iid,,
    if (status)
    pub status: return,
// Convert 1/10 °K to 1/1000 °C
// temperature = (le16_to_cpu(temp_le) - 2731) * 100L;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ssam_tmp_get_name(sdev: *mut ssam_device, iid: u8, buf: *mut c_char, buf_len: usize) -> c_int {
    static int ssam_tmp_get_name(struct ssam_device *sdev, u8 iid, char *buf, size_t buf_len)
    {
    pub name_rsp: ssam_tmp_get_name_rsp,
    pub status: c_int,
    pub &name_rsp): status = __ssam_tmp_get_name(sdev->ctrl, sdev->uid.target, iid,,
    if (status)
    pub status: return,
//
// This should not fail unless the name in the returned struct is not
// null-terminated or someone changed something in the struct
// definitions above, since our buffer and struct have the same
// capacity by design. So if this fails, log an error message. Since
// the more likely cause is that the returned string isn't
// null-terminated, we might have received garbage (as opposed to just
// an incomplete string), so also fail the function.
//
    pub buf_len): status = strscpy(buf, name_rsp.name,,
    if (status < 0) {
    pub string\n"): dev_err(&sdev->dev, "received non-null-terminated sensor name,
    pub status: return,
    }
    pub 0: return,
    }
// -- Driver.----------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_temp {
    pub sdev: *mut ssam_device,
    pub sensors: i16,
    pub names: [c_char; SSAM_TMP_SENSOR_MAX_COUNT][SSAM_TMP_SENSOR_NAME_LENGTH],
}

    static umode_t ssam_temp_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct ssam_temp *ssam_temp = data;
    if (!(ssam_temp.sensors & BIT(channel)))
    return 0;
    return 0444;
    }
    static int ssam_temp_hwmon_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long *value)
    {
    const struct ssam_temp *ssam_temp = dev_get_drvdata(dev);
    return ssam_tmp_get_temperature(ssam_temp.sdev, channel + 1, value);
    }
    static int ssam_temp_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    const struct ssam_temp *ssam_temp = dev_get_drvdata(dev);
// str = ssam_temp->names[channel];
    return 0;
    }
    static const struct hwmon_channel_info * const ssam_temp_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip,
    HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops ssam_temp_hwmon_ops = {
    .is_visible = ssam_temp_hwmon_is_visible,
    .read = ssam_temp_hwmon_read,
    .read_string = ssam_temp_hwmon_read_string,
    };
    static const struct hwmon_chip_info ssam_temp_hwmon_chip_info = {
    .ops = &ssam_temp_hwmon_ops,
    .info = ssam_temp_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn ssam_temp_probe(sdev: *mut ssam_device) -> c_int {
    static int ssam_temp_probe(struct ssam_device *sdev)
    {
    struct ssam_temp *ssam_temp;
    struct device *hwmon_dev;
    s16 sensors;
    int channel;
    int status;
    status = ssam_tmp_get_available_sensors(sdev, &sensors);
    if (status)
    return status;
    ssam_temp = devm_kzalloc(&sdev.dev, sizeof(*ssam_temp), GFP_KERNEL);
    if (!ssam_temp)
    return -ENOMEM;
    ssam_temp.sdev = sdev;
    ssam_temp.sensors = sensors;
// Retrieve the name for each available sensor.
    for (channel = 0; channel < SSAM_TMP_SENSOR_MAX_COUNT; channel++) {
    if (!(sensors & BIT(channel)))
    continue;
    status = ssam_tmp_get_name(sdev, channel + 1, ssam_temp.names[channel],
    SSAM_TMP_SENSOR_NAME_LENGTH);
    if (status)
    return status;
    }
    hwmon_dev = devm_hwmon_device_register_with_info(&sdev.dev, "surface_thermal", ssam_temp,
    &ssam_temp_hwmon_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct ssam_device_id ssam_temp_match[] = {
    { SSAM_SDEV(TMP, SAM, 0x00, 0x02) },
    { },
    };
    MODULE_DEVICE_TABLE(ssam, ssam_temp_match);
    static struct ssam_device_driver ssam_temp = {
    .probe = ssam_temp_probe,
    .match_table = ssam_temp_match,
    .driver = {
    .name = "surface_temp",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_ssam_device_driver(ssam_temp);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Thermal sensor subsystem driver for Surface System Aggregator Module");
    MODULE_LICENSE("GPL");
