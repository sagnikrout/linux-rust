//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/aquantia/aquantia_hwmon.c
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
// HWMON driver for Aquantia PHY
//
// Author: Nikita Yushchenko <nikita.yoush@cogentembedded.com>
// Author: Andrew Lunn <andrew@lunn.ch>
// Author: Heiner Kallweit <hkallweit1@gmail.com>
//

    static umode_t aqr_hwmon_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type != hwmon_temp)
    return 0;
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_min_alarm:
    case hwmon_temp_max_alarm:
    case hwmon_temp_lcrit_alarm:
    case hwmon_temp_crit_alarm:
    return 0444;
    case hwmon_temp_min:
    case hwmon_temp_max:
    case hwmon_temp_lcrit:
    case hwmon_temp_crit:
    return 0644;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn aqr_hwmon_get(phydev: *mut phy_device, reg: c_int, value: *mut c_long) -> c_int {
    static int aqr_hwmon_get(struct phy_device *phydev, int reg, long *value)
    {
    let mut temp: c_int = phy_read_mmd(phydev, MDIO_MMD_VEND1, reg);
    if (temp < 0)
    return temp;
// 16 bit value is 2's complement with LSB = 1/256th degree Celsius
// value = (s16)temp * 1000 / 256;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aqr_hwmon_set(phydev: *mut phy_device, reg: c_int, value: c_long) -> c_int {
    static int aqr_hwmon_set(struct phy_device *phydev, int reg, long value)
    {
    int temp;
    if (value >= 128000 || value < -128000)
    return -ERANGE;
    temp = value * 256 / 1000;
// temp is in s16 range and we're interested in lower 16 bits only
    return phy_write_mmd(phydev, MDIO_MMD_VEND1, reg, (u16)temp);
    }
#[no_mangle]
unsafe extern "C" fn aqr_hwmon_test_bit(phydev: *mut phy_device, reg: c_int, bit: c_int) -> c_int {
    static int aqr_hwmon_test_bit(struct phy_device *phydev, int reg, int bit)
    {
    let mut val: c_int = phy_read_mmd(phydev, MDIO_MMD_VEND1, reg);
    if (val < 0)
    return val;
    return !!(val & bit);
    }
#[no_mangle]
unsafe extern "C" fn aqr_hwmon_status1(phydev: *mut phy_device, bit: c_int, value: *mut c_long) -> c_int {
    static int aqr_hwmon_status1(struct phy_device *phydev, int bit, long *value)
    {
    let mut val: c_int = aqr_hwmon_test_bit(phydev, VEND1_GENERAL_STAT1, bit);
    if (val < 0)
    return val;
// value = val;
    return 0;
    }
    static int aqr_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *value)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    int reg;
    if (type != hwmon_temp)
    return -EOPNOTSUPP;
    switch (attr) {
    case hwmon_temp_input:
    reg = aqr_hwmon_test_bit(phydev, VEND1_THERMAL_STAT2,
    VEND1_THERMAL_STAT2_VALID);
    if (reg < 0)
    return reg;
    if (!reg)
    return -EBUSY;
    return aqr_hwmon_get(phydev, VEND1_THERMAL_STAT1, value);
    case hwmon_temp_lcrit:
    return aqr_hwmon_get(phydev, VEND1_THERMAL_PROV_LOW_TEMP_FAIL,
    value);
    case hwmon_temp_min:
    return aqr_hwmon_get(phydev, VEND1_THERMAL_PROV_LOW_TEMP_WARN,
    value);
    case hwmon_temp_max:
    return aqr_hwmon_get(phydev, VEND1_THERMAL_PROV_HIGH_TEMP_WARN,
    value);
    case hwmon_temp_crit:
    return aqr_hwmon_get(phydev, VEND1_THERMAL_PROV_HIGH_TEMP_FAIL,
    value);
    case hwmon_temp_lcrit_alarm:
    return aqr_hwmon_status1(phydev,
    VEND1_GENERAL_STAT1_LOW_TEMP_FAIL,
    value);
    case hwmon_temp_min_alarm:
    return aqr_hwmon_status1(phydev,
    VEND1_GENERAL_STAT1_LOW_TEMP_WARN,
    value);
    case hwmon_temp_max_alarm:
    return aqr_hwmon_status1(phydev,
    VEND1_GENERAL_STAT1_HIGH_TEMP_WARN,
    value);
    case hwmon_temp_crit_alarm:
    return aqr_hwmon_status1(phydev,
    VEND1_GENERAL_STAT1_HIGH_TEMP_FAIL,
    value);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int aqr_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long value)
    {
    struct phy_device *phydev = dev_get_drvdata(dev);
    if (type != hwmon_temp)
    return -EOPNOTSUPP;
    switch (attr) {
    case hwmon_temp_lcrit:
    return aqr_hwmon_set(phydev, VEND1_THERMAL_PROV_LOW_TEMP_FAIL,
    value);
    case hwmon_temp_min:
    return aqr_hwmon_set(phydev, VEND1_THERMAL_PROV_LOW_TEMP_WARN,
    value);
    case hwmon_temp_max:
    return aqr_hwmon_set(phydev, VEND1_THERMAL_PROV_HIGH_TEMP_WARN,
    value);
    case hwmon_temp_crit:
    return aqr_hwmon_set(phydev, VEND1_THERMAL_PROV_HIGH_TEMP_FAIL,
    value);
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_ops aqr_hwmon_ops = {
    .is_visible = aqr_hwmon_is_visible,
    .read = aqr_hwmon_read,
    .write = aqr_hwmon_write,
    };
    static const struct hwmon_channel_info * const aqr_hwmon_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT |
    HWMON_T_MAX | HWMON_T_MIN |
    HWMON_T_MAX_ALARM | HWMON_T_MIN_ALARM |
    HWMON_T_CRIT | HWMON_T_LCRIT |
    HWMON_T_CRIT_ALARM | HWMON_T_LCRIT_ALARM),
    core::ptr::null_mut(),
    };
    static const struct hwmon_chip_info aqr_hwmon_chip_info = {
    .ops = &aqr_hwmon_ops,
    .info = aqr_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn aqr_hwmon_probe(phydev: *mut phy_device) -> c_int {
    int aqr_hwmon_probe(struct phy_device *phydev)
    {
    struct device *dev = &phydev.mdio.dev;
    struct device *hwmon_dev;
    char *hwmon_name;
    int i, j;
    hwmon_name = devm_kstrdup(dev, dev_name(dev), GFP_KERNEL);
    if (!hwmon_name)
    return -ENOMEM;
    for (i = j = 0; hwmon_name[i]; i++) {
    if (isalnum(hwmon_name[i])) {
    if (i != j)
    hwmon_name[j] = hwmon_name[i];
    j++;
    }
    }
    hwmon_name[j] = '\0';
    hwmon_dev = devm_hwmon_device_register_with_info(dev, hwmon_name,
    phydev, &aqr_hwmon_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
