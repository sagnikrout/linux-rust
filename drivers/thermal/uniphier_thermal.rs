//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/uniphier_thermal.c
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
// uniphier_thermal.c - Socionext UniPhier thermal driver
// Copyright 2014      Panasonic Corporation
// Copyright 2016-2017 Socionext Inc.
// Author:
// Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

//
// block registers
// addresses are the offset from .block_base
//
pub const PVTCTLEN: c_uint = 0x0000;

pub const PVTCTLMODE: c_uint = 0x0004;
pub const PVTCTLMODE_MASK: c_uint = 0xf;
pub const PVTCTLMODE_TEMPMON: c_uint = 0x5;
pub const EMONREPEAT: c_uint = 0x0040;

pub const EMONREPEAT_PERIOD_1000000: c_uint = 0x9;
//
// common registers
// addresses are the offset from .map_base
//
pub const PVTCTLSEL: c_uint = 0x0900;

pub const PVTCTLSEL_MONITOR: c_int = 0;
pub const SETALERT0: c_uint = 0x0910;
pub const SETALERT1: c_uint = 0x0914;
pub const SETALERT2: c_uint = 0x0918;

pub const PMALERTINTCTL: c_uint = 0x0920;

    GENMASK(2, 0))
pub const TMOD: c_uint = 0x0928;
pub const TMOD_WIDTH: c_int = 9;
pub const TMODCOEF: c_uint = 0x0e5c;

// SoC critical temperature

// Max # of alert channels
pub const ALERT_CH_NUM: c_int = 3;
// SoC specific thermal sensor data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_tm_soc_data {
    pub map_base: u32,
    pub block_base: u32,
    pub tmod_setup_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_tm_dev {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub alert_en: [bool; ALERT_CH_NUM],
    pub tz_dev: *mut thermal_zone_device,
    pub data: *const uniphier_tm_soc_data,
}

#[no_mangle]
unsafe extern "C" fn uniphier_tm_initialize_sensor(tdev: *mut uniphier_tm_dev) -> c_int {
    static int uniphier_tm_initialize_sensor(struct uniphier_tm_dev *tdev)
    {
    struct regmap *map = tdev.regmap;
    u32 val;
    u32 tmod_calib[2];
    int ret;
// stop PVT
    regmap_write_bits(map, tdev.data.block_base + PVTCTLEN,
    PVTCTLEN_EN, 0);
//
// Since SoC has a calibrated value that was set in advance,
// TMODCOEF shows non-zero and PVT refers the value internally.
//
// If TMODCOEF shows zero, the boards don't have the calibrated
// value, and the driver has to set default value from DT.
//
    ret = regmap_read(map, tdev.data.map_base + TMODCOEF, &val);
    if (ret)
    return ret;
    if (!val) {
// look for the default values in DT
    ret = of_property_read_u32_array(tdev.dev.of_node,
    "socionext,tmod-calibration",
    tmod_calib,
    ARRAY_SIZE(tmod_calib));
    if (ret)
    return ret;
    regmap_write(map, tdev.data.tmod_setup_addr,
    TMODSETUP0_EN | TMODSETUP0_VAL(tmod_calib[0]) |
    TMODSETUP1_EN | TMODSETUP1_VAL(tmod_calib[1]));
    }
// select temperature mode
    regmap_write_bits(map, tdev.data.block_base + PVTCTLMODE,
    PVTCTLMODE_MASK, PVTCTLMODE_TEMPMON);
// set monitoring period
    regmap_write_bits(map, tdev.data.block_base + EMONREPEAT,
    EMONREPEAT_ENDLESS | EMONREPEAT_PERIOD,
    EMONREPEAT_ENDLESS | EMONREPEAT_PERIOD_1000000);
// set monitor mode
    regmap_write_bits(map, tdev.data.map_base + PVTCTLSEL,
    PVTCTLSEL_MASK, PVTCTLSEL_MONITOR);
    return 0;
    }
    static void uniphier_tm_set_alert(struct uniphier_tm_dev *tdev, u32 ch,
    u32 temp)
    {
    struct regmap *map = tdev.regmap;
// set alert temperature
    regmap_write_bits(map, tdev.data.map_base + SETALERT0 + (ch << 2),
    SETALERT_EN | SETALERT_TEMP_OVF,
    SETALERT_EN |
    SETALERT_TEMP_OVF_VALUE(temp / 1000));
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_enable_sensor(tdev: *mut uniphier_tm_dev) {
    static void uniphier_tm_enable_sensor(struct uniphier_tm_dev *tdev)
    {
    struct regmap *map = tdev.regmap;
    int i;
    let mut bits: u32 = 0;
    for (i = 0; i < ALERT_CH_NUM; i++)
    if (tdev.alert_en[i])
    bits |= PMALERTINTCTL_EN(i);
// enable alert interrupt
    regmap_write_bits(map, tdev.data.map_base + PMALERTINTCTL,
    PMALERTINTCTL_MASK, bits);
// start PVT
    regmap_write_bits(map, tdev.data.block_base + PVTCTLEN,
    PVTCTLEN_EN, PVTCTLEN_EN);
    usleep_range(700, 1500);	/* The spec note says at least 700us */
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_disable_sensor(tdev: *mut uniphier_tm_dev) {
    static void uniphier_tm_disable_sensor(struct uniphier_tm_dev *tdev)
    {
    struct regmap *map = tdev.regmap;
// disable alert interrupt
    regmap_write_bits(map, tdev.data.map_base + PMALERTINTCTL,
    PMALERTINTCTL_MASK, 0);
// stop PVT
    regmap_write_bits(map, tdev.data.block_base + PVTCTLEN,
    PVTCTLEN_EN, 0);
    usleep_range(1000, 2000);	/* The spec note says at least 1ms */
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_get_temp(tz: *mut thermal_zone_device, out_temp: *mut c_int) -> c_int {
    static int uniphier_tm_get_temp(struct thermal_zone_device *tz, int *out_temp)
    {
    struct uniphier_tm_dev *tdev = thermal_zone_device_priv(tz);
    struct regmap *map = tdev.regmap;
    int ret;
    u32 temp;
    ret = regmap_read(map, tdev.data.map_base + TMOD, &temp);
    if (ret)
    return ret;
// MSB of the TMOD field is a sign bit
// out_temp = sign_extend32(temp, TMOD_WIDTH - 1) * 1000;
    return 0;
    }
    static const struct thermal_zone_device_ops uniphier_of_thermal_ops = {
    .get_temp = uniphier_tm_get_temp,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_tm_irq_clear(tdev: *mut uniphier_tm_dev) {
    static void uniphier_tm_irq_clear(struct uniphier_tm_dev *tdev)
    {
    let mut mask: u32 = 0, bits = 0;
    int i;
    for (i = 0; i < ALERT_CH_NUM; i++) {
    mask |= (PMALERTINTCTL_CLR(i) | PMALERTINTCTL_SET(i));
    bits |= PMALERTINTCTL_CLR(i);
    }
// clear alert interrupt
    regmap_write_bits(tdev.regmap,
    tdev.data.map_base + PMALERTINTCTL, mask, bits);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_alarm_irq(irq: c_int, _tdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_tm_alarm_irq(int irq, void *_tdev)
    {
    struct uniphier_tm_dev *tdev = _tdev;
    disable_irq_nosync(irq);
    uniphier_tm_irq_clear(tdev);
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_alarm_irq_thread(irq: c_int, _tdev: *mut c_void) -> irqreturn_t {
    static irqreturn_t uniphier_tm_alarm_irq_thread(int irq, void *_tdev)
    {
    struct uniphier_tm_dev *tdev = _tdev;
    thermal_zone_device_update(tdev.tz_dev, THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trip_walk_data {
    pub tdev: *mut uniphier_tm_dev,
    pub crit_temp: c_int,
    pub index: c_int,
}

#[no_mangle]
unsafe extern "C" fn uniphier_tm_trip_walk_cb(trip: *mut thermal_trip, arg: *mut c_void) -> c_int {
    static int uniphier_tm_trip_walk_cb(struct thermal_trip *trip, void *arg)
    {
    struct trip_walk_data *twd = arg;
    if (trip.type == THERMAL_TRIP_CRITICAL &&
    trip.temperature < twd.crit_temp)
    twd.crit_temp = trip.temperature;
    uniphier_tm_set_alert(twd.tdev, twd.index, trip.temperature);
    twd.tdev.alert_en[twd.index++] = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_tm_probe(struct platform_device *pdev)
    {
    let mut twd: trip_walk_data = { .crit_temp = INT_MAX, .index = 0 };
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    struct device_node *parent;
    struct uniphier_tm_dev *tdev;
    int ret, irq;
    tdev = devm_kzalloc(dev, sizeof(*tdev), GFP_KERNEL);
    if (!tdev)
    return -ENOMEM;
    tdev.dev = dev;
    tdev.data = of_device_get_match_data(dev);
    if (WARN_ON(!tdev.data))
    return -EINVAL;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
// get regmap from syscon node
    parent = of_get_parent(dev.of_node); /* parent should be syscon node */
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to get regmap (error %ld)\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    tdev.regmap = regmap;
    ret = uniphier_tm_initialize_sensor(tdev);
    if (ret) {
    dev_err(dev, "failed to initialize sensor\n");
    return ret;
    }
    ret = devm_request_threaded_irq(dev, irq, uniphier_tm_alarm_irq,
    uniphier_tm_alarm_irq_thread,
    0, "thermal", tdev);
    if (ret)
    return ret;
    platform_set_drvdata(pdev, tdev);
    tdev.tz_dev = devm_thermal_of_zone_register(dev, 0, tdev,
    &uniphier_of_thermal_ops);
    if (IS_ERR(tdev.tz_dev)) {
    dev_err(dev, "failed to register sensor device\n");
    return PTR_ERR(tdev.tz_dev);
    }
// set alert temperatures
    twd.tdev = tdev;
    thermal_zone_for_each_trip(tdev.tz_dev, uniphier_tm_trip_walk_cb, &twd);
    if (twd.crit_temp > CRITICAL_TEMP_LIMIT) {
    dev_err(dev, "critical trip is over limit(>%d), or not set\n",
    CRITICAL_TEMP_LIMIT);
    return -EINVAL;
    }
    uniphier_tm_enable_sensor(tdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_tm_remove(pdev: *mut platform_device) {
    static void uniphier_tm_remove(struct platform_device *pdev)
    {
    struct uniphier_tm_dev *tdev = platform_get_drvdata(pdev);
// disable sensor
    uniphier_tm_disable_sensor(tdev);
    }
    static const struct uniphier_tm_soc_data uniphier_pxs2_tm_data = {
    .map_base        = 0xe000,
    .block_base      = 0xe000,
    .tmod_setup_addr = 0xe904,
    };
    static const struct uniphier_tm_soc_data uniphier_ld20_tm_data = {
    .map_base        = 0xe000,
    .block_base      = 0xe800,
    .tmod_setup_addr = 0xe938,
    };
    static const struct of_device_id uniphier_tm_dt_ids[] = {
    {
    .compatible = "socionext,uniphier-pxs2-thermal",
    .data       = &uniphier_pxs2_tm_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-thermal",
    .data       = &uniphier_ld20_tm_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-thermal",
    .data       = &uniphier_ld20_tm_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-thermal",
    .data       = &uniphier_ld20_tm_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_tm_dt_ids);
    static struct platform_driver uniphier_tm_driver = {
    .probe = uniphier_tm_probe,
    .remove = uniphier_tm_remove,
    .driver = {
    .name = "uniphier-thermal",
    .of_match_table = uniphier_tm_dt_ids,
    },
    };
    module_platform_driver(uniphier_tm_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier thermal driver");
    MODULE_LICENSE("GPL v2");
