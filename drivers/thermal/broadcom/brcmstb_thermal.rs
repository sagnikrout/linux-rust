//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/broadcom/brcmstb_thermal.c
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
// Broadcom STB AVS TMON thermal sensor driver
//
// Copyright (c) 2015-2017 Broadcom
//

pub const AVS_TMON_STATUS: c_uint = 0x00;

pub const AVS_TMON_STATUS_data_shift: c_int = 1;
pub const AVS_TMON_EN_OVERTEMP_RESET: c_uint = 0x04;

pub const AVS_TMON_RESET_THRESH: c_uint = 0x08;

pub const AVS_TMON_RESET_THRESH_shift: c_int = 1;
pub const AVS_TMON_INT_IDLE_TIME: c_uint = 0x10;
pub const AVS_TMON_EN_TEMP_INT_SRCS: c_uint = 0x14;

pub const AVS_TMON_INT_THRESH: c_uint = 0x18;

pub const AVS_TMON_INT_THRESH_high_shift: c_int = 17;

pub const AVS_TMON_INT_THRESH_low_shift: c_int = 1;
pub const AVS_TMON_TEMP_INT_CODE: c_uint = 0x1c;
pub const AVS_TMON_TP_TEST_ENABLE: c_uint = 0x20;
// Default coefficients
pub const AVS_TMON_TEMP_SLOPE: c_int = 487;
pub const AVS_TMON_TEMP_OFFSET: c_int = 410040;
// HW related temperature constants
pub const AVS_TMON_TEMP_MAX: c_uint = 0x3ff;

    enum avs_tmon_trip_type {
    TMON_TRIP_TYPE_LOW = 0,
    TMON_TRIP_TYPE_HIGH,
    TMON_TRIP_TYPE_RESET,
    TMON_TRIP_TYPE_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_tmon_trip {
// HW bit to enable the trip
    pub enable_offs: u32,
    pub enable_mask: u32,
// HW field to read the trip temperature
    pub reg_offs: u32,
    pub reg_msk: u32,
    pub reg_shift: c_int,
}

    static struct avs_tmon_trip avs_tmon_trips[] = {
// Trips when temperature is below threshold
    [TMON_TRIP_TYPE_LOW] = {
    .enable_offs	= AVS_TMON_EN_TEMP_INT_SRCS,
    .enable_mask	= AVS_TMON_EN_TEMP_INT_SRCS_low,
    .reg_offs	= AVS_TMON_INT_THRESH,
    .reg_msk	= AVS_TMON_INT_THRESH_low_msk,
    .reg_shift	= AVS_TMON_INT_THRESH_low_shift,
    },
// Trips when temperature is above threshold
    [TMON_TRIP_TYPE_HIGH] = {
    .enable_offs	= AVS_TMON_EN_TEMP_INT_SRCS,
    .enable_mask	= AVS_TMON_EN_TEMP_INT_SRCS_high,
    .reg_offs	= AVS_TMON_INT_THRESH,
    .reg_msk	= AVS_TMON_INT_THRESH_high_msk,
    .reg_shift	= AVS_TMON_INT_THRESH_high_shift,
    },
// Automatically resets chip when above threshold
    [TMON_TRIP_TYPE_RESET] = {
    .enable_offs	= AVS_TMON_EN_OVERTEMP_RESET,
    .enable_mask	= AVS_TMON_EN_OVERTEMP_RESET_msk,
    .reg_offs	= AVS_TMON_RESET_THRESH,
    .reg_msk	= AVS_TMON_RESET_THRESH_msk,
    .reg_shift	= AVS_TMON_RESET_THRESH_shift,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_thermal_params {
    pub offset: c_uint,
    pub mult: c_uint,
    pub of_ops: *const thermal_zone_device_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmstb_thermal_priv {
    pub tmon_base: *mut void __iomem,
    pub dev: *mut device,
    pub thermal: *mut thermal_zone_device,
// Process specific thermal parameters used for calculations
    pub temp_params: *const brcmstb_thermal_params,
}

// Convert a HW code to a temperature reading (millidegree celsius)
    static inline int avs_tmon_code_to_temp(struct brcmstb_thermal_priv *priv,
    u32 code)
    {
    let mut offset: c_int = priv.temp_params.offset;
    let mut mult: c_int = priv.temp_params.mult;
    return (offset - (int)((code & AVS_TMON_TEMP_MASK) * mult));
    }
//
// Convert a temperature value (millidegree celsius) to a HW code
//
// @temp: temperature to convert
// @low: if true, round toward the low side
//
    static inline u32 avs_tmon_temp_to_code(struct brcmstb_thermal_priv *priv,
    int temp, bool low)
    {
    let mut offset: c_int = priv.temp_params.offset;
    let mut mult: c_int = priv.temp_params.mult;
    if (temp < AVS_TMON_TEMP_MIN)
    return AVS_TMON_TEMP_MAX;	/* Maximum code value */
    if (temp >= offset)
    return 0;	/* Minimum code value */
    if (low)
    return (u32)(DIV_ROUND_UP(offset - temp, mult));
    else
    return (u32)((offset - temp) / mult);
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int brcmstb_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct brcmstb_thermal_priv *priv = thermal_zone_device_priv(tz);
    u32 val;
    int t;
    val = __raw_readl(priv.tmon_base + AVS_TMON_STATUS);
    if (!(val & AVS_TMON_STATUS_valid_msk))
    return -EIO;
    val = (val & AVS_TMON_STATUS_data_msk) >> AVS_TMON_STATUS_data_shift;
    t = avs_tmon_code_to_temp(priv, val);
// temp = max(0, t);
    return 0;
    }
    static void avs_tmon_trip_enable(struct brcmstb_thermal_priv *priv,
    enum avs_tmon_trip_type type, int en)
    {
    struct avs_tmon_trip *trip = &avs_tmon_trips[type];
    let mut val: u32 = __raw_readl(priv.tmon_base + trip.enable_offs);
    dev_dbg(priv.dev, "%sable trip, type %d\n", en ? "en" : "dis", type);
    if (en)
    val |= trip.enable_mask;
    else
    val &= ~trip.enable_mask;
    __raw_writel(val, priv.tmon_base + trip.enable_offs);
    }
    static int avs_tmon_get_trip_temp(struct brcmstb_thermal_priv *priv,
    enum avs_tmon_trip_type type)
    {
    struct avs_tmon_trip *trip = &avs_tmon_trips[type];
    let mut val: u32 = __raw_readl(priv.tmon_base + trip.reg_offs);
    val &= trip.reg_msk;
    val >>= trip.reg_shift;
    return avs_tmon_code_to_temp(priv, val);
    }
    static void avs_tmon_set_trip_temp(struct brcmstb_thermal_priv *priv,
    enum avs_tmon_trip_type type,
    int temp)
    {
    struct avs_tmon_trip *trip = &avs_tmon_trips[type];
    u32 val, orig;
    dev_dbg(priv.dev, "set temp %d to %d\n", type, temp);
// round toward low temp for the low interrupt
    val = avs_tmon_temp_to_code(priv, temp,
    type == TMON_TRIP_TYPE_LOW);
    val <<= trip.reg_shift;
    val &= trip.reg_msk;
    orig = __raw_readl(priv.tmon_base + trip.reg_offs);
    orig &= ~trip.reg_msk;
    orig |= val;
    __raw_writel(orig, priv.tmon_base + trip.reg_offs);
    }
#[no_mangle]
unsafe extern "C" fn avs_tmon_get_intr_temp(priv: *mut brcmstb_thermal_priv) -> c_int {
    static int avs_tmon_get_intr_temp(struct brcmstb_thermal_priv *priv)
    {
    u32 val;
    val = __raw_readl(priv.tmon_base + AVS_TMON_TEMP_INT_CODE);
    return avs_tmon_code_to_temp(priv, val);
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_tmon_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t brcmstb_tmon_irq_thread(int irq, void *data)
    {
    struct brcmstb_thermal_priv *priv = data;
    int low, high, intr;
    low = avs_tmon_get_trip_temp(priv, TMON_TRIP_TYPE_LOW);
    high = avs_tmon_get_trip_temp(priv, TMON_TRIP_TYPE_HIGH);
    intr = avs_tmon_get_intr_temp(priv);
    dev_dbg(priv.dev, "low/intr/high: %d/%d/%d\n",
    low, intr, high);
// Disable high-temp until next threshold shift
    if (intr >= high)
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_HIGH, 0);
// Disable low-temp until next threshold shift
    if (intr <= low)
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_LOW, 0);
//
// Notify using the interrupt temperature, in case the temperature
// changes before it can next be read out
//
    thermal_zone_device_update(priv.thermal, intr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn brcmstb_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int) -> c_int {
    static int brcmstb_set_trips(struct thermal_zone_device *tz, int low, int high)
    {
    struct brcmstb_thermal_priv *priv = thermal_zone_device_priv(tz);
    dev_dbg(priv.dev, "set trips %d <-. %d\n", low, high);
//
// Disable low-temp if "low" is too small. As per thermal framework
// API, we use -INT_MAX rather than INT_MIN.
//
    if (low <= -INT_MAX) {
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_LOW, 0);
    } else {
    avs_tmon_set_trip_temp(priv, TMON_TRIP_TYPE_LOW, low);
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_LOW, 1);
    }
// Disable high-temp if "high" is too big.
    if (high == INT_MAX) {
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_HIGH, 0);
    } else {
    avs_tmon_set_trip_temp(priv, TMON_TRIP_TYPE_HIGH, high);
    avs_tmon_trip_enable(priv, TMON_TRIP_TYPE_HIGH, 1);
    }
    return 0;
    }
    static const struct thermal_zone_device_ops brcmstb_of_ops = {
    .get_temp	= brcmstb_get_temp,
    };
    static const struct brcmstb_thermal_params brcmstb_8nm_params = {
    .offset	= 418670,
    .mult	= 509,
    .of_ops	= &brcmstb_of_ops,
    };
    static const struct brcmstb_thermal_params brcmstb_16nm_params = {
    .offset	= 457829,
    .mult	= 557,
    .of_ops	= &brcmstb_of_ops,
    };
    static const struct thermal_zone_device_ops brcmstb_28nm_of_ops = {
    .get_temp	= brcmstb_get_temp,
    .set_trips	= brcmstb_set_trips,
    };
    static const struct brcmstb_thermal_params brcmstb_28nm_params = {
    .offset	= 410040,
    .mult	= 487,
    .of_ops	= &brcmstb_28nm_of_ops,
    };
    static const struct of_device_id brcmstb_thermal_id_table[] = {
    { .compatible = "brcm,avs-tmon-bcm74110", .data = &brcmstb_8nm_params },
    { .compatible = "brcm,avs-tmon-bcm7216", .data = &brcmstb_16nm_params },
    { .compatible = "brcm,avs-tmon", .data = &brcmstb_28nm_params },
    {},
    };
    MODULE_DEVICE_TABLE(of, brcmstb_thermal_id_table);
#[no_mangle]
unsafe extern "C" fn brcmstb_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int brcmstb_thermal_probe(struct platform_device *pdev)
    {
    const struct thermal_zone_device_ops *of_ops;
    struct thermal_zone_device *thermal;
    struct brcmstb_thermal_priv *priv;
    int irq, ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.temp_params = of_device_get_match_data(&pdev.dev);
    if (!priv.temp_params)
    return -EINVAL;
    priv.tmon_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(priv.tmon_base))
    return PTR_ERR(priv.tmon_base);
    priv.dev = &pdev.dev;
    of_ops = priv.temp_params.of_ops;
    thermal = devm_thermal_of_zone_register(&pdev.dev, 0, priv,
    of_ops);
    if (IS_ERR(thermal))
    return dev_err_probe(&pdev.dev, PTR_ERR(thermal),
    "could not register sensor\n");
    priv.thermal = thermal;
    irq = platform_get_irq_optional(pdev, 0);
    if (irq >= 0) {
    ret = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    brcmstb_tmon_irq_thread,
    IRQF_ONESHOT,
    DRV_NAME, priv);
    if (ret < 0)
    return ret;
    }
    dev_info(&pdev.dev, "registered AVS TMON of-sensor driver\n");
    return 0;
    }
    static struct platform_driver brcmstb_thermal_driver = {
    .probe = brcmstb_thermal_probe,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = brcmstb_thermal_id_table,
    },
    };
    module_platform_driver(brcmstb_thermal_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Brian Norris");
    MODULE_DESCRIPTION("Broadcom STB AVS TMON thermal driver");
