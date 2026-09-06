//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/broadcom/bcm2835_thermal.c
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
// Driver for Broadcom BCM2835 SoC temperature sensor
//
// Copyright (C) 2016 Martin Sperl
//

pub const BCM2835_TS_TSENSCTL: c_uint = 0x00;
pub const BCM2835_TS_TSENSSTAT: c_uint = 0x04;

//
// bandgap reference voltage in 6 mV increments
// 000b = 1178 mV, 001b = 1184 mV, ... 111b = 1220 mV
//
pub const BCM2835_TS_TSENSCTL_CTRL_BITS: c_int = 3;
pub const BCM2835_TS_TSENSCTL_CTRL_SHIFT: c_int = 2;

    GENMASK(BCM2835_TS_TSENSCTL_CTRL_BITS +     \
    BCM2835_TS_TSENSCTL_CTRL_SHIFT - 1, \
    BCM2835_TS_TSENSCTL_CTRL_SHIFT)
pub const BCM2835_TS_TSENSCTL_CTRL_DEFAULT: c_int = 1;

pub const BCM2835_TS_TSENSCTL_THOLD_SHIFT: c_int = 8;
pub const BCM2835_TS_TSENSCTL_THOLD_BITS: c_int = 10;

    GENMASK(BCM2835_TS_TSENSCTL_THOLD_BITS +     \
    BCM2835_TS_TSENSCTL_THOLD_SHIFT - 1, \
    BCM2835_TS_TSENSCTL_THOLD_SHIFT)
//
// time how long the block to be asserted in reset
// which based on a clock counter (TSENS clock assumed)
//
pub const BCM2835_TS_TSENSCTL_RSTDELAY_SHIFT: c_int = 18;
pub const BCM2835_TS_TSENSCTL_RSTDELAY_BITS: c_int = 8;

pub const BCM2835_TS_TSENSSTAT_DATA_BITS: c_int = 10;
pub const BCM2835_TS_TSENSSTAT_DATA_SHIFT: c_int = 0;

    GENMASK(BCM2835_TS_TSENSSTAT_DATA_BITS +     \
    BCM2835_TS_TSENSSTAT_DATA_SHIFT - 1, \
    BCM2835_TS_TSENSSTAT_DATA_SHIFT)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_thermal_data {
    pub tz: *mut thermal_zone_device,
    pub regs: *mut void __iomem,
    pub clk: *mut clk,
    pub debugfsdir: *mut dentry,
}

#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_adc2temp(adc: u32, offset: c_int, slope: c_int) -> c_int {
    static int bcm2835_thermal_adc2temp(u32 adc, int offset, int slope)
    {
    return offset + slope * adc;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_temp2adc(temp: c_int, offset: c_int, slope: c_int) -> c_int {
    static int bcm2835_thermal_temp2adc(int temp, int offset, int slope)
    {
    temp -= offset;
    temp /= slope;
    return clamp(temp, 0, (int)BIT(BCM2835_TS_TSENSSTAT_DATA_BITS) - 1);
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int bcm2835_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct bcm2835_thermal_data *data = thermal_zone_device_priv(tz);
    let mut val: u32 = readl(data.regs + BCM2835_TS_TSENSSTAT);
    if (!(val & BCM2835_TS_TSENSSTAT_VALID))
    return -EIO;
    val &= BCM2835_TS_TSENSSTAT_DATA_MASK;
// temp = bcm2835_thermal_adc2temp(
    val,
    thermal_zone_get_offset(data.tz),
    thermal_zone_get_slope(data.tz));
    return 0;
    }
    static const struct debugfs_reg32 bcm2835_thermal_regs[] = {
    {
    .name = "ctl",
    .offset = 0
    },
    {
    .name = "stat",
    .offset = 4
    }
    };
#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_debugfs(pdev: *mut platform_device) {
    static void bcm2835_thermal_debugfs(struct platform_device *pdev)
    {
    struct bcm2835_thermal_data *data = platform_get_drvdata(pdev);
    struct debugfs_regset32 *regset;
    data.debugfsdir = debugfs_create_dir("bcm2835_thermal", core::ptr::null_mut());
    regset = devm_kzalloc(&pdev.dev, sizeof(*regset), GFP_KERNEL);
    if (!regset)
    return;
    regset.regs = bcm2835_thermal_regs;
    regset.nregs = ARRAY_SIZE(bcm2835_thermal_regs);
    regset.base = data.regs;
    debugfs_create_regset32("regset", 0444, data.debugfsdir, regset);
    }
    static const struct thermal_zone_device_ops bcm2835_thermal_ops = {
    .get_temp = bcm2835_thermal_get_temp,
    };
//
// Note: as per Raspberry Foundation FAQ
// (https://www.raspberrypi.org/help/faqs/#performanceOperatingTemperature)
// the recommended temperature range for the SoC -40C to +85C
// so the trip limit is set to 80C.
// this applies to all the BCM283X SoC
//
    static const struct of_device_id bcm2835_thermal_of_match_table[] = {
    {
    .compatible = "brcm,bcm2835-thermal",
    },
    {
    .compatible = "brcm,bcm2836-thermal",
    },
    {
    .compatible = "brcm,bcm2837-thermal",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm2835_thermal_of_match_table);
#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int bcm2835_thermal_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct of_device_id *match;
    struct thermal_zone_device *tz;
    struct bcm2835_thermal_data *data;
    let mut err: c_int = 0;
    u32 val;
    unsigned long rate;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    match = of_match_device(bcm2835_thermal_of_match_table, dev);
    if (!match)
    return -EINVAL;
    data.regs = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(data.regs)) {
    err = PTR_ERR(data.regs);
    return err;
    }
    data.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(data.clk))
    return dev_err_probe(dev, PTR_ERR(data.clk), "Could not get clk\n");
    rate = clk_get_rate(data.clk);
    if ((rate < 1920000) || (rate > 5000000))
    dev_warn(dev,
    "Clock %pC running at %lu Hz is outside of the recommended range: 1.92 to 5MHz\n",
    data.clk, rate);
// register of thermal sensor and get info from DT
    tz = devm_thermal_of_zone_register(dev, 0, data, &bcm2835_thermal_ops);
    if (IS_ERR(tz))
    return dev_err_probe(dev, PTR_ERR(tz), "Failed to register the thermal device\n");
//
// right now the FW does set up the HW-block, so we are not
// touching the configuration registers.
// But if the HW is not enabled, then set it up
// using "sane" values used by the firmware right now.
//
    val = readl(data.regs + BCM2835_TS_TSENSCTL);
    if (!(val & BCM2835_TS_TSENSCTL_RSTB)) {
    int offset, slope, crit_temp;
    slope = thermal_zone_get_slope(tz);
    offset = thermal_zone_get_offset(tz);
//
// For now we deal only with critical, otherwise
// would need to iterate
//
    err = thermal_zone_get_crit_temp(tz, &crit_temp);
    if (err < 0) {
    dev_err(dev, "Not able to read trip_temp: %d\n", err);
    return err;
    }
// set bandgap reference voltage and enable voltage regulator
    val = (BCM2835_TS_TSENSCTL_CTRL_DEFAULT <<
    BCM2835_TS_TSENSCTL_CTRL_SHIFT) |
    BCM2835_TS_TSENSCTL_REGULEN;
// use the recommended reset duration
    val |= (0xFE << BCM2835_TS_TSENSCTL_RSTDELAY_SHIFT);
// trip_adc value from info
    val |= bcm2835_thermal_temp2adc(crit_temp,
    offset,
    slope)
    << BCM2835_TS_TSENSCTL_THOLD_SHIFT;
// write the value back to the register as 2 steps
    writel(val, data.regs + BCM2835_TS_TSENSCTL);
    val |= BCM2835_TS_TSENSCTL_RSTB;
    writel(val, data.regs + BCM2835_TS_TSENSCTL);
    }
    data.tz = tz;
    platform_set_drvdata(pdev, data);
//
// Thermal_zone doesn't enable hwmon as default,
// enable it here
//
    err = thermal_add_hwmon_sysfs(tz);
    if (err)
    return err;
    bcm2835_thermal_debugfs(pdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_thermal_remove(pdev: *mut platform_device) {
    static void bcm2835_thermal_remove(struct platform_device *pdev)
    {
    struct bcm2835_thermal_data *data = platform_get_drvdata(pdev);
    debugfs_remove_recursive(data.debugfsdir);
    }
    static struct platform_driver bcm2835_thermal_driver = {
    .probe = bcm2835_thermal_probe,
    .remove = bcm2835_thermal_remove,
    .driver = {
    .name = "bcm2835_thermal",
    .of_match_table = bcm2835_thermal_of_match_table,
    },
    };
    module_platform_driver(bcm2835_thermal_driver);
    MODULE_AUTHOR("Martin Sperl");
    MODULE_DESCRIPTION("Thermal driver for bcm2835 chip");
    MODULE_LICENSE("GPL");
