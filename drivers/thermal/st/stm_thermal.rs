//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/st/stm_thermal.c
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
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Author: David Hernandez Sanchez <david.hernandezsanchez@st.com> for
// STMicroelectronics.
//

// DTS register offsets
pub const DTS_CFGR1_OFFSET: c_uint = 0x0;
pub const DTS_T0VALR1_OFFSET: c_uint = 0x8;

pub const DTS_ITR1_OFFSET: c_uint = 0x14;
pub const DTS_DR_OFFSET: c_uint = 0x1C;
pub const DTS_SR_OFFSET: c_uint = 0x20;
pub const DTS_ITENR_OFFSET: c_uint = 0x24;
pub const DTS_ICIFR_OFFSET: c_uint = 0x28;
// DTS_CFGR1 register mask definitions

// DTS_T0VALR1 register mask definitions

// DTS_RAMPVALR register mask definitions

// DTS_ITR1 register mask definitions

// DTS_DR register mask definitions

// DTS_ITENR register mask definitions

// DTS_ICIFR register mask definitions

// Less significant bit position definitions
pub const TS1_T0_POS: c_int = 16;
pub const TS1_HITTHD_POS: c_int = 16;
pub const TS1_LITTHD_POS: c_int = 0;
pub const HSREF_CLK_DIV_POS: c_int = 24;
// DTS_CFGR1 bit definitions

// DTS_SR bit definitions

// Bit definitions below are common for DTS_SR, DTS_ITENR and DTS_CIFR

// Constants
pub const ADJUST: c_int = 100;
pub const POLL_TIMEOUT: c_int = 5000;
pub const STARTUP_TIME: c_int = 40;

pub const NO_HW_TRIG: c_int = 0;
pub const SAMPLING_TIME: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_thermal_sensor {
    pub dev: *mut device,
    pub th_dev: *mut thermal_zone_device,
    pub mode: enum thermal_device_mode,
    pub clk: *mut clk,
    pub low_temp_enabled: c_uint,
    pub high_temp_enabled: c_uint,
    pub irq: c_int,
    pub base: *mut void __iomem,
    pub ramp_coeff: int t0, fmt0,,
}

#[no_mangle]
unsafe extern "C" fn stm_enable_irq(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_enable_irq(struct stm_thermal_sensor *sensor)
    {
    u32 value;
    dev_dbg(sensor.dev, "low:%d high:%d\n", sensor.low_temp_enabled,
    sensor.high_temp_enabled);
// Disable IT generation for low and high thresholds
    value = readl_relaxed(sensor.base + DTS_ITENR_OFFSET);
    value &= ~(LOW_THRESHOLD | HIGH_THRESHOLD);
    if (sensor.low_temp_enabled)
    value |= HIGH_THRESHOLD;
    if (sensor.high_temp_enabled)
    value |= LOW_THRESHOLD;
// Enable interrupts
    writel_relaxed(value, sensor.base + DTS_ITENR_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_irq_handler(irq: c_int, sdata: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm_thermal_irq_handler(int irq, void *sdata)
    {
    struct stm_thermal_sensor *sensor = sdata;
    dev_dbg(sensor.dev, "sr:%d\n",
    readl_relaxed(sensor.base + DTS_SR_OFFSET));
    thermal_zone_device_update(sensor.th_dev, THERMAL_EVENT_UNSPECIFIED);
    stm_enable_irq(sensor);
// Acknoledge all DTS irqs
    writel_relaxed(ICIFR_MASK, sensor.base + DTS_ICIFR_OFFSET);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn stm_sensor_power_on(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_sensor_power_on(struct stm_thermal_sensor *sensor)
    {
    int ret;
    u32 value;
// Enable sensor
    value = readl_relaxed(sensor.base + DTS_CFGR1_OFFSET);
    value |= TS1_EN;
    writel_relaxed(value, sensor.base + DTS_CFGR1_OFFSET);
//
// The DTS block can be enabled by setting TSx_EN bit in
// DTS_CFGRx register. It requires a startup time of
// 40μs. Use 5 ms as arbitrary timeout.
//
    ret = readl_poll_timeout(sensor.base + DTS_SR_OFFSET,
    value, (value & TS_RDY),
    STARTUP_TIME, POLL_TIMEOUT);
    if (ret)
    return ret;
// Start continuous measuring
    value = readl_relaxed(sensor.base +
    DTS_CFGR1_OFFSET);
    value |= TS1_START;
    writel_relaxed(value, sensor.base +
    DTS_CFGR1_OFFSET);
    sensor.mode = THERMAL_DEVICE_ENABLED;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_sensor_power_off(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_sensor_power_off(struct stm_thermal_sensor *sensor)
    {
    u32 value;
    sensor.mode = THERMAL_DEVICE_DISABLED;
// Stop measuring
    value = readl_relaxed(sensor.base + DTS_CFGR1_OFFSET);
    value &= ~TS1_START;
    writel_relaxed(value, sensor.base + DTS_CFGR1_OFFSET);
// Ensure stop is taken into account
    usleep_range(STARTUP_TIME, POLL_TIMEOUT);
// Disable sensor
    value = readl_relaxed(sensor.base + DTS_CFGR1_OFFSET);
    value &= ~TS1_EN;
    writel_relaxed(value, sensor.base + DTS_CFGR1_OFFSET);
// Ensure disable is taken into account
    return readl_poll_timeout(sensor.base + DTS_SR_OFFSET, value,
    !(value & TS_RDY),
    STARTUP_TIME, POLL_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_calibration(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_thermal_calibration(struct stm_thermal_sensor *sensor)
    {
    u32 value, clk_freq;
    u32 prescaler;
// Figure out prescaler value for PCLK during calibration
    clk_freq = clk_get_rate(sensor.clk);
    if (!clk_freq)
    return -EINVAL;
    prescaler = 0;
    clk_freq /= HZ_PER_MHZ;
    if (clk_freq) {
    while (prescaler <= clk_freq)
    prescaler++;
    }
    value = readl_relaxed(sensor.base + DTS_CFGR1_OFFSET);
// Clear prescaler
    value &= ~HSREF_CLK_DIV_MASK;
// Set prescaler. pclk_freq/prescaler < 1MHz
    value |= (prescaler << HSREF_CLK_DIV_POS);
// Select PCLK as reference clock
    value &= ~REFCLK_SEL;
// Set maximal sampling time for better precision
    value |= TS1_SMP_TIME_MASK;
// Measure with calibration
    value &= ~CALIBRATION_CONTROL;
// select trigger
    value &= ~TS1_INTRIG_SEL_MASK;
    value |= NO_HW_TRIG;
    writel_relaxed(value, sensor.base + DTS_CFGR1_OFFSET);
    return 0;
    }
// Fill in DTS structure with factory sensor values
#[no_mangle]
unsafe extern "C" fn stm_thermal_read_factory_settings(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_thermal_read_factory_settings(struct stm_thermal_sensor *sensor)
    {
// Retrieve engineering calibration temperature
    sensor.t0 = readl_relaxed(sensor.base + DTS_T0VALR1_OFFSET) &
    TS1_T0_MASK;
    if (!sensor.t0)
    sensor.t0 = TS1_T0_VAL0;
    else
    sensor.t0 = TS1_T0_VAL1;
// Retrieve fmt0 and put it on Hz
    sensor.fmt0 = ADJUST * (readl_relaxed(sensor.base +
    DTS_T0VALR1_OFFSET) & TS1_FMT0_MASK);
// Retrieve ramp coefficient
    sensor.ramp_coeff = readl_relaxed(sensor.base + DTS_RAMPVALR_OFFSET) &
    TS1_RAMP_COEFF_MASK;
    if (!sensor.fmt0 || !sensor.ramp_coeff) {
    dev_err(sensor.dev, "%s: wrong setting\n", __func__);
    return -EINVAL;
    }
    dev_dbg(sensor.dev, "%s: T0 = %doC, FMT0 = %dHz, RAMP_COEFF = %dHz/oC",
    __func__, sensor.t0, sensor.fmt0, sensor.ramp_coeff);
    return 0;
    }
    static int stm_thermal_calculate_threshold(struct stm_thermal_sensor *sensor,
    int temp, u32 *th)
    {
    int freqM;
// Figure out the CLK_PTAT frequency for a given temperature
    freqM = ((temp - sensor.t0) * sensor.ramp_coeff) / 1000 +
    sensor.fmt0;
// Figure out the threshold sample number
// th = clk_get_rate(sensor->clk) * SAMPLING_TIME / freqM;
    if (!*th)
    return -EINVAL;
    dev_dbg(sensor.dev, "freqM=%d Hz, threshold=0x%x", freqM, *th);
    return 0;
    }
// Disable temperature interrupt
#[no_mangle]
unsafe extern "C" fn stm_disable_irq(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_disable_irq(struct stm_thermal_sensor *sensor)
    {
    u32 value;
// Disable IT generation
    value = readl_relaxed(sensor.base + DTS_ITENR_OFFSET);
    value &= ~ITENR_MASK;
    writel_relaxed(value, sensor.base + DTS_ITENR_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_set_trips(tz: *mut thermal_zone_device, low: c_int, high: c_int) -> c_int {
    static int stm_thermal_set_trips(struct thermal_zone_device *tz, int low, int high)
    {
    struct stm_thermal_sensor *sensor = thermal_zone_device_priv(tz);
    u32 itr1, th;
    int ret;
    dev_dbg(sensor.dev, "set trips %d <-. %d\n", low, high);
// Erase threshold content
    itr1 = readl_relaxed(sensor.base + DTS_ITR1_OFFSET);
    itr1 &= ~(TS1_LITTHD_MASK | TS1_HITTHD_MASK);
//
// Disable low-temp if "low" is too small. As per thermal framework
// API, we use -INT_MAX rather than INT_MIN.
//
    if (low > -INT_MAX) {
    sensor.low_temp_enabled = 1;
// add 0.5 of hysteresis due to measurement error
    ret = stm_thermal_calculate_threshold(sensor, low - 500, &th);
    if (ret)
    return ret;
    itr1 |= (TS1_HITTHD_MASK  & (th << TS1_HITTHD_POS));
    } else {
    sensor.low_temp_enabled = 0;
    }
// Disable high-temp if "high" is too big.
    if (high < INT_MAX) {
    sensor.high_temp_enabled = 1;
    ret = stm_thermal_calculate_threshold(sensor, high, &th);
    if (ret)
    return ret;
    itr1 |= (TS1_LITTHD_MASK  & (th << TS1_LITTHD_POS));
    } else {
    sensor.high_temp_enabled = 0;
    }
// Write new threshod values
    writel_relaxed(itr1, sensor.base + DTS_ITR1_OFFSET);
    return 0;
    }
// Callback to get temperature from HW
#[no_mangle]
unsafe extern "C" fn stm_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int stm_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct stm_thermal_sensor *sensor = thermal_zone_device_priv(tz);
    u32 periods;
    int freqM, ret;
    if (sensor.mode != THERMAL_DEVICE_ENABLED)
    return -EAGAIN;
// Retrieve the number of periods sampled
    ret = readl_relaxed_poll_timeout(sensor.base + DTS_DR_OFFSET, periods,
    (periods & TS1_MFREQ_MASK),
    STARTUP_TIME, POLL_TIMEOUT);
    if (ret)
    return ret;
// Figure out the CLK_PTAT frequency
    freqM = (clk_get_rate(sensor.clk) * SAMPLING_TIME) / periods;
    if (!freqM)
    return -EINVAL;
// Figure out the temperature in mili celsius
// temp = (freqM - sensor->fmt0) * 1000 / sensor->ramp_coeff + sensor->t0;
    return 0;
    }
// Registers DTS irq to be visible by GIC
#[no_mangle]
unsafe extern "C" fn stm_register_irq(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_register_irq(struct stm_thermal_sensor *sensor)
    {
    struct device *dev = sensor.dev;
    struct platform_device *pdev = to_platform_device(dev);
    int ret;
    sensor.irq = platform_get_irq(pdev, 0);
    if (sensor.irq < 0)
    return sensor.irq;
    ret = devm_request_threaded_irq(dev, sensor.irq,
    core::ptr::null_mut(),
    stm_thermal_irq_handler,
    IRQF_ONESHOT,
    dev.driver.name, sensor);
    if (ret)
    return ret;
    dev_dbg(dev, "%s: thermal IRQ registered", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_sensor_off(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_thermal_sensor_off(struct stm_thermal_sensor *sensor)
    {
    int ret;
    stm_disable_irq(sensor);
    ret = stm_sensor_power_off(sensor);
    if (ret)
    return ret;
    clk_disable_unprepare(sensor.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_prepare(sensor: *mut stm_thermal_sensor) -> c_int {
    static int stm_thermal_prepare(struct stm_thermal_sensor *sensor)
    {
    int ret;
    ret = clk_prepare_enable(sensor.clk);
    if (ret)
    return ret;
    ret = stm_thermal_read_factory_settings(sensor);
    if (ret)
    goto thermal_unprepare;
    ret = stm_thermal_calibration(sensor);
    if (ret)
    goto thermal_unprepare;
    return 0;
    thermal_unprepare:
    clk_disable_unprepare(sensor.clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_suspend(dev: *mut device) -> c_int {
    static int stm_thermal_suspend(struct device *dev)
    {
    struct stm_thermal_sensor *sensor = dev_get_drvdata(dev);
    return stm_thermal_sensor_off(sensor);
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_resume(dev: *mut device) -> c_int {
    static int stm_thermal_resume(struct device *dev)
    {
    int ret;
    struct stm_thermal_sensor *sensor = dev_get_drvdata(dev);
    ret = stm_thermal_prepare(sensor);
    if (ret)
    return ret;
    ret = stm_sensor_power_on(sensor);
    if (ret)
    return ret;
    thermal_zone_device_update(sensor.th_dev, THERMAL_EVENT_UNSPECIFIED);
    stm_enable_irq(sensor);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(stm_thermal_pm_ops,
    stm_thermal_suspend, stm_thermal_resume);
    static const struct thermal_zone_device_ops stm_tz_ops = {
    .get_temp	= stm_thermal_get_temp,
    .set_trips	= stm_thermal_set_trips,
    };
    static const struct of_device_id stm_thermal_of_match[] = {
    { .compatible = "st,stm32-thermal"},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, stm_thermal_of_match);
#[no_mangle]
unsafe extern "C" fn stm_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int stm_thermal_probe(struct platform_device *pdev)
    {
    struct stm_thermal_sensor *sensor;
    void __iomem *base;
    int ret;
    if (!pdev.dev.of_node) {
    dev_err(&pdev.dev, "%s: device tree node not found\n",
    __func__);
    return -EINVAL;
    }
    sensor = devm_kzalloc(&pdev.dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    platform_set_drvdata(pdev, sensor);
    sensor.dev = &pdev.dev;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(base))
    return PTR_ERR(base);
// Populate sensor
    sensor.base = base;
    sensor.clk = devm_clk_get(&pdev.dev, "pclk");
    if (IS_ERR(sensor.clk)) {
    dev_err(&pdev.dev, "%s: failed to fetch PCLK clock\n",
    __func__);
    return PTR_ERR(sensor.clk);
    }
    stm_disable_irq(sensor);
// Clear irq flags
    writel_relaxed(ICIFR_MASK, sensor.base + DTS_ICIFR_OFFSET);
// Configure and enable HW sensor
    ret = stm_thermal_prepare(sensor);
    if (ret) {
    dev_err(&pdev.dev, "Error prepare sensor: %d\n", ret);
    return ret;
    }
    ret = stm_sensor_power_on(sensor);
    if (ret) {
    dev_err(&pdev.dev, "Error power on sensor: %d\n", ret);
    return ret;
    }
    sensor.th_dev = devm_thermal_of_zone_register(&pdev.dev, 0,
    sensor,
    &stm_tz_ops);
    if (IS_ERR(sensor.th_dev)) {
    dev_err(&pdev.dev, "%s: thermal zone sensor registering KO\n",
    __func__);
    ret = PTR_ERR(sensor.th_dev);
    return ret;
    }
// Register IRQ into GIC
    ret = stm_register_irq(sensor);
    if (ret)
    goto err_tz;
    stm_enable_irq(sensor);
//
// Thermal_zone doesn't enable hwmon as default,
// enable it here
//
    ret = thermal_add_hwmon_sysfs(sensor.th_dev);
    if (ret)
    goto err_tz;
    dev_info(&pdev.dev, "%s: Driver initialized successfully\n",
    __func__);
    return 0;
    err_tz:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_thermal_remove(pdev: *mut platform_device) {
    static void stm_thermal_remove(struct platform_device *pdev)
    {
    struct stm_thermal_sensor *sensor = platform_get_drvdata(pdev);
    stm_thermal_sensor_off(sensor);
    thermal_remove_hwmon_sysfs(sensor.th_dev);
    }
    static struct platform_driver stm_thermal_driver = {
    .driver = {
    .name	= "stm_thermal",
    .pm     = pm_sleep_ptr(&stm_thermal_pm_ops),
    .of_match_table = stm_thermal_of_match,
    },
    .probe		= stm_thermal_probe,
    .remove		= stm_thermal_remove,
    };
    module_platform_driver(stm_thermal_driver);
    MODULE_DESCRIPTION("STMicroelectronics STM32 Thermal Sensor Driver");
    MODULE_AUTHOR("David Hernandez Sanchez <david.hernandezsanchez@st.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:stm_thermal");
