//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/sprd_thermal.c
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
// Copyright (C) 2020 Spreadtrum Communications Inc.

pub const SPRD_THM_CTL: c_uint = 0x0;
pub const SPRD_THM_INT_EN: c_uint = 0x4;
pub const SPRD_THM_INT_STS: c_uint = 0x8;
pub const SPRD_THM_INT_RAW_STS: c_uint = 0xc;
pub const SPRD_THM_DET_PERIOD: c_uint = 0x10;
pub const SPRD_THM_INT_CLR: c_uint = 0x14;
pub const SPRD_THM_INT_CLR_ST: c_uint = 0x18;
pub const SPRD_THM_MON_PERIOD: c_uint = 0x4c;
pub const SPRD_THM_MON_CTL: c_uint = 0x50;
pub const SPRD_THM_INTERNAL_STS1: c_uint = 0x54;
pub const SPRD_THM_RAW_READ_MSK: c_uint = 0x3ff;

// bits definitions for register THM_CTL

// bits definitions for register THM_INT_CTL

pub const SPRD_THM_OTP_TRIP_SHIFT: c_int = 10;
// bits definitions for register SPRD_THM_INTERNAL_STS1

pub const SPRD_THM_DET_PERIOD_DATA: c_uint = 0x800;

pub const SPRD_THM_MON_MODE: c_uint = 0x7;

pub const SPRD_THM_MON_PERIOD_DATA: c_uint = 0x10;

// thermal sensor calibration parameters

pub const SPRD_THM_TEMP_HIGH: c_int = 120000;
pub const SPRD_THM_OTP_TEMP: c_int = 120000;
pub const SPRD_THM_HOT_TEMP: c_int = 75000;
pub const SPRD_THM_RAW_DATA_LOW: c_int = 0;
pub const SPRD_THM_RAW_DATA_HIGH: c_int = 1000;
pub const SPRD_THM_SEN_NUM: c_int = 8;
pub const SPRD_THM_DT_OFFSET: c_int = 24;
pub const SPRD_THM_RATION_OFFSET: c_int = 17;
pub const SPRD_THM_RATION_SIGN: c_int = 16;
pub const SPRD_THM_RDYST_POLLING_TIME: c_int = 10;
pub const SPRD_THM_RDYST_TIMEOUT: c_int = 700;
pub const SPRD_THM_TEMP_READY_POLL_TIME: c_int = 10000;
pub const SPRD_THM_TEMP_READY_TIMEOUT: c_int = 600000;
pub const SPRD_THM_MAX_SENSOR: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_thermal_sensor {
    pub tzd: *mut thermal_zone_device,
    pub data: *mut sprd_thermal_data,
    pub dev: *mut device,
    pub cal_slope: c_int,
    pub cal_offset: c_int,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_thermal_data {
    pub var_data: *const sprd_thm_variant_data,
    pub sensor: [*mut sprd_thermal_sensor; SPRD_THM_MAX_SENSOR],
    pub clk: *mut clk,
    pub base: *mut void __iomem,
    pub ratio_off: u32,
    pub ratio_sign: c_int,
    pub nr_sensors: c_int,
}

//
// The conversion between ADC and temperature is based on linear relationship,
// and use idea_k to specify the slope and ideal_b to specify the offset.
//
// Since different Spreadtrum SoCs have different ideal_k and ideal_b,
// we should save ideal_k and ideal_b in the device data structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_thm_variant_data {
    pub ideal_k: u32,
    pub ideal_b: u32,
}

    static const struct sprd_thm_variant_data ums512_data = {
    .ideal_k = 262,
    .ideal_b = 66400,
    };
#[no_mangle]
pub unsafe extern "C" fn sprd_thm_update_bits(reg: *mut void __iomem, mask: u32, val: u32) {
    static inline void sprd_thm_update_bits(void __iomem *reg, u32 mask, u32 val)
    {
    u32 tmp, orig;
    orig = readl(reg);
    tmp = orig & ~mask;
    tmp |= val & mask;
    writel(tmp, reg);
    }
    static int sprd_thm_cal_read(struct device_node *np, const char *cell_id,
    u32 *val)
    {
    struct nvmem_cell *cell;
    void *buf;
    size_t len;
    cell = of_nvmem_cell_get(np, cell_id);
    if (IS_ERR(cell))
    return PTR_ERR(cell);
    buf = nvmem_cell_read(cell, &len);
    nvmem_cell_put(cell);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    if (len > sizeof(u32)) {
    kfree(buf);
    return -EINVAL;
    }
    memcpy(val, buf, len);
    kfree(buf);
    return 0;
    }
    static int sprd_thm_sensor_calibration(struct device_node *np,
    struct sprd_thermal_data *thm,
    struct sprd_thermal_sensor *sen)
    {
    int ret;
//
// According to thermal datasheet, the default calibration offset is 64,
// and the default ratio is 1000.
//
    let mut dt_offset: c_int = 64, ratio = 1000;
    ret = sprd_thm_cal_read(np, "sen_delta_cal", &dt_offset);
    if (ret)
    return ret;
    ratio += thm.ratio_sign * thm.ratio_off;
//
// According to the ideal slope K and ideal offset B, combined with
// calibration value of thermal from efuse, then calibrate the real
// slope k and offset b:
// k_cal = (k * ratio) / 1000.
// b_cal = b + (dt_offset - 64) * 500.
//
    sen.cal_slope = (thm.var_data.ideal_k * ratio) / 1000;
    sen.cal_offset = thm.var_data.ideal_b + (dt_offset - 128) * 250;
    return 0;
    }
    static int sprd_thm_rawdata_to_temp(struct sprd_thermal_sensor *sen,
    u32 rawdata)
    {
    rawdata = clamp(rawdata, SPRD_THM_RAW_DATA_LOW, SPRD_THM_RAW_DATA_HIGH);
//
// According to the thermal datasheet, the formula of converting
// adc value to the temperature value should be:
// T_final = k_cal * x - b_cal.
//
    return sen.cal_slope * rawdata - sen.cal_offset;
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_temp_to_rawdata(temp: c_int, sen: *mut sprd_thermal_sensor) -> c_int {
    static int sprd_thm_temp_to_rawdata(int temp, struct sprd_thermal_sensor *sen)
    {
    u32 val;
    temp = clamp(temp, SPRD_THM_TEMP_LOW, SPRD_THM_TEMP_HIGH);
//
// According to the thermal datasheet, the formula of converting
// adc value to the temperature value should be:
// T_final = k_cal * x - b_cal.
//
    val = (temp + sen.cal_offset) / sen.cal_slope;
    return min(val, SPRD_THM_RAW_DATA_HIGH - 1);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_read_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int sprd_thm_read_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct sprd_thermal_sensor *sen = thermal_zone_device_priv(tz);
    u32 data;
    data = readl(sen.data.base + SPRD_THM_TEMP(sen.id)) &
    SPRD_THM_RAW_READ_MSK;
// temp = sprd_thm_rawdata_to_temp(sen, data);
    return 0;
    }
    static const struct thermal_zone_device_ops sprd_thm_ops = {
    .get_temp = sprd_thm_read_temp,
    };
#[no_mangle]
unsafe extern "C" fn sprd_thm_poll_ready_status(thm: *mut sprd_thermal_data) -> c_int {
    static int sprd_thm_poll_ready_status(struct sprd_thermal_data *thm)
    {
    u32 val;
    int ret;
//
// Wait for thermal ready status before configuring thermal parameters.
//
    ret = readl_poll_timeout(thm.base + SPRD_THM_CTL, val,
    !(val & SPRD_THM_SET_RDY_ST),
    SPRD_THM_RDYST_POLLING_TIME,
    SPRD_THM_RDYST_TIMEOUT);
    if (ret)
    return ret;
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL, SPRD_THM_MON_EN,
    SPRD_THM_MON_EN);
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL, SPRD_THM_SET_RDY,
    SPRD_THM_SET_RDY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_wait_temp_ready(thm: *mut sprd_thermal_data) -> c_int {
    static int sprd_thm_wait_temp_ready(struct sprd_thermal_data *thm)
    {
    u32 val;
// Wait for first temperature data ready before reading temperature
    return readl_poll_timeout(thm.base + SPRD_THM_INTERNAL_STS1, val,
    !(val & SPRD_THM_TEMPER_RDY),
    SPRD_THM_TEMP_READY_POLL_TIME,
    SPRD_THM_TEMP_READY_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_set_ready(thm: *mut sprd_thermal_data) -> c_int {
    static int sprd_thm_set_ready(struct sprd_thermal_data *thm)
    {
    int ret;
    ret = sprd_thm_poll_ready_status(thm);
    if (ret)
    return ret;
//
// Clear interrupt status, enable thermal interrupt and enable thermal.
//
// The SPRD thermal controller integrates a hardware interrupt signal,
// which means if the temperature is overheat, it will generate an
// interrupt and notify the event to PMIC automatically to shutdown the
// system. So here we should enable the interrupt bits, though we have
// not registered an irq handler.
//
    writel(SPRD_THM_INT_CLR_MASK, thm.base + SPRD_THM_INT_CLR);
    sprd_thm_update_bits(thm.base + SPRD_THM_INT_EN,
    SPRD_THM_BIT_INT_EN, SPRD_THM_BIT_INT_EN);
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL,
    SPRD_THM_EN, SPRD_THM_EN);
    return 0;
    }
    static void sprd_thm_sensor_init(struct sprd_thermal_data *thm,
    struct sprd_thermal_sensor *sen)
    {
    u32 otp_rawdata, hot_rawdata;
    otp_rawdata = sprd_thm_temp_to_rawdata(SPRD_THM_OTP_TEMP, sen);
    hot_rawdata = sprd_thm_temp_to_rawdata(SPRD_THM_HOT_TEMP, sen);
// Enable the sensor' overheat temperature protection interrupt
    sprd_thm_update_bits(thm.base + SPRD_THM_INT_EN,
    SPRD_THM_SEN_OVERHEAT_ALARM_EN(sen.id),
    SPRD_THM_SEN_OVERHEAT_ALARM_EN(sen.id));
// Set the sensor' overheat and hot threshold temperature
    sprd_thm_update_bits(thm.base + SPRD_THM_THRES(sen.id),
    SPRD_THM_THRES_MASK,
    (otp_rawdata << SPRD_THM_OTP_TRIP_SHIFT) |
    hot_rawdata);
// Enable the corresponding sensor
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL, SPRD_THM_SEN(sen.id),
    SPRD_THM_SEN(sen.id));
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_para_config(thm: *mut sprd_thermal_data) {
    static void sprd_thm_para_config(struct sprd_thermal_data *thm)
    {
// Set the period of two valid temperature detection action
    sprd_thm_update_bits(thm.base + SPRD_THM_DET_PERIOD,
    SPRD_THM_DET_PERIOD_MASK, SPRD_THM_DET_PERIOD);
// Set the sensors' monitor mode
    sprd_thm_update_bits(thm.base + SPRD_THM_MON_CTL,
    SPRD_THM_MON_MODE_MASK, SPRD_THM_MON_MODE);
// Set the sensors' monitor period
    sprd_thm_update_bits(thm.base + SPRD_THM_MON_PERIOD,
    SPRD_THM_MON_PERIOD_MASK, SPRD_THM_MON_PERIOD);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_toggle_sensor(sen: *mut sprd_thermal_sensor, on: bool) {
    static void sprd_thm_toggle_sensor(struct sprd_thermal_sensor *sen, bool on)
    {
    struct thermal_zone_device *tzd = sen.tzd;
    if (on)
    thermal_zone_device_enable(tzd);
    else
    thermal_zone_device_disable(tzd);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_probe(pdev: *mut platform_device) -> c_int {
    static int sprd_thm_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device_node *sen_child;
    struct sprd_thermal_data *thm;
    struct sprd_thermal_sensor *sen;
    const struct sprd_thm_variant_data *pdata;
    int ret, i;
    u32 val;
    pdata = of_device_get_match_data(&pdev.dev);
    if (!pdata) {
    dev_err(&pdev.dev, "No matching driver data found\n");
    return -EINVAL;
    }
    thm = devm_kzalloc(&pdev.dev, sizeof(*thm), GFP_KERNEL);
    if (!thm)
    return -ENOMEM;
    thm.var_data = pdata;
    thm.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(thm.base))
    return PTR_ERR(thm.base);
    thm.nr_sensors = of_get_child_count(np);
    if (thm.nr_sensors == 0 || thm.nr_sensors > SPRD_THM_MAX_SENSOR) {
    dev_err(&pdev.dev, "incorrect sensor count\n");
    return -EINVAL;
    }
    thm.clk = devm_clk_get_enabled(&pdev.dev, "enable");
    if (IS_ERR(thm.clk)) {
    dev_err(&pdev.dev, "failed to get enable clock\n");
    return PTR_ERR(thm.clk);
    }
    sprd_thm_para_config(thm);
    ret = sprd_thm_cal_read(np, "thm_sign_cal", &val);
    if (ret)
    return ret;
    if (val > 0)
    thm.ratio_sign = -1;
    else
    thm.ratio_sign = 1;
    ret = sprd_thm_cal_read(np, "thm_ratio_cal", &thm.ratio_off);
    if (ret)
    return ret;
    for_each_child_of_node(np, sen_child) {
    sen = devm_kzalloc(&pdev.dev, sizeof(*sen), GFP_KERNEL);
    if (!sen) {
    ret = -ENOMEM;
    goto of_put;
    }
    sen.data = thm;
    sen.dev = &pdev.dev;
    ret = of_property_read_u32(sen_child, "reg", &sen.id);
    if (ret) {
    dev_err(&pdev.dev, "get sensor reg failed");
    goto of_put;
    }
    ret = sprd_thm_sensor_calibration(sen_child, thm, sen);
    if (ret) {
    dev_err(&pdev.dev, "efuse cal analysis failed");
    goto of_put;
    }
    sprd_thm_sensor_init(thm, sen);
    sen.tzd = devm_thermal_of_zone_register(sen.dev,
    sen.id,
    sen,
    &sprd_thm_ops);
    if (IS_ERR(sen.tzd)) {
    dev_err(&pdev.dev, "register thermal zone failed %d\n",
    sen.id);
    ret = PTR_ERR(sen.tzd);
    goto of_put;
    }
    thm.sensor[sen.id] = sen;
    }
// sen_child set to NULL at this point
    ret = sprd_thm_set_ready(thm);
    if (ret)
    goto of_put;
    ret = sprd_thm_wait_temp_ready(thm);
    if (ret)
    goto of_put;
    for (i = 0; i < thm.nr_sensors; i++)
    sprd_thm_toggle_sensor(thm.sensor[i], true);
    platform_set_drvdata(pdev, thm);
    return 0;
    of_put:
    of_node_put(sen_child);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn sprd_thm_hw_suspend(thm: *mut sprd_thermal_data) {
    static void sprd_thm_hw_suspend(struct sprd_thermal_data *thm)
    {
    int i;
    for (i = 0; i < thm.nr_sensors; i++) {
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL,
    SPRD_THM_SEN(thm.sensor[i].id), 0);
    }
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL,
    SPRD_THM_EN, 0x0);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_suspend(dev: *mut device) -> c_int {
    static int sprd_thm_suspend(struct device *dev)
    {
    struct sprd_thermal_data *thm = dev_get_drvdata(dev);
    int i;
    for (i = 0; i < thm.nr_sensors; i++)
    sprd_thm_toggle_sensor(thm.sensor[i], false);
    sprd_thm_hw_suspend(thm);
    clk_disable_unprepare(thm.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_hw_resume(thm: *mut sprd_thermal_data) -> c_int {
    static int sprd_thm_hw_resume(struct sprd_thermal_data *thm)
    {
    int ret, i;
    for (i = 0; i < thm.nr_sensors; i++) {
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL,
    SPRD_THM_SEN(thm.sensor[i].id),
    SPRD_THM_SEN(thm.sensor[i].id));
    }
    ret = sprd_thm_poll_ready_status(thm);
    if (ret)
    return ret;
    writel(SPRD_THM_INT_CLR_MASK, thm.base + SPRD_THM_INT_CLR);
    sprd_thm_update_bits(thm.base + SPRD_THM_CTL,
    SPRD_THM_EN, SPRD_THM_EN);
    return sprd_thm_wait_temp_ready(thm);
    }
#[no_mangle]
unsafe extern "C" fn sprd_thm_resume(dev: *mut device) -> c_int {
    static int sprd_thm_resume(struct device *dev)
    {
    struct sprd_thermal_data *thm = dev_get_drvdata(dev);
    int ret, i;
    ret = clk_prepare_enable(thm.clk);
    if (ret)
    return ret;
    ret = sprd_thm_hw_resume(thm);
    if (ret)
    goto disable_clk;
    for (i = 0; i < thm.nr_sensors; i++)
    sprd_thm_toggle_sensor(thm.sensor[i], true);
    return 0;
    disable_clk:
    clk_disable_unprepare(thm.clk);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn sprd_thm_remove(pdev: *mut platform_device) {
    static void sprd_thm_remove(struct platform_device *pdev)
    {
    struct sprd_thermal_data *thm = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < thm.nr_sensors; i++) {
    sprd_thm_toggle_sensor(thm.sensor[i], false);
    devm_thermal_of_zone_unregister(&pdev.dev,
    thm.sensor[i].tzd);
    }
    }
    static const struct of_device_id sprd_thermal_of_match[] = {
    { .compatible = "sprd,ums512-thermal", .data = &ums512_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, sprd_thermal_of_match);
    static const struct dev_pm_ops sprd_thermal_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(sprd_thm_suspend, sprd_thm_resume)
    };
    static struct platform_driver sprd_thermal_driver = {
    .probe = sprd_thm_probe,
    .remove = sprd_thm_remove,
    .driver = {
    .name = "sprd-thermal",
    .pm = &sprd_thermal_pm_ops,
    .of_match_table = sprd_thermal_of_match,
    },
    };
    module_platform_driver(sprd_thermal_driver);
    MODULE_AUTHOR("Freeman Liu <freeman.liu@unisoc.com>");
    MODULE_DESCRIPTION("Spreadtrum thermal driver");
    MODULE_LICENSE("GPL v2");
