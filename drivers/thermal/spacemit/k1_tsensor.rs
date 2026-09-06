//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/spacemit/k1_tsensor.c
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
// Thermal sensor driver for SpacemiT K1 SoC
//
// Copyright (C) 2026 Shuwei Wu <shuwei.wu@mailbox.org>
//

pub const K1_TSENSOR_PCTRL_REG: c_uint = 0x00;

pub const K1_TSENSOR_EN_REG: c_uint = 0x08;

pub const K1_TSENSOR_TIME_REG: c_uint = 0x0C;

pub const K1_TSENSOR_INT_CLR_REG: c_uint = 0x10;
pub const K1_TSENSOR_INT_EN_REG: c_uint = 0x14;
pub const K1_TSENSOR_INT_STA_REG: c_uint = 0x18;

pub const K1_TSENSOR_DATA_BASE_REG: c_uint = 0x20;

pub const K1_TSENSOR_THRSH_BASE_REG: c_uint = 0x40;

pub const MAX_SENSOR_NUMBER: c_int = 5;
// Hardware offset value required for temperature calculation
pub const TEMPERATURE_OFFSET: c_int = 278;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct k1_tsensor_channel {
    pub ts: *mut k1_tsensor,
    pub tzd: *mut thermal_zone_device,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct k1_tsensor {
    pub base: *mut void __iomem,
    pub ch: [k1_tsensor_channel; MAX_SENSOR_NUMBER],
}

#[no_mangle]
unsafe extern "C" fn k1_tsensor_init(ts: *mut k1_tsensor) {
    static void k1_tsensor_init(struct k1_tsensor *ts)
    {
    u32 val;
// Disable all the interrupts
    writel(0xffffffff, ts.base + K1_TSENSOR_INT_EN_REG);
// Configure ADC sampling time and filter period
    val = readl(ts.base + K1_TSENSOR_TIME_REG);
    val &= ~K1_TSENSOR_TIME_MASK;
    val |= K1_TSENSOR_TIME_FILTER_PERIOD |
    K1_TSENSOR_TIME_ADC_CNT_RST |
    K1_TSENSOR_TIME_WAIT_REF_CNT;
    writel(val, ts.base + K1_TSENSOR_TIME_REG);
//
// Enable all sensors' auto mode, enable dither control,
// consecutive mode, and power up sensor.
//
    val = readl(ts.base + K1_TSENSOR_PCTRL_REG);
    val &= ~K1_TSENSOR_PCTRL_SW_CTRL;
    val &= ~K1_TSENSOR_PCTRL_CTUNE;
    val |= K1_TSENSOR_PCTRL_RAW_SEL |
    K1_TSENSOR_PCTRL_TEMP_MODE |
    K1_TSENSOR_PCTRL_HW_AUTO_MODE |
    K1_TSENSOR_PCTRL_ENABLE;
    writel(val, ts.base + K1_TSENSOR_PCTRL_REG);
// Enable each sensor
    val = readl(ts.base + K1_TSENSOR_EN_REG);
    val |= K1_TSENSOR_EN_ALL;
    writel(val, ts.base + K1_TSENSOR_EN_REG);
    }
#[no_mangle]
unsafe extern "C" fn k1_tsensor_enable_irq(ch: *mut k1_tsensor_channel) {
    static void k1_tsensor_enable_irq(struct k1_tsensor_channel *ch)
    {
    struct k1_tsensor *ts = ch.ts;
    u32 val;
    val = readl(ts.base + K1_TSENSOR_INT_CLR_REG);
    val |= K1_TSENSOR_INT_MASK(ch.id);
    writel(val, ts.base + K1_TSENSOR_INT_CLR_REG);
    val = readl(ts.base + K1_TSENSOR_INT_EN_REG);
    val &= ~K1_TSENSOR_INT_MASK(ch.id);
    writel(val, ts.base + K1_TSENSOR_INT_EN_REG);
// Enable thermal interrupt
    val = readl(ts.base + K1_TSENSOR_INT_EN_REG);
    val |= K1_TSENSOR_INT_EN_MASK;
    writel(val, ts.base + K1_TSENSOR_INT_EN_REG);
    }
//
// The conversion formula used is:
// T(m°C) = (((raw_value & mask) >> shift) - TEMPERATURE_OFFSET) * 1000
//
#[no_mangle]
unsafe extern "C" fn k1_tsensor_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int k1_tsensor_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct k1_tsensor_channel *ch = thermal_zone_device_priv(tz);
    struct k1_tsensor *ts = ch.ts;
    u32 val;
    val = readl(ts.base + K1_TSENSOR_DATA_REG(ch.id));
    if (ch.id % 2)
// temp = FIELD_GET(K1_TSENSOR_DATA_HIGH_MASK, val);
    else
// temp = FIELD_GET(K1_TSENSOR_DATA_LOW_MASK, val);
// temp -= TEMPERATURE_OFFSET;
// temp *= 1000;
    return 0;
    }
//
// For each sensor, the hardware threshold register is 32 bits:
// - Lower 16 bits [15:0] configure the low threshold temperature.
// - Upper 16 bits [31:16] configure the high threshold temperature.
//
#[no_mangle]
unsafe extern "C" fn k1_tsensor_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int) -> c_int {
    static int k1_tsensor_set_trips(struct thermal_zone_device *tz, int low, int high)
    {
    struct k1_tsensor_channel *ch = thermal_zone_device_priv(tz);
    struct k1_tsensor *ts = ch.ts;
    u32 val;
    low = clamp_val(low / 1000 + TEMPERATURE_OFFSET, TEMPERATURE_OFFSET,
    FIELD_MAX(K1_TSENSOR_THRSH_LOW_MASK));
    high = clamp_val(high / 1000 + TEMPERATURE_OFFSET, TEMPERATURE_OFFSET,
    FIELD_MAX(K1_TSENSOR_THRSH_HIGH_MASK));
    if (low >= high)
    return -EINVAL;
    val = readl(ts.base + K1_TSENSOR_THRSH_REG(ch.id));
    val &= ~(K1_TSENSOR_THRSH_LOW_MASK | K1_TSENSOR_THRSH_HIGH_MASK);
    val |= FIELD_PREP(K1_TSENSOR_THRSH_LOW_MASK, low);
    val |= FIELD_PREP(K1_TSENSOR_THRSH_HIGH_MASK, high);
    writel(val, ts.base + K1_TSENSOR_THRSH_REG(ch.id));
    return 0;
    }
    static const struct thermal_zone_device_ops k1_tsensor_ops = {
    .get_temp = k1_tsensor_get_temp,
    .set_trips = k1_tsensor_set_trips,
    };
#[no_mangle]
unsafe extern "C" fn k1_tsensor_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t k1_tsensor_irq_thread(int irq, void *data)
    {
    struct k1_tsensor *ts = (struct k1_tsensor *)data;
    int mask, status, i;
    status = readl(ts.base + K1_TSENSOR_INT_STA_REG);
    for (i = 0; i < MAX_SENSOR_NUMBER; i++) {
    if (status & K1_TSENSOR_INT_MASK(i)) {
    mask = readl(ts.base + K1_TSENSOR_INT_CLR_REG);
    mask |= K1_TSENSOR_INT_MASK(i);
    writel(mask, ts.base + K1_TSENSOR_INT_CLR_REG);
    thermal_zone_device_update(ts.ch[i].tzd, THERMAL_EVENT_UNSPECIFIED);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn k1_tsensor_shutdown(ts: *mut k1_tsensor) {
    static void k1_tsensor_shutdown(struct k1_tsensor *ts)
    {
    u32 val;
// Disable all interrupts
    writel(0xffffffff, ts.base + K1_TSENSOR_INT_EN_REG);
// Disable all sensors
    val = readl(ts.base + K1_TSENSOR_EN_REG);
    val &= ~K1_TSENSOR_EN_ALL;
    writel(val, ts.base + K1_TSENSOR_EN_REG);
// Clear the sampling configuration set by k1_tsensor_init()
    val = readl(ts.base + K1_TSENSOR_TIME_REG);
    val &= ~(K1_TSENSOR_TIME_FILTER_PERIOD |
    K1_TSENSOR_TIME_ADC_CNT_RST |
    K1_TSENSOR_TIME_WAIT_REF_CNT);
    writel(val, ts.base + K1_TSENSOR_TIME_REG);
// Clear the control bits configured by k1_tsensor_init()
    val = readl(ts.base + K1_TSENSOR_PCTRL_REG);
    val &= ~(K1_TSENSOR_PCTRL_RAW_SEL |
    K1_TSENSOR_PCTRL_TEMP_MODE |
    K1_TSENSOR_PCTRL_HW_AUTO_MODE |
    K1_TSENSOR_PCTRL_ENABLE);
    writel(val, ts.base + K1_TSENSOR_PCTRL_REG);
    }
#[no_mangle]
unsafe extern "C" fn k1_tsensor_shutdown_action(data: *mut c_void) {
    static void k1_tsensor_shutdown_action(void *data)
    {
    k1_tsensor_shutdown(data);
    }
#[no_mangle]
unsafe extern "C" fn k1_tsensor_probe(pdev: *mut platform_device) -> c_int {
    static int k1_tsensor_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct k1_tsensor *ts;
    struct reset_control *reset;
    struct clk *clk;
    int i, irq, ret;
    ts = devm_kzalloc(dev, sizeof(*ts), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ts.base))
    return dev_err_probe(dev, PTR_ERR(ts.base), "Failed to get reg\n");
    reset = devm_reset_control_get_exclusive_deasserted(dev, core::ptr::null_mut());
    if (IS_ERR(reset))
    return dev_err_probe(dev, PTR_ERR(reset), "Failed to get/deassert reset control\n");
    clk = devm_clk_get_enabled(dev, "core");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Failed to get core clock\n");
    clk = devm_clk_get_enabled(dev, "bus");
    if (IS_ERR(clk))
    return dev_err_probe(dev, PTR_ERR(clk), "Failed to get bus clock\n");
    k1_tsensor_init(ts);
    for (i = 0; i < MAX_SENSOR_NUMBER; ++i) {
    ts.ch[i].id = i;
    ts.ch[i].ts = ts;
    ts.ch[i].tzd = devm_thermal_of_zone_register(dev, i, ts.ch + i, &k1_tsensor_ops);
    if (IS_ERR(ts.ch[i].tzd)) {
    ret = PTR_ERR(ts.ch[i].tzd);
    goto err_shutdown;
    }
// Attach sysfs hwmon attributes for userspace monitoring
    ret = devm_thermal_add_hwmon_sysfs(dev, ts.ch[i].tzd);
    if (ret)
    dev_warn(dev, "Failed to add hwmon sysfs attributes\n");
    }
    irq = platform_get_irq(pdev, 0);
    if (irq < 0) {
    ret = irq;
    goto err_shutdown;
    }
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    k1_tsensor_irq_thread,
    IRQF_ONESHOT, "k1_tsensor", ts);
    if (ret < 0)
    goto err_shutdown;
    ret = devm_add_action_or_reset(dev, k1_tsensor_shutdown_action, ts);
    if (ret)
    return ret;
// Enable interrupts only after all zones and the handler are ready
    for (i = 0; i < MAX_SENSOR_NUMBER; ++i)
    k1_tsensor_enable_irq(ts.ch + i);
    platform_set_drvdata(pdev, ts);
    return 0;
    err_shutdown:
    k1_tsensor_shutdown(ts);
    return ret;
    }
    static const struct of_device_id k1_tsensor_dt_ids[] = {
    { .compatible = "spacemit,k1-tsensor" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, k1_tsensor_dt_ids);
    static struct platform_driver k1_tsensor_driver = {
    .driver = {
    .name		= "k1_tsensor",
    .of_match_table = k1_tsensor_dt_ids,
    },
    .probe	= k1_tsensor_probe,
    };
    module_platform_driver(k1_tsensor_driver);
    MODULE_DESCRIPTION("SpacemiT K1 Thermal Sensor Driver");
    MODULE_AUTHOR("Shuwei Wu <shuwei.wu@mailbox.org>");
    MODULE_LICENSE("GPL");
