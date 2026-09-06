//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface_platform_profile.c
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
// Surface Platform Profile / Performance Mode driver for Surface System
// Aggregator Module (thermal and fan subsystem).
//
// Copyright (C) 2021-2022 Maximilian Luz <luzmaximilian@gmail.com>
//

// Enum for the platform performance profile sent to the TMP module.
    enum ssam_tmp_profile {
    SSAM_TMP_PROFILE_NORMAL             = 1,
    SSAM_TMP_PROFILE_BATTERY_SAVER      = 2,
    SSAM_TMP_PROFILE_BETTER_PERFORMANCE = 3,
    SSAM_TMP_PROFILE_BEST_PERFORMANCE   = 4,
    };
// Enum for the fan profile sent to the FAN module. This fan profile is
// only sent to the EC if the 'has_fan' property is set. The integers are
// not a typo, they differ from the performance profile indices.
    enum ssam_fan_profile {
    SSAM_FAN_PROFILE_NORMAL             = 2,
    SSAM_FAN_PROFILE_BATTERY_SAVER      = 1,
    SSAM_FAN_PROFILE_BETTER_PERFORMANCE = 3,
    SSAM_FAN_PROFILE_BEST_PERFORMANCE   = 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tmp_profile_info {
    pub profile: __le32,
    pub unknown1: __le16,
    pub unknown2: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_platform_profile_device {
    pub sdev: *mut ssam_device,
    pub ppdev: *mut device,
    pub has_fan: bool,
}

    SSAM_DEFINE_SYNC_REQUEST_CL_R(__ssam_tmp_profile_get, struct ssam_tmp_profile_info, {
    .target_category = SSAM_SSH_TC_TMP,
    .command_id      = 0x02,
    });
    SSAM_DEFINE_SYNC_REQUEST_CL_W(__ssam_tmp_profile_set, __le32, {
    .target_category = SSAM_SSH_TC_TMP,
    .command_id      = 0x03,
    });
    SSAM_DEFINE_SYNC_REQUEST_W(__ssam_fan_profile_set, u8, {
    .target_category = SSAM_SSH_TC_FAN,
    .target_id = SSAM_SSH_TID_SAM,
    .command_id = 0x0e,
    .instance_id = 0x01,
    });
#[no_mangle]
unsafe extern "C" fn ssam_tmp_profile_get(sdev: *mut ssam_device, p: *mut enum ssam_tmp_profile) -> c_int {
    static int ssam_tmp_profile_get(struct ssam_device *sdev, enum ssam_tmp_profile *p)
    {
    struct ssam_tmp_profile_info info;
    int status;
    status = ssam_retry(__ssam_tmp_profile_get, sdev, &info);
    if (status < 0)
    return status;
// p = le32_to_cpu(info.profile);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssam_tmp_profile_set(sdev: *mut ssam_device, p: enum ssam_tmp_profile) -> c_int {
    static int ssam_tmp_profile_set(struct ssam_device *sdev, enum ssam_tmp_profile p)
    {
    let mut profile_le: __le32 = cpu_to_le32(p);
    return ssam_retry(__ssam_tmp_profile_set, sdev, &profile_le);
    }
#[no_mangle]
unsafe extern "C" fn ssam_fan_profile_set(sdev: *mut ssam_device, p: enum ssam_fan_profile) -> c_int {
    static int ssam_fan_profile_set(struct ssam_device *sdev, enum ssam_fan_profile p)
    {
    let mut profile: u8 = p;
    return ssam_retry(__ssam_fan_profile_set, sdev.ctrl, &profile);
    }
#[no_mangle]
unsafe extern "C" fn convert_ssam_tmp_to_profile(sdev: *mut ssam_device, p: enum ssam_tmp_profile) -> c_int {
    static int convert_ssam_tmp_to_profile(struct ssam_device *sdev, enum ssam_tmp_profile p)
    {
    switch (p) {
    case SSAM_TMP_PROFILE_NORMAL:
    return PLATFORM_PROFILE_BALANCED;
    case SSAM_TMP_PROFILE_BATTERY_SAVER:
    return PLATFORM_PROFILE_LOW_POWER;
    case SSAM_TMP_PROFILE_BETTER_PERFORMANCE:
    return PLATFORM_PROFILE_BALANCED_PERFORMANCE;
    case SSAM_TMP_PROFILE_BEST_PERFORMANCE:
    return PLATFORM_PROFILE_PERFORMANCE;
    default:
    dev_err(&sdev.dev, "invalid performance profile: %d", p);
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn convert_profile_to_ssam_tmp(sdev: *mut ssam_device, p: enum platform_profile_option) -> c_int {
    static int convert_profile_to_ssam_tmp(struct ssam_device *sdev, enum platform_profile_option p)
    {
    switch (p) {
    case PLATFORM_PROFILE_LOW_POWER:
    return SSAM_TMP_PROFILE_BATTERY_SAVER;
    case PLATFORM_PROFILE_BALANCED:
    return SSAM_TMP_PROFILE_NORMAL;
    case PLATFORM_PROFILE_BALANCED_PERFORMANCE:
    return SSAM_TMP_PROFILE_BETTER_PERFORMANCE;
    case PLATFORM_PROFILE_PERFORMANCE:
    return SSAM_TMP_PROFILE_BEST_PERFORMANCE;
    default:
// This should have already been caught by platform_profile_store().
    WARN(true, "unsupported platform profile");
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn convert_profile_to_ssam_fan(sdev: *mut ssam_device, p: enum platform_profile_option) -> c_int {
    static int convert_profile_to_ssam_fan(struct ssam_device *sdev, enum platform_profile_option p)
    {
    switch (p) {
    case PLATFORM_PROFILE_LOW_POWER:
    return SSAM_FAN_PROFILE_BATTERY_SAVER;
    case PLATFORM_PROFILE_BALANCED:
    return SSAM_FAN_PROFILE_NORMAL;
    case PLATFORM_PROFILE_BALANCED_PERFORMANCE:
    return SSAM_FAN_PROFILE_BETTER_PERFORMANCE;
    case PLATFORM_PROFILE_PERFORMANCE:
    return SSAM_FAN_PROFILE_BEST_PERFORMANCE;
    default:
// This should have already been caught by platform_profile_store().
    WARN(true, "unsupported platform profile");
    return -EOPNOTSUPP;
    }
    }
    static int ssam_platform_profile_get(struct device *dev,
    enum platform_profile_option *profile)
    {
    struct ssam_platform_profile_device *tpd;
    enum ssam_tmp_profile tp;
    int status;
    tpd = dev_get_drvdata(dev);
    status = ssam_tmp_profile_get(tpd.sdev, &tp);
    if (status)
    return status;
    status = convert_ssam_tmp_to_profile(tpd.sdev, tp);
    if (status < 0)
    return status;
// profile = status;
    return 0;
    }
    static int ssam_platform_profile_set(struct device *dev,
    enum platform_profile_option profile)
    {
    struct ssam_platform_profile_device *tpd;
    int tp;
    tpd = dev_get_drvdata(dev);
    tp = convert_profile_to_ssam_tmp(tpd.sdev, profile);
    if (tp < 0)
    return tp;
    tp = ssam_tmp_profile_set(tpd.sdev, tp);
    if (tp < 0)
    return tp;
    if (tpd.has_fan) {
    tp = convert_profile_to_ssam_fan(tpd.sdev, profile);
    if (tp < 0)
    return tp;
    tp = ssam_fan_profile_set(tpd.sdev, tp);
    }
    return tp;
    }
#[no_mangle]
unsafe extern "C" fn ssam_platform_profile_probe(drvdata: *mut c_void, choices: *mut c_ulong) -> c_int {
    static int ssam_platform_profile_probe(void *drvdata, unsigned long *choices)
    {
    set_bit(PLATFORM_PROFILE_LOW_POWER, choices);
    set_bit(PLATFORM_PROFILE_BALANCED, choices);
    set_bit(PLATFORM_PROFILE_BALANCED_PERFORMANCE, choices);
    set_bit(PLATFORM_PROFILE_PERFORMANCE, choices);
    return 0;
    }
    static const struct platform_profile_ops ssam_platform_profile_ops = {
    .probe = ssam_platform_profile_probe,
    .profile_get = ssam_platform_profile_get,
    .profile_set = ssam_platform_profile_set,
    };
#[no_mangle]
unsafe extern "C" fn surface_platform_profile_probe(sdev: *mut ssam_device) -> c_int {
    static int surface_platform_profile_probe(struct ssam_device *sdev)
    {
    struct ssam_platform_profile_device *tpd;
    tpd = devm_kzalloc(&sdev.dev, sizeof(*tpd), GFP_KERNEL);
    if (!tpd)
    return -ENOMEM;
    tpd.sdev = sdev;
    ssam_device_set_drvdata(sdev, tpd);
    tpd.has_fan = device_property_read_bool(&sdev.dev, "has_fan");
    tpd.ppdev = devm_platform_profile_register(&sdev.dev, "Surface Platform Profile",
    tpd, &ssam_platform_profile_ops);
    return PTR_ERR_OR_ZERO(tpd.ppdev);
    }
    static const struct ssam_device_id ssam_platform_profile_match[] = {
    { SSAM_SDEV(TMP, SAM, 0x00, 0x01) },
    { },
    };
    MODULE_DEVICE_TABLE(ssam, ssam_platform_profile_match);
    static struct ssam_device_driver surface_platform_profile = {
    .probe = surface_platform_profile_probe,
    .match_table = ssam_platform_profile_match,
    .driver = {
    .name = "surface_platform_profile",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_ssam_device_driver(surface_platform_profile);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Platform Profile Support for Surface System Aggregator Module");
    MODULE_LICENSE("GPL");
