//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/imx_thermal.c
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
// Copyright 2013 Freescale Semiconductor, Inc.

pub const REG_SET: c_uint = 0x4;
pub const REG_CLR: c_uint = 0x8;
pub const REG_TOG: c_uint = 0xc;
// i.MX6 specific
pub const IMX6_MISC0: c_uint = 0x0150;

pub const IMX6_MISC1: c_uint = 0x0160;

// Below LOW and PANIC bits are only for TEMPMON_IMX6SX

pub const IMX6_TEMPSENSE0: c_uint = 0x0180;
pub const IMX6_TEMPSENSE0_ALARM_VALUE_SHIFT: c_int = 20;

pub const IMX6_TEMPSENSE0_TEMP_CNT_SHIFT: c_int = 8;

pub const IMX6_TEMPSENSE1: c_uint = 0x0190;
pub const IMX6_TEMPSENSE1_MEASURE_FREQ: c_uint = 0xffff;
pub const IMX6_TEMPSENSE1_MEASURE_FREQ_SHIFT: c_int = 0;
pub const OCOTP_MEM0: c_uint = 0x0480;
pub const OCOTP_ANA1: c_uint = 0x04e0;
// Below TEMPSENSE2 is only for TEMPMON_IMX6SX
pub const IMX6_TEMPSENSE2: c_uint = 0x0290;
pub const IMX6_TEMPSENSE2_LOW_VALUE_SHIFT: c_int = 0;
pub const IMX6_TEMPSENSE2_LOW_VALUE_MASK: c_uint = 0xfff;
pub const IMX6_TEMPSENSE2_PANIC_VALUE_SHIFT: c_int = 16;
pub const IMX6_TEMPSENSE2_PANIC_VALUE_MASK: c_uint = 0xfff0000;
// i.MX7 specific
pub const IMX7_ANADIG_DIGPROG: c_uint = 0x800;
pub const IMX7_TEMPSENSE0: c_uint = 0x300;
pub const IMX7_TEMPSENSE0_PANIC_ALARM_SHIFT: c_int = 18;

pub const IMX7_TEMPSENSE0_HIGH_ALARM_SHIFT: c_int = 9;

pub const IMX7_TEMPSENSE0_LOW_ALARM_SHIFT: c_int = 0;
pub const IMX7_TEMPSENSE0_LOW_ALARM_MASK: c_uint = 0x1ff;
pub const IMX7_TEMPSENSE1: c_uint = 0x310;
pub const IMX7_TEMPSENSE1_MEASURE_FREQ_SHIFT: c_int = 16;

pub const IMX7_TEMPSENSE1_TEMP_VALUE_SHIFT: c_int = 0;
pub const IMX7_TEMPSENSE1_TEMP_VALUE_MASK: c_uint = 0x1ff;
// The driver supports 1 passive trip point and 1 critical trip point
    enum imx_thermal_trip {
    IMX_TRIP_PASSIVE,
    IMX_TRIP_CRITICAL,
    };

pub const IMX_PASSIVE_DELAY: c_int = 1000;
pub const TEMPMON_IMX6Q: c_int = 1;
pub const TEMPMON_IMX6SX: c_int = 2;
pub const TEMPMON_IMX7D: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_soc_data {
    pub version: u32,
    pub sensor_ctrl: u32,
    pub power_down_mask: u32,
    pub measure_temp_mask: u32,
    pub measure_freq_ctrl: u32,
    pub measure_freq_mask: u32,
    pub measure_freq_shift: u32,
    pub temp_data: u32,
    pub temp_value_mask: u32,
    pub temp_value_shift: u32,
    pub temp_valid_mask: u32,
    pub panic_alarm_ctrl: u32,
    pub panic_alarm_mask: u32,
    pub panic_alarm_shift: u32,
    pub high_alarm_ctrl: u32,
    pub high_alarm_mask: u32,
    pub high_alarm_shift: u32,
    pub low_alarm_ctrl: u32,
    pub low_alarm_mask: u32,
    pub low_alarm_shift: u32,
}

    static struct thermal_trip trips[] = {
    [IMX_TRIP_PASSIVE]  = { .type = THERMAL_TRIP_PASSIVE,
    .flags = THERMAL_TRIP_FLAG_RW_TEMP },
    [IMX_TRIP_CRITICAL] = { .type = THERMAL_TRIP_CRITICAL },
    };
    static struct thermal_soc_data thermal_imx6q_data = {
    .version = TEMPMON_IMX6Q,
    .sensor_ctrl = IMX6_TEMPSENSE0,
    .power_down_mask = IMX6_TEMPSENSE0_POWER_DOWN,
    .measure_temp_mask = IMX6_TEMPSENSE0_MEASURE_TEMP,
    .measure_freq_ctrl = IMX6_TEMPSENSE1,
    .measure_freq_shift = IMX6_TEMPSENSE1_MEASURE_FREQ_SHIFT,
    .measure_freq_mask = IMX6_TEMPSENSE1_MEASURE_FREQ,
    .temp_data = IMX6_TEMPSENSE0,
    .temp_value_mask = IMX6_TEMPSENSE0_TEMP_CNT_MASK,
    .temp_value_shift = IMX6_TEMPSENSE0_TEMP_CNT_SHIFT,
    .temp_valid_mask = IMX6_TEMPSENSE0_FINISHED,
    .high_alarm_ctrl = IMX6_TEMPSENSE0,
    .high_alarm_mask = IMX6_TEMPSENSE0_ALARM_VALUE_MASK,
    .high_alarm_shift = IMX6_TEMPSENSE0_ALARM_VALUE_SHIFT,
    };
    static struct thermal_soc_data thermal_imx6sx_data = {
    .version = TEMPMON_IMX6SX,
    .sensor_ctrl = IMX6_TEMPSENSE0,
    .power_down_mask = IMX6_TEMPSENSE0_POWER_DOWN,
    .measure_temp_mask = IMX6_TEMPSENSE0_MEASURE_TEMP,
    .measure_freq_ctrl = IMX6_TEMPSENSE1,
    .measure_freq_shift = IMX6_TEMPSENSE1_MEASURE_FREQ_SHIFT,
    .measure_freq_mask = IMX6_TEMPSENSE1_MEASURE_FREQ,
    .temp_data = IMX6_TEMPSENSE0,
    .temp_value_mask = IMX6_TEMPSENSE0_TEMP_CNT_MASK,
    .temp_value_shift = IMX6_TEMPSENSE0_TEMP_CNT_SHIFT,
    .temp_valid_mask = IMX6_TEMPSENSE0_FINISHED,
    .high_alarm_ctrl = IMX6_TEMPSENSE0,
    .high_alarm_mask = IMX6_TEMPSENSE0_ALARM_VALUE_MASK,
    .high_alarm_shift = IMX6_TEMPSENSE0_ALARM_VALUE_SHIFT,
    .panic_alarm_ctrl = IMX6_TEMPSENSE2,
    .panic_alarm_mask = IMX6_TEMPSENSE2_PANIC_VALUE_MASK,
    .panic_alarm_shift = IMX6_TEMPSENSE2_PANIC_VALUE_SHIFT,
    .low_alarm_ctrl = IMX6_TEMPSENSE2,
    .low_alarm_mask = IMX6_TEMPSENSE2_LOW_VALUE_MASK,
    .low_alarm_shift = IMX6_TEMPSENSE2_LOW_VALUE_SHIFT,
    };
    static struct thermal_soc_data thermal_imx7d_data = {
    .version = TEMPMON_IMX7D,
    .sensor_ctrl = IMX7_TEMPSENSE1,
    .power_down_mask = IMX7_TEMPSENSE1_POWER_DOWN,
    .measure_temp_mask = IMX7_TEMPSENSE1_MEASURE_TEMP,
    .measure_freq_ctrl = IMX7_TEMPSENSE1,
    .measure_freq_shift = IMX7_TEMPSENSE1_MEASURE_FREQ_SHIFT,
    .measure_freq_mask = IMX7_TEMPSENSE1_MEASURE_FREQ_MASK,
    .temp_data = IMX7_TEMPSENSE1,
    .temp_value_mask = IMX7_TEMPSENSE1_TEMP_VALUE_MASK,
    .temp_value_shift = IMX7_TEMPSENSE1_TEMP_VALUE_SHIFT,
    .temp_valid_mask = IMX7_TEMPSENSE1_FINISHED,
    .panic_alarm_ctrl = IMX7_TEMPSENSE1,
    .panic_alarm_mask = IMX7_TEMPSENSE0_PANIC_ALARM_MASK,
    .panic_alarm_shift = IMX7_TEMPSENSE0_PANIC_ALARM_SHIFT,
    .high_alarm_ctrl = IMX7_TEMPSENSE0,
    .high_alarm_mask = IMX7_TEMPSENSE0_HIGH_ALARM_MASK,
    .high_alarm_shift = IMX7_TEMPSENSE0_HIGH_ALARM_SHIFT,
    .low_alarm_ctrl = IMX7_TEMPSENSE0,
    .low_alarm_mask = IMX7_TEMPSENSE0_LOW_ALARM_MASK,
    .low_alarm_shift = IMX7_TEMPSENSE0_LOW_ALARM_SHIFT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_thermal_data {
    pub dev: *mut device,
    pub policy: *mut cpufreq_policy,
    pub tz: *mut thermal_zone_device,
    pub cdev: *mut thermal_cooling_device,
    pub tempmon: *mut regmap,
    pub /: *mut *mut u32 c1, c2; / See formula in imx_init_calib(),
    pub temp_max: c_int,
    pub alarm_temp: c_int,
    pub last_temp: c_int,
    pub irq_enabled: bool,
    pub irq: c_int,
    pub thermal_clk: *mut clk,
    pub socdata: *const thermal_soc_data,
    pub temp_grade: *const c_char,
}

    static void imx_set_panic_temp(struct imx_thermal_data *data,
    int panic_temp)
    {
    const struct thermal_soc_data *soc_data = data.socdata;
    struct regmap *map = data.tempmon;
    int critical_value;
    critical_value = (data.c2 - panic_temp) / data.c1;
    regmap_write(map, soc_data.panic_alarm_ctrl + REG_CLR,
    soc_data.panic_alarm_mask);
    regmap_write(map, soc_data.panic_alarm_ctrl + REG_SET,
    critical_value << soc_data.panic_alarm_shift);
    }
    static void imx_set_alarm_temp(struct imx_thermal_data *data,
    int alarm_temp)
    {
    struct regmap *map = data.tempmon;
    const struct thermal_soc_data *soc_data = data.socdata;
    int alarm_value;
    data.alarm_temp = alarm_temp;
    if (data.socdata.version == TEMPMON_IMX7D)
    alarm_value = alarm_temp / 1000 + data.c1 - 25;
    else
    alarm_value = (data.c2 - alarm_temp) / data.c1;
    regmap_write(map, soc_data.high_alarm_ctrl + REG_CLR,
    soc_data.high_alarm_mask);
    regmap_write(map, soc_data.high_alarm_ctrl + REG_SET,
    alarm_value << soc_data.high_alarm_shift);
    }
#[no_mangle]
unsafe extern "C" fn imx_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int imx_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct imx_thermal_data *data = thermal_zone_device_priv(tz);
    const struct thermal_soc_data *soc_data = data.socdata;
    struct regmap *map = data.tempmon;
    unsigned int n_meas;
    u32 val;
    int ret;
    ret = pm_runtime_resume_and_get(data.dev);
    if (ret < 0)
    return ret;
    regmap_read(map, soc_data.temp_data, &val);
    if ((val & soc_data.temp_valid_mask) == 0)
    return -EAGAIN;
    n_meas = (val & soc_data.temp_value_mask)
    >> soc_data.temp_value_shift;
// See imx_init_calib() for formula derivation
    if (data.socdata.version == TEMPMON_IMX7D)
// temp = (n_meas - data->c1 + 25) * 1000;
    else
// temp = data->c2 - n_meas * data->c1;
// Update alarm value to next higher trip point for TEMPMON_IMX6Q
    if (data.socdata.version == TEMPMON_IMX6Q) {
    if (data.alarm_temp == trips[IMX_TRIP_PASSIVE].temperature &&
// temp >= trips[IMX_TRIP_PASSIVE].temperature)
    imx_set_alarm_temp(data, trips[IMX_TRIP_CRITICAL].temperature);
    if (data.alarm_temp == trips[IMX_TRIP_CRITICAL].temperature &&
// temp < trips[IMX_TRIP_PASSIVE].temperature) {
    imx_set_alarm_temp(data, trips[IMX_TRIP_PASSIVE].temperature);
    dev_dbg(data.dev, "thermal alarm off: T < %d\n",
    data.alarm_temp / 1000);
    }
    }
    if (*temp != data.last_temp) {
    dev_dbg(data.dev, "millicelsius: %d\n", *temp);
    data.last_temp = *temp;
    }
// Reenable alarm IRQ if temperature below alarm temperature
    if (!data.irq_enabled && *temp < data.alarm_temp) {
    data.irq_enabled = true;
    enable_irq(data.irq);
    }
    pm_runtime_put(data.dev);
    return 0;
    }
    static int imx_change_mode(struct thermal_zone_device *tz,
    enum thermal_device_mode mode)
    {
    struct imx_thermal_data *data = thermal_zone_device_priv(tz);
    if (mode == THERMAL_DEVICE_ENABLED) {
    pm_runtime_get(data.dev);
    if (!data.irq_enabled) {
    data.irq_enabled = true;
    enable_irq(data.irq);
    }
    } else {
    pm_runtime_put(data.dev);
    if (data.irq_enabled) {
    disable_irq(data.irq);
    data.irq_enabled = false;
    }
    }
    return 0;
    }
    static int imx_set_trip_temp(struct thermal_zone_device *tz,
    const struct thermal_trip *trip, int temp)
    {
    struct imx_thermal_data *data = thermal_zone_device_priv(tz);
    int ret;
    ret = pm_runtime_resume_and_get(data.dev);
    if (ret < 0)
    return ret;
// do not allow passive to be set higher than critical
    if (temp < 0 || temp > trips[IMX_TRIP_CRITICAL].temperature)
    return -EINVAL;
    imx_set_alarm_temp(data, temp);
    trips[IMX_TRIP_PASSIVE].temperature = temp;
    pm_runtime_put(data.dev);
    return 0;
    }
    static bool imx_should_bind(struct thermal_zone_device *tz,
    const struct thermal_trip *trip,
    struct thermal_cooling_device *cdev,
    struct cooling_spec *c)
    {
    return trip.type == THERMAL_TRIP_PASSIVE;
    }
    static const struct thermal_zone_device_ops imx_tz_ops = {
    .should_bind = imx_should_bind,
    .get_temp = imx_get_temp,
    .change_mode = imx_change_mode,
    .set_trip_temp = imx_set_trip_temp,
    };
#[no_mangle]
unsafe extern "C" fn imx_init_calib(pdev: *mut platform_device, ocotp_ana1: u32) -> c_int {
    static int imx_init_calib(struct platform_device *pdev, u32 ocotp_ana1)
    {
    struct imx_thermal_data *data = platform_get_drvdata(pdev);
    int n1;
    u64 temp64;
    if (ocotp_ana1 == 0 || ocotp_ana1 == ~0) {
    dev_err(&pdev.dev, "invalid sensor calibration data\n");
    return -EINVAL;
    }
//
// On i.MX7D, we only use the calibration data at 25C to get the temp,
// Tmeas = ( Nmeas - n1) + 25; n1 is the fuse value for 25C.
//
    if (data.socdata.version == TEMPMON_IMX7D) {
    data.c1 = (ocotp_ana1 >> 9) & 0x1ff;
    return 0;
    }
//
// The sensor is calibrated at 25 °C (aka T1) and the value measured
// (aka N1) at this temperature is provided in bits [31:20] in the
// i.MX's OCOTP value ANA1.
// To find the actual temperature T, the following formula has to be used
// when reading value n from the sensor:
//
// T = T1 + (N - N1) / (0.4148468 - 0.0015423 * N1) °C + 3.580661 °C
// = [T1' - N1 / (0.4148468 - 0.0015423 * N1) °C] + N / (0.4148468 - 0.0015423 * N1) °C
// = [T1' + N1 / (0.0015423 * N1 - 0.4148468) °C] - N / (0.0015423 * N1 - 0.4148468) °C
// = c2 - c1 * N
//
// with
//
// T1' = 28.580661 °C
// c1 = 1 / (0.0015423 * N1 - 0.4297157) °C
// c2 = T1' + N1 / (0.0015423 * N1 - 0.4148468) °C
// = T1' + N1 * c1
//
    n1 = ocotp_ana1 >> 20;
    temp64 = 10000000; /* use 10^7 as fixed point constant for values in formula */
    temp64 *= 1000; /* to get result in °mC */
    do_div(temp64, 15423 * n1 - 4148468);
    data.c1 = temp64;
    data.c2 = n1 * data.c1 + 28581;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_init_temp_grade(pdev: *mut platform_device, ocotp_mem0: u32) {
    static void imx_init_temp_grade(struct platform_device *pdev, u32 ocotp_mem0)
    {
    struct imx_thermal_data *data = platform_get_drvdata(pdev);
// The maximum die temp is specified by the Temperature Grade
    switch ((ocotp_mem0 >> 6) & 0x3) {
    case 0: /* Commercial (0 to 95 °C) */
    data.temp_grade = "Commercial";
    data.temp_max = 95000;
    break;
    case 1: /* Extended Commercial (-20 °C to 105 °C) */
    data.temp_grade = "Extended Commercial";
    data.temp_max = 105000;
    break;
    case 2: /* Industrial (-40 °C to 105 °C) */
    data.temp_grade = "Industrial";
    data.temp_max = 105000;
    break;
    case 3: /* Automotive (-40 °C to 125 °C) */
    data.temp_grade = "Automotive";
    data.temp_max = 125000;
    break;
    }
//
// Set the critical trip point at 5 °C under max
// Set the passive trip point at 10 °C under max (changeable via sysfs)
//
    trips[IMX_TRIP_PASSIVE].temperature = data.temp_max - (1000 * 10);
    trips[IMX_TRIP_CRITICAL].temperature = data.temp_max - (1000 * 5);
    }
#[no_mangle]
unsafe extern "C" fn imx_init_from_tempmon_data(pdev: *mut platform_device) -> c_int {
    static int imx_init_from_tempmon_data(struct platform_device *pdev)
    {
    struct regmap *map;
    int ret;
    u32 val;
    map = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "fsl,tempmon-data");
    if (IS_ERR(map)) {
    ret = PTR_ERR(map);
    dev_err(&pdev.dev, "failed to get sensor regmap: %d\n", ret);
    return ret;
    }
    ret = regmap_read(map, OCOTP_ANA1, &val);
    if (ret) {
    dev_err(&pdev.dev, "failed to read sensor data: %d\n", ret);
    return ret;
    }
    ret = imx_init_calib(pdev, val);
    if (ret)
    return ret;
    ret = regmap_read(map, OCOTP_MEM0, &val);
    if (ret) {
    dev_err(&pdev.dev, "failed to read sensor data: %d\n", ret);
    return ret;
    }
    imx_init_temp_grade(pdev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_init_from_nvmem_cells(pdev: *mut platform_device) -> c_int {
    static int imx_init_from_nvmem_cells(struct platform_device *pdev)
    {
    int ret;
    u32 val;
    ret = nvmem_cell_read_u32(&pdev.dev, "calib", &val);
    if (ret)
    return ret;
    ret = imx_init_calib(pdev, val);
    if (ret)
    return ret;
    ret = nvmem_cell_read_u32(&pdev.dev, "temp_grade", &val);
    if (ret)
    return ret;
    imx_init_temp_grade(pdev, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_alarm_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_thermal_alarm_irq(int irq, void *dev)
    {
    struct imx_thermal_data *data = dev;
    disable_irq_nosync(irq);
    data.irq_enabled = false;
    return IRQ_WAKE_THREAD;
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_alarm_irq_thread(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t imx_thermal_alarm_irq_thread(int irq, void *dev)
    {
    struct imx_thermal_data *data = dev;
    dev_dbg(data.dev, "THERMAL ALARM: T > %d\n", data.alarm_temp / 1000);
    thermal_zone_device_update(data.tz, THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
    static const struct of_device_id of_imx_thermal_match[] = {
    { .compatible = "fsl,imx6q-tempmon", .data = &thermal_imx6q_data, },
    { .compatible = "fsl,imx6sx-tempmon", .data = &thermal_imx6sx_data, },
    { .compatible = "fsl,imx7d-tempmon", .data = &thermal_imx7d_data, },
    { /* end */ }
    };
    MODULE_DEVICE_TABLE(of, of_imx_thermal_match);

//
// Create cooling device in case no #cooling-cells property is available in
// CPU node
//
#[no_mangle]
unsafe extern "C" fn imx_thermal_register_legacy_cooling(data: *mut imx_thermal_data) -> c_int {
    static int imx_thermal_register_legacy_cooling(struct imx_thermal_data *data)
    {
    struct device_node *np;
    let mut ret: c_int = 0;
    data.policy = cpufreq_cpu_get(0);
    if (!data.policy) {
    pr_debug("%s: CPUFreq policy not found\n", __func__);
    return -EPROBE_DEFER;
    }
    np = of_get_cpu_node(data.policy.cpu, core::ptr::null_mut());
    if (!np || !of_property_present(np, "#cooling-cells")) {
    data.cdev = cpufreq_cooling_register(data.policy);
    if (IS_ERR(data.cdev)) {
    ret = PTR_ERR(data.cdev);
    cpufreq_cpu_put(data.policy);
    }
    }
    of_node_put(np);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_unregister_legacy_cooling(data: *mut imx_thermal_data) {
    static void imx_thermal_unregister_legacy_cooling(struct imx_thermal_data *data)
    {
    cpufreq_cooling_unregister(data.cdev);
    cpufreq_cpu_put(data.policy);
    }

#[no_mangle]
pub unsafe extern "C" fn imx_thermal_register_legacy_cooling(data: *mut imx_thermal_data) -> c_int {
    static inline int imx_thermal_register_legacy_cooling(struct imx_thermal_data *data)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn imx_thermal_unregister_legacy_cooling(data: *mut imx_thermal_data) {
    static inline void imx_thermal_unregister_legacy_cooling(struct imx_thermal_data *data)
    {
    }

#[no_mangle]
unsafe extern "C" fn imx_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int imx_thermal_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct imx_thermal_data *data;
    struct regmap *map;
    int measure_freq;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.dev = dev;
    map = syscon_regmap_lookup_by_phandle(dev.of_node, "fsl,tempmon");
    if (IS_ERR(map)) {
    ret = PTR_ERR(map);
    dev_err(dev, "failed to get tempmon regmap: %d\n", ret);
    return ret;
    }
    data.tempmon = map;
    data.socdata = of_device_get_match_data(dev);
    if (!data.socdata) {
    dev_err(dev, "no device match found\n");
    return -ENODEV;
    }
// make sure the IRQ flag is clear before enabling irq on i.MX6SX
    if (data.socdata.version == TEMPMON_IMX6SX) {
    regmap_write(map, IMX6_MISC1 + REG_CLR,
    IMX6_MISC1_IRQ_TEMPHIGH | IMX6_MISC1_IRQ_TEMPLOW
    | IMX6_MISC1_IRQ_TEMPPANIC);
//
// reset value of LOW ALARM is incorrect, set it to lowest
// value to avoid false trigger of low alarm.
//
    regmap_write(map, data.socdata.low_alarm_ctrl + REG_SET,
    data.socdata.low_alarm_mask);
    }
    data.irq = platform_get_irq(pdev, 0);
    if (data.irq < 0)
    return data.irq;
    platform_set_drvdata(pdev, data);
    if (of_property_present(dev.of_node, "nvmem-cells")) {
    ret = imx_init_from_nvmem_cells(pdev);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to init from nvmem\n");
    } else {
    ret = imx_init_from_tempmon_data(pdev);
    if (ret) {
    dev_err(dev, "failed to init from fsl,tempmon-data\n");
    return ret;
    }
    }
// Make sure sensor is in known good state for measurements
    regmap_write(map, data.socdata.sensor_ctrl + REG_CLR,
    data.socdata.power_down_mask);
    regmap_write(map, data.socdata.sensor_ctrl + REG_CLR,
    data.socdata.measure_temp_mask);
    regmap_write(map, data.socdata.measure_freq_ctrl + REG_CLR,
    data.socdata.measure_freq_mask);
    if (data.socdata.version != TEMPMON_IMX7D)
    regmap_write(map, IMX6_MISC0 + REG_SET,
    IMX6_MISC0_REFTOP_SELBIASOFF);
    regmap_write(map, data.socdata.sensor_ctrl + REG_SET,
    data.socdata.power_down_mask);
    ret = imx_thermal_register_legacy_cooling(data);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to register cpufreq cooling device\n");
    data.thermal_clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(data.thermal_clk)) {
    ret = dev_err_probe(dev, PTR_ERR(data.thermal_clk), "failed to get thermal clk\n");
    goto legacy_cleanup;
    }
//
// Thermal sensor needs clk on to get correct value, normally
// we should enable its clk before taking measurement and disable
// clk after measurement is done, but if alarm function is enabled,
// hardware will auto measure the temperature periodically, so we
// need to keep the clk always on for alarm function.
//
    ret = clk_prepare_enable(data.thermal_clk);
    if (ret) {
    dev_err(dev, "failed to enable thermal clk: %d\n", ret);
    goto legacy_cleanup;
    }
    data.tz = thermal_zone_device_register_with_trips("imx_thermal_zone",
    trips,
    ARRAY_SIZE(trips),
    data,
    &imx_tz_ops, core::ptr::null_mut(),
    IMX_PASSIVE_DELAY,
    IMX_POLLING_DELAY);
    if (IS_ERR(data.tz)) {
    ret = PTR_ERR(data.tz);
    dev_err(dev, "failed to register thermal zone device %d\n",
    ret);
    goto clk_disable;
    }
    dev_info(dev, "%s CPU temperature grade - max:%dC critical:%dC passive:%dC\n",
    data.temp_grade,
    data.temp_max / 1000, trips[IMX_TRIP_CRITICAL].temperature / 1000,
    trips[IMX_TRIP_PASSIVE].temperature / 1000);
// Enable measurements at ~ 10 Hz
    regmap_write(map, data.socdata.measure_freq_ctrl + REG_CLR,
    data.socdata.measure_freq_mask);
    measure_freq = DIV_ROUND_UP(32768, 10); /* 10 Hz */
    regmap_write(map, data.socdata.measure_freq_ctrl + REG_SET,
    measure_freq << data.socdata.measure_freq_shift);
    imx_set_alarm_temp(data, trips[IMX_TRIP_PASSIVE].temperature);
    if (data.socdata.version == TEMPMON_IMX6SX)
    imx_set_panic_temp(data, trips[IMX_TRIP_CRITICAL].temperature);
    regmap_write(map, data.socdata.sensor_ctrl + REG_CLR,
    data.socdata.power_down_mask);
    regmap_write(map, data.socdata.sensor_ctrl + REG_SET,
    data.socdata.measure_temp_mask);
// After power up, we need a delay before first access can be done.
    usleep_range(20, 50);
// the core was configured and enabled just before
    pm_runtime_set_active(dev);
    pm_runtime_enable(data.dev);
    ret = pm_runtime_resume_and_get(data.dev);
    if (ret < 0)
    goto disable_runtime_pm;
    data.irq_enabled = true;
    ret = thermal_zone_device_enable(data.tz);
    if (ret)
    goto thermal_zone_unregister;
    ret = devm_request_threaded_irq(dev, data.irq,
    imx_thermal_alarm_irq, imx_thermal_alarm_irq_thread,
    0, "imx_thermal", data);
    if (ret < 0)
    goto thermal_zone_unregister;
    pm_runtime_put(data.dev);
    return 0;
    thermal_zone_unregister:
    thermal_zone_device_unregister(data.tz);
    disable_runtime_pm:
    pm_runtime_put_noidle(data.dev);
    pm_runtime_disable(data.dev);
    clk_disable:
    clk_disable_unprepare(data.thermal_clk);
    legacy_cleanup:
    imx_thermal_unregister_legacy_cooling(data);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_remove(pdev: *mut platform_device) {
    static void imx_thermal_remove(struct platform_device *pdev)
    {
    struct imx_thermal_data *data = platform_get_drvdata(pdev);
    pm_runtime_put_noidle(data.dev);
    pm_runtime_disable(data.dev);
    thermal_zone_device_unregister(data.tz);
    imx_thermal_unregister_legacy_cooling(data);
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_suspend(dev: *mut device) -> c_int {
    static int imx_thermal_suspend(struct device *dev)
    {
    struct imx_thermal_data *data = dev_get_drvdata(dev);
    int ret;
//
// Need to disable thermal sensor, otherwise, when thermal core
// try to get temperature before thermal sensor resume, a wrong
// temperature will be read as the thermal sensor is powered
// down. This is done in change_mode() operation called from
// thermal_zone_device_disable()
//
    ret = thermal_zone_device_disable(data.tz);
    if (ret)
    return ret;
    return pm_runtime_force_suspend(data.dev);
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_resume(dev: *mut device) -> c_int {
    static int imx_thermal_resume(struct device *dev)
    {
    struct imx_thermal_data *data = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(data.dev);
    if (ret)
    return ret;
// Enabled thermal sensor after resume
    return thermal_zone_device_enable(data.tz);
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_runtime_suspend(dev: *mut device) -> c_int {
    static int imx_thermal_runtime_suspend(struct device *dev)
    {
    struct imx_thermal_data *data = dev_get_drvdata(dev);
    const struct thermal_soc_data *socdata = data.socdata;
    struct regmap *map = data.tempmon;
    int ret;
    ret = regmap_write(map, socdata.sensor_ctrl + REG_CLR,
    socdata.measure_temp_mask);
    if (ret)
    return ret;
    ret = regmap_write(map, socdata.sensor_ctrl + REG_SET,
    socdata.power_down_mask);
    if (ret)
    return ret;
    clk_disable_unprepare(data.thermal_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_thermal_runtime_resume(dev: *mut device) -> c_int {
    static int imx_thermal_runtime_resume(struct device *dev)
    {
    struct imx_thermal_data *data = dev_get_drvdata(dev);
    const struct thermal_soc_data *socdata = data.socdata;
    struct regmap *map = data.tempmon;
    int ret;
    ret = clk_prepare_enable(data.thermal_clk);
    if (ret)
    return ret;
    ret = regmap_write(map, socdata.sensor_ctrl + REG_CLR,
    socdata.power_down_mask);
    if (ret)
    goto disable_clk;
    ret = regmap_write(map, socdata.sensor_ctrl + REG_SET,
    socdata.measure_temp_mask);
    if (ret)
    goto disable_clk;
//
// According to the temp sensor designers, it may require up to ~17us
// to complete a measurement.
//
    usleep_range(20, 50);
    return 0;
    disable_clk:
    clk_disable_unprepare(data.thermal_clk);
    return ret;
    }
    static const struct dev_pm_ops imx_thermal_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(imx_thermal_suspend, imx_thermal_resume)
    RUNTIME_PM_OPS(imx_thermal_runtime_suspend,
    imx_thermal_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver imx_thermal = {
    .driver = {
    .name	= "imx_thermal",
    .pm	= pm_ptr(&imx_thermal_pm_ops),
    .of_match_table = of_imx_thermal_match,
    },
    .probe		= imx_thermal_probe,
    .remove		= imx_thermal_remove,
    };
    module_platform_driver(imx_thermal);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("Thermal driver for Freescale i.MX SoCs");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:imx-thermal");
