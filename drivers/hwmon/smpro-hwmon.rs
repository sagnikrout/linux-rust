//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/smpro-hwmon.c
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
// Ampere Computing SoC's SMPro Hardware Monitoring Driver
//
// Copyright (c) 2022, Ampere Computing LLC
//

// Logical Power Sensor Registers
pub const SOC_TEMP: c_uint = 0x10;
pub const SOC_VRD_TEMP: c_uint = 0x11;
pub const DIMM_VRD_TEMP: c_uint = 0x12;
pub const CORE_VRD_TEMP: c_uint = 0x13;
pub const CH0_DIMM_TEMP: c_uint = 0x14;
pub const CH1_DIMM_TEMP: c_uint = 0x15;
pub const CH2_DIMM_TEMP: c_uint = 0x16;
pub const CH3_DIMM_TEMP: c_uint = 0x17;
pub const CH4_DIMM_TEMP: c_uint = 0x18;
pub const CH5_DIMM_TEMP: c_uint = 0x19;
pub const CH6_DIMM_TEMP: c_uint = 0x1A;
pub const CH7_DIMM_TEMP: c_uint = 0x1B;
pub const RCA_VRD_TEMP: c_uint = 0x1C;
pub const CORE_VRD_PWR: c_uint = 0x20;
pub const SOC_PWR: c_uint = 0x21;
pub const DIMM_VRD1_PWR: c_uint = 0x22;
pub const DIMM_VRD2_PWR: c_uint = 0x23;
pub const CORE_VRD_PWR_MW: c_uint = 0x26;
pub const SOC_PWR_MW: c_uint = 0x27;
pub const DIMM_VRD1_PWR_MW: c_uint = 0x28;
pub const DIMM_VRD2_PWR_MW: c_uint = 0x29;
pub const RCA_VRD_PWR: c_uint = 0x2A;
pub const RCA_VRD_PWR_MW: c_uint = 0x2B;
pub const MEM_HOT_THRESHOLD: c_uint = 0x32;
pub const SOC_VR_HOT_THRESHOLD: c_uint = 0x33;
pub const CORE_VRD_VOLT: c_uint = 0x34;
pub const SOC_VRD_VOLT: c_uint = 0x35;
pub const DIMM_VRD1_VOLT: c_uint = 0x36;
pub const DIMM_VRD2_VOLT: c_uint = 0x37;
pub const RCA_VRD_VOLT: c_uint = 0x38;
pub const CORE_VRD_CURR: c_uint = 0x39;
pub const SOC_VRD_CURR: c_uint = 0x3A;
pub const DIMM_VRD1_CURR: c_uint = 0x3B;
pub const DIMM_VRD2_CURR: c_uint = 0x3C;
pub const RCA_VRD_CURR: c_uint = 0x3D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smpro_hwmon {
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smpro_sensor {
    pub reg: u8,
    pub reg_ext: u8,
    pub label: *const c_char,
}

    static const struct smpro_sensor temperature[] = {
    {
    .reg = SOC_TEMP,
    .label = "temp1 SoC"
    },
    {
    .reg = SOC_VRD_TEMP,
    .reg_ext = SOC_VR_HOT_THRESHOLD,
    .label = "temp2 SoC VRD"
    },
    {
    .reg = DIMM_VRD_TEMP,
    .label = "temp3 DIMM VRD"
    },
    {
    .reg = CORE_VRD_TEMP,
    .label = "temp4 CORE VRD"
    },
    {
    .reg = CH0_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp5 CH0 DIMM"
    },
    {
    .reg = CH1_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp6 CH1 DIMM"
    },
    {
    .reg = CH2_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp7 CH2 DIMM"
    },
    {
    .reg = CH3_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp8 CH3 DIMM"
    },
    {
    .reg = CH4_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp9 CH4 DIMM"
    },
    {
    .reg = CH5_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp10 CH5 DIMM"
    },
    {
    .reg = CH6_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp11 CH6 DIMM"
    },
    {
    .reg = CH7_DIMM_TEMP,
    .reg_ext = MEM_HOT_THRESHOLD,
    .label = "temp12 CH7 DIMM"
    },
    {
    .reg = RCA_VRD_TEMP,
    .label = "temp13 RCA VRD"
    },
    };
    static const struct smpro_sensor voltage[] = {
    {
    .reg = CORE_VRD_VOLT,
    .label = "vout0 CORE VRD"
    },
    {
    .reg = SOC_VRD_VOLT,
    .label = "vout1 SoC VRD"
    },
    {
    .reg = DIMM_VRD1_VOLT,
    .label = "vout2 DIMM VRD1"
    },
    {
    .reg = DIMM_VRD2_VOLT,
    .label = "vout3 DIMM VRD2"
    },
    {
    .reg = RCA_VRD_VOLT,
    .label = "vout4 RCA VRD"
    },
    };
    static const struct smpro_sensor curr_sensor[] = {
    {
    .reg = CORE_VRD_CURR,
    .label = "iout1 CORE VRD"
    },
    {
    .reg = SOC_VRD_CURR,
    .label = "iout2 SoC VRD"
    },
    {
    .reg = DIMM_VRD1_CURR,
    .label = "iout3 DIMM VRD1"
    },
    {
    .reg = DIMM_VRD2_CURR,
    .label = "iout4 DIMM VRD2"
    },
    {
    .reg = RCA_VRD_CURR,
    .label = "iout5 RCA VRD"
    },
    };
    static const struct smpro_sensor power[] = {
    {
    .reg = CORE_VRD_PWR,
    .reg_ext = CORE_VRD_PWR_MW,
    .label = "power1 CORE VRD"
    },
    {
    .reg = SOC_PWR,
    .reg_ext = SOC_PWR_MW,
    .label = "power2 SoC"
    },
    {
    .reg = DIMM_VRD1_PWR,
    .reg_ext = DIMM_VRD1_PWR_MW,
    .label = "power3 DIMM VRD1"
    },
    {
    .reg = DIMM_VRD2_PWR,
    .reg_ext = DIMM_VRD2_PWR_MW,
    .label = "power4 DIMM VRD2"
    },
    {
    .reg = RCA_VRD_PWR,
    .reg_ext = RCA_VRD_PWR_MW,
    .label = "power5 RCA VRD"
    },
    };
#[no_mangle]
unsafe extern "C" fn smpro_read_temp(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int smpro_read_temp(struct device *dev, u32 attr, int channel, long *val)
    {
    struct smpro_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int value;
    int ret;
    switch (attr) {
    case hwmon_temp_input:
    ret = regmap_read(hwmon.regmap, temperature[channel].reg, &value);
    if (ret)
    return ret;
    break;
    case hwmon_temp_crit:
    ret = regmap_read(hwmon.regmap, temperature[channel].reg_ext, &value);
    if (ret)
    return ret;
    break;
    default:
    return -EOPNOTSUPP;
    }
// val = sign_extend32(value, 8) * 1000;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smpro_read_in(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int smpro_read_in(struct device *dev, u32 attr, int channel, long *val)
    {
    struct smpro_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int value;
    int ret;
    switch (attr) {
    case hwmon_in_input:
    ret = regmap_read(hwmon.regmap, voltage[channel].reg, &value);
    if (ret < 0)
    return ret;
// 15-bit value in 1mV
// val = value & 0x7fff;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn smpro_read_curr(dev: *mut device, attr: u32, channel: c_int, val: *mut c_long) -> c_int {
    static int smpro_read_curr(struct device *dev, u32 attr, int channel, long *val)
    {
    struct smpro_hwmon *hwmon = dev_get_drvdata(dev);
    unsigned int value;
    int ret;
    switch (attr) {
    case hwmon_curr_input:
    ret = regmap_read(hwmon.regmap, curr_sensor[channel].reg, &value);
    if (ret < 0)
    return ret;
// Scale reported by the hardware is 1mA
// val = value & 0x7fff;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
#[no_mangle]
unsafe extern "C" fn smpro_read_power(dev: *mut device, attr: u32, channel: c_int, val_pwr: *mut c_long) -> c_int {
    static int smpro_read_power(struct device *dev, u32 attr, int channel, long *val_pwr)
    {
    struct smpro_hwmon *hwmon = dev_get_drvdata(dev);
    let mut val: c_uint = 0, val_mw = 0;
    int ret;
    switch (attr) {
    case hwmon_power_input:
    ret = regmap_read(hwmon.regmap, power[channel].reg, &val);
    if (ret)
    return ret;
    ret = regmap_read(hwmon.regmap, power[channel].reg_ext, &val_mw);
    if (ret)
    return ret;
// 10-bit value
// val_pwr = (val & 0x3ff) * 1000000 + (val_mw & 0x3ff) * 1000;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static int smpro_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    switch (type) {
    case hwmon_temp:
    return smpro_read_temp(dev, attr, channel, val);
    case hwmon_in:
    return smpro_read_in(dev, attr, channel, val);
    case hwmon_power:
    return smpro_read_power(dev, attr, channel, val);
    case hwmon_curr:
    return smpro_read_curr(dev, attr, channel, val);
    default:
    return -EOPNOTSUPP;
    }
    }
    static int smpro_read_string(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, const char **str)
    {
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_label:
// str = temperature[channel].label;
    return 0;
    default:
    break;
    }
    break;
    case hwmon_in:
    switch (attr) {
    case hwmon_in_label:
// str = voltage[channel].label;
    return 0;
    default:
    break;
    }
    break;
    case hwmon_curr:
    switch (attr) {
    case hwmon_curr_label:
// str = curr_sensor[channel].label;
    return 0;
    default:
    break;
    }
    break;
    case hwmon_power:
    switch (attr) {
    case hwmon_power_label:
// str = power[channel].label;
    return 0;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static umode_t smpro_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct smpro_hwmon *hwmon = data;
    unsigned int value;
    int ret;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
    case hwmon_temp_label:
    case hwmon_temp_crit:
    ret = regmap_read(hwmon.regmap, temperature[channel].reg, &value);
    if (ret || value == 0xFFFF)
    return 0;
    break;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return 0444;
    }
    static const struct hwmon_channel_info * const smpro_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL | HWMON_T_CRIT,
    HWMON_T_INPUT | HWMON_T_LABEL),
    HWMON_CHANNEL_INFO(in,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL,
    HWMON_I_INPUT | HWMON_I_LABEL),
    HWMON_CHANNEL_INFO(power,
    HWMON_P_INPUT | HWMON_P_LABEL,
    HWMON_P_INPUT | HWMON_P_LABEL,
    HWMON_P_INPUT | HWMON_P_LABEL,
    HWMON_P_INPUT | HWMON_P_LABEL,
    HWMON_P_INPUT | HWMON_P_LABEL),
    HWMON_CHANNEL_INFO(curr,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL,
    HWMON_C_INPUT | HWMON_C_LABEL),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops smpro_hwmon_ops = {
    .is_visible = smpro_is_visible,
    .read = smpro_read,
    .read_string = smpro_read_string,
    };
    static const struct hwmon_chip_info smpro_chip_info = {
    .ops = &smpro_hwmon_ops,
    .info = smpro_info,
    };
#[no_mangle]
unsafe extern "C" fn smpro_hwmon_probe(pdev: *mut platform_device) -> c_int {
    static int smpro_hwmon_probe(struct platform_device *pdev)
    {
    struct smpro_hwmon *hwmon;
    struct device *hwmon_dev;
    hwmon = devm_kzalloc(&pdev.dev, sizeof(struct smpro_hwmon), GFP_KERNEL);
    if (!hwmon)
    return -ENOMEM;
    hwmon.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!hwmon.regmap)
    return -ENODEV;
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev, "smpro_hwmon",
    hwmon, &smpro_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct platform_driver smpro_hwmon_driver = {
    .probe		= smpro_hwmon_probe,
    .driver = {
    .name	= "smpro-hwmon",
    },
    };
    module_platform_driver(smpro_hwmon_driver);
    MODULE_AUTHOR("Thu Nguyen <thu@os.amperecomputing.com>");
    MODULE_AUTHOR("Quan Nguyen <quan@os.amperecomputing.com>");
    MODULE_DESCRIPTION("Ampere Altra SMPro hwmon driver");
    MODULE_LICENSE("GPL");
