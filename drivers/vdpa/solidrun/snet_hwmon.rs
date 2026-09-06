//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/solidrun/snet_hwmon.c
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
// SolidRun DPU driver for control plane
//
// Copyright (C) 2022-2023 SolidRun
//
// Author: Alvaro Karsz <alvaro.karsz@solid-run.com>
//

// Monitor offsets
pub const SNET_MON_TMP0_IN_OFF: c_uint = 0x00;
pub const SNET_MON_TMP0_MAX_OFF: c_uint = 0x08;
pub const SNET_MON_TMP0_CRIT_OFF: c_uint = 0x10;
pub const SNET_MON_TMP1_IN_OFF: c_uint = 0x18;
pub const SNET_MON_TMP1_CRIT_OFF: c_uint = 0x20;
pub const SNET_MON_CURR_IN_OFF: c_uint = 0x28;
pub const SNET_MON_CURR_MAX_OFF: c_uint = 0x30;
pub const SNET_MON_CURR_CRIT_OFF: c_uint = 0x38;
pub const SNET_MON_PWR_IN_OFF: c_uint = 0x40;
pub const SNET_MON_VOLT_IN_OFF: c_uint = 0x48;
pub const SNET_MON_VOLT_CRIT_OFF: c_uint = 0x50;
pub const SNET_MON_VOLT_LCRIT_OFF: c_uint = 0x58;
#[no_mangle]
unsafe extern "C" fn snet_hwmon_read_reg(psnet: *mut psnet, reg: u32, out: *mut c_long) {
    static void snet_hwmon_read_reg(struct psnet *psnet, u32 reg, long *out)
    {
// out = psnet_read64(psnet, psnet->cfg.hwmon_off + reg);
    }
    static umode_t snet_howmon_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    return 0444;
    }
    static int snet_howmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct psnet *psnet = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    switch (type) {
    case hwmon_in:
    switch (attr) {
    case hwmon_in_lcrit:
    snet_hwmon_read_reg(psnet, SNET_MON_VOLT_LCRIT_OFF, val);
    break;
    case hwmon_in_crit:
    snet_hwmon_read_reg(psnet, SNET_MON_VOLT_CRIT_OFF, val);
    break;
    case hwmon_in_input:
    snet_hwmon_read_reg(psnet, SNET_MON_VOLT_IN_OFF, val);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    break;
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
    snet_hwmon_read_reg(psnet, SNET_MON_PWR_IN_OFF, val);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    break;
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_input:
    snet_hwmon_read_reg(psnet, SNET_MON_CURR_IN_OFF, val);
    break;
    case hwmon_curr_max:
    snet_hwmon_read_reg(psnet, SNET_MON_CURR_MAX_OFF, val);
    break;
    case hwmon_curr_crit:
    snet_hwmon_read_reg(psnet, SNET_MON_CURR_CRIT_OFF, val);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    break;
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    if (channel == 0)
    snet_hwmon_read_reg(psnet, SNET_MON_TMP0_IN_OFF, val);
    else
    snet_hwmon_read_reg(psnet, SNET_MON_TMP1_IN_OFF, val);
    break;
    case hwmon_temp_max:
    if (channel == 0)
    snet_hwmon_read_reg(psnet, SNET_MON_TMP0_MAX_OFF, val);
    else
    ret = -EOPNOTSUPP;
    break;
    case hwmon_temp_crit:
    if (channel == 0)
    snet_hwmon_read_reg(psnet, SNET_MON_TMP0_CRIT_OFF, val);
    else
    snet_hwmon_read_reg(psnet, SNET_MON_TMP1_CRIT_OFF, val);
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static int snet_hwmon_read_string(struct device *dev,
    enum hwmon_sensor_types type, u32 attr,
    int channel, const char **str)
    {
    let mut ret: c_int = 0;
    switch (type) {
    case hwmon_in:
// str = "main_vin";
    break;
    case hwmon_power:
// str = "soc_pin";
    break;
    case hwmon_curr:
// str = "soc_iin";
    break;
    case hwmon_temp:
    if (channel == 0)
// str = "power_stage_temp";
    else
// str = "ic_junction_temp";
    break;
    default:
    ret = -EOPNOTSUPP;
    break;
    }
    return ret;
    }
    static const struct hwmon_ops snet_hwmon_ops = {
    .is_visible = snet_howmon_is_visible,
    .read = snet_howmon_read,
    .read_string = snet_hwmon_read_string
    };
    static const struct hwmon_channel_info * const snet_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_CRIT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_CRIT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(power, HWMON_P_INPUT | HWMON_P_LABEL),
    HWMON_CHANNEL_INFO(curr, HWMON_C_INPUT | HWMON_C_MAX | HWMON_C_CRIT | HWMON_C_LABEL),
    HWMON_CHANNEL_INFO(in, HWMON_I_INPUT | HWMON_I_CRIT | HWMON_I_LCRIT | HWMON_I_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info snet_hwmono_info = {
    .ops = &snet_hwmon_ops,
    .info = snet_hwmon_info,
    };
// Create an HW monitor device
#[no_mangle]
pub unsafe extern "C" fn psnet_create_hwmon(pdev: *mut pci_dev) {
    void psnet_create_hwmon(struct pci_dev *pdev)
    {
    struct device *hwmon;
    struct psnet *psnet = pci_get_drvdata(pdev);
    snprintf(psnet.hwmon_name, SNET_NAME_SIZE, "snet_%s", pci_name(pdev));
    hwmon = devm_hwmon_device_register_with_info(&pdev.dev, psnet.hwmon_name, psnet,
    &snet_hwmono_info, core::ptr::null_mut());
// The monitor is not mandatory, Just alert user in case of an error
    if (IS_ERR(hwmon))
    SNET_WARN(pdev, "Failed to create SNET hwmon, error %ld\n", PTR_ERR(hwmon));
    }
