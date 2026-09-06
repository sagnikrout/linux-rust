//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/realtek/realtek_hwmon.c
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
// HWMON support for Realtek PHY's
//
// Author: Heiner Kallweit <hkallweit1@gmail.com>
//

pub const RTL822X_VND2_TSALRM: c_uint = 0xa662;
pub const RTL822X_VND2_TSRR: c_uint = 0xbd84;
pub const RTL822X_VND2_TSSR: c_uint = 0xb54c;
#[no_mangle]
unsafe extern "C" fn rtl822x_hwmon_get_temp(raw: c_int) -> c_int {
    static int rtl822x_hwmon_get_temp(int raw)
    {
    if (raw >= 512)
    raw -= 1024;
    return 1000 * raw / 2;
    }
    static int rtl822x_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    int raw;
    switch (attr) {
    case hwmon_temp_input:
    raw = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_TSRR) & 0x3ff;
// val = rtl822x_hwmon_get_temp(raw);
    break;
    case hwmon_temp_max:
// Chip reduces speed to 1G if threshold is exceeded
    raw = phy_read_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_TSSR) >> 6;
// val = rtl822x_hwmon_get_temp(raw);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct hwmon_ops rtl822x_hwmon_ops = {
    .visible = 0444,
    .read = rtl822x_hwmon_read,
    };
    static const struct hwmon_channel_info * const rtl822x_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT | HWMON_T_MAX),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info rtl822x_hwmon_chip_info = {
    .ops = &rtl822x_hwmon_ops,
    .info = rtl822x_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn rtl822x_hwmon_init(phydev: *mut phy_device) -> c_int {
    int rtl822x_hwmon_init(struct phy_device *phydev)
    {
    struct device *hwdev, *dev = &phydev.mdio.dev;
// Ensure over-temp alarm is reset.
    phy_clear_bits_mmd(phydev, MDIO_MMD_VEND2, RTL822X_VND2_TSALRM, 3);
    hwdev = devm_hwmon_device_register_with_info(dev, core::ptr::null_mut(), phydev,
    &rtl822x_hwmon_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwdev);
    }
