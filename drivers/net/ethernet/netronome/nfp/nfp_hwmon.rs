//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfp_hwmon.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017 Netronome Systems, Inc.

#[no_mangle]
unsafe extern "C" fn nfp_hwmon_sensor_id(type: enum hwmon_sensor_types, channel: c_int) -> c_int {
    static int nfp_hwmon_sensor_id(enum hwmon_sensor_types type, int channel)
    {
    if (type == hwmon_temp)
    return NFP_SENSOR_CHIP_TEMPERATURE;
    if (type == hwmon_power)
    return NFP_SENSOR_ASSEMBLY_POWER + channel;
    return -EINVAL;
    }
    static int
    nfp_hwmon_read(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    static const struct {
    enum hwmon_sensor_types type;
    u32 attr;
    long val;
    } const_vals[] = {
    { hwmon_temp,	hwmon_temp_max,		NFP_TEMP_MAX },
    { hwmon_temp,	hwmon_temp_crit,	NFP_TEMP_CRIT },
    { hwmon_power,	hwmon_power_max,	NFP_POWER_MAX },
    };
    struct nfp_pf *pf = dev_get_drvdata(dev);
    enum nfp_nsp_sensor_id id;
    int err, i;
    for (i = 0; i < ARRAY_SIZE(const_vals); i++)
    if (const_vals[i].type == type && const_vals[i].attr == attr) {
// val = const_vals[i].val;
    return 0;
    }
    err = nfp_hwmon_sensor_id(type, channel);
    if (err < 0)
    return err;
    id = err;
    if (!(pf.nspi.sensor_mask & BIT(id)))
    return -EOPNOTSUPP;
    if (type == hwmon_temp && attr == hwmon_temp_input)
    return nfp_hwmon_read_sensor(pf.cpp, id, val);
    if (type == hwmon_power && attr == hwmon_power_input)
    return nfp_hwmon_read_sensor(pf.cpp, id, val);
    return -EINVAL;
    }
    static umode_t
    nfp_hwmon_is_visible(const void *data, enum hwmon_sensor_types type, u32 attr,
    int channel)
    {
    if (type == hwmon_temp) {
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_crit:
    case hwmon_temp_max:
    return 0444;
    }
    } else if (type == hwmon_power) {
    switch (attr) {
    case hwmon_power_input:
    case hwmon_power_max:
    return 0444;
    }
    }
    return 0;
    }
    static const struct hwmon_channel_info * const nfp_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT),
    HWMON_CHANNEL_INFO(power, HWMON_P_INPUT | HWMON_P_MAX,
    HWMON_P_INPUT,
    HWMON_P_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops nfp_hwmon_ops = {
    .is_visible = nfp_hwmon_is_visible,
    .read = nfp_hwmon_read,
    };
    static const struct hwmon_chip_info nfp_chip_info = {
    .ops = &nfp_hwmon_ops,
    .info = nfp_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn nfp_hwmon_register(pf: *mut nfp_pf) -> c_int {
    int nfp_hwmon_register(struct nfp_pf *pf)
    {
    if (!IS_REACHABLE(CONFIG_HWMON))
    return 0;
    if (!pf.nspi) {
    nfp_warn(pf.cpp, "not registering HWMON (no NSP info)\n");
    return 0;
    }
    if (!pf.nspi.sensor_mask) {
    nfp_info(pf.cpp,
    "not registering HWMON (NSP doesn't report sensors)\n");
    return 0;
    }
    pf.hwmon_dev = hwmon_device_register_with_info(&pf.pdev.dev, "nfp",
    pf, &nfp_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(pf.hwmon_dev);
    }
#[no_mangle]
pub unsafe extern "C" fn nfp_hwmon_unregister(pf: *mut nfp_pf) {
    void nfp_hwmon_unregister(struct nfp_pf *pf)
    {
    if (!IS_REACHABLE(CONFIG_HWMON) || !pf.hwmon_dev)
    return;
    hwmon_device_unregister(pf.hwmon_dev);
    }
