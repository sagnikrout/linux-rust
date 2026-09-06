//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/qcom/qcom-spmi-mbg-tm.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

pub const MBG_TEMP_MON2_FAULT_STATUS: c_uint = 0x50;

pub const MON_FAULT_LVL1_UPR: c_uint = 0x5;
pub const MON2_LVL1_UP_THRESH: c_uint = 0x59;
pub const MBG_TEMP_MON2_MISC_CFG: c_uint = 0x5f;

pub const MBG_TEMP_STEP_MV: c_int = 8;
pub const MBG_TEMP_DEFAULT_TEMP_MV: c_int = 600;
pub const MBG_TEMP_CONSTANT: c_int = 1000;
pub const MBG_MIN_TRIP_TEMP: c_int = 25000;
pub const MBG_MAX_SUPPORTED_TEMP: c_int = 160000;
//
// struct mbg_tm_chip - MBG thermal monitor device data.
// @map: regmap for accessing MBG thermal registers.
// @dev: mbg_tm_chip device.
// @tz_dev: thermal zone device registered with the thermal framework.
// @lock: mbg_tm_chip lock for set trip temperature.
// @base: base register offset for this MBG instance
// @irq: interrupt line used to signal threshold events
// @last_temp: last measured temperature.
// @last_thres_crossed: indicates whether the last interrupt crossed a threshold
// @adc: IIO ADC channel used for temperature sensing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbg_tm_chip {
    pub map: *mut regmap,
    pub dev: *mut device,
    pub tz_dev: *mut thermal_zone_device,
    pub lock: mutex,
    pub base: c_uint,
    pub irq: c_int,
    pub last_temp: c_int,
    pub last_thres_crossed: bool,
    pub adc: *mut iio_channel,
}

//
// struct mbg_map_table - temperature to voltage mapping entry
// @min_temp: minimum temperature supported by this mapping entry
// @vtemp0: reference voltage or ADC code corresponding to the temperature
// @tc: temperature coefficient used for conversion calculations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbg_map_table {
    pub min_temp: c_int,
    pub vtemp0: c_int,
    pub tc: c_int,
}

    static const struct mbg_map_table map_table[] = {
    { -60000, 4337, 1967 },
    { -40000, 4731, 1964 },
    { -20000, 5124, 1957 },
    { 0,      5515, 1949 },
    { 20000,  5905, 1940 },
    { 40000,  6293, 1930 },
    { 60000,  6679, 1921 },
    { 80000,  7064, 1910 },
    { 100000, 7446, 1896 },
    { 120000, 7825, 1878 },
    { 140000, 8201, 1859 },
    };
#[no_mangle]
unsafe extern "C" fn mbg_tm_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int mbg_tm_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    struct mbg_tm_chip *chip = thermal_zone_device_priv(tz);
    int ret, milli_celsius;
    scoped_guard(mutex, &chip.lock) {
    if (chip.last_thres_crossed) {
    dev_dbg(chip.dev, "last_temp: %d\n", chip.last_temp);
    chip.last_thres_crossed = false;
// temp = chip->last_temp;
    return 0;
    }
    }
    ret = iio_read_channel_processed(chip.adc, &milli_celsius);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to read iio channel with %d\n", ret);
    return ret;
    }
// temp = milli_celsius;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn temp_to_vtemp_mv(temp: c_int) -> c_int {
    static int temp_to_vtemp_mv(int temp)
    {
    int idx, vtemp, tc = 0, t0 = 0, vtemp0 = 0;
    for (idx = 0; idx < ARRAY_SIZE(map_table); idx++)
    if (temp >= map_table[idx].min_temp &&
    temp < (map_table[idx].min_temp + 20000)) {
    tc = map_table[idx].tc;
    t0 = map_table[idx].min_temp;
    vtemp0 = map_table[idx].vtemp0;
    break;
    }
//
// Formula to calculate vtemp(mV) from a given temp
// vtemp = (temp - minT) * tc + vtemp0
// tc, t0 and vtemp0 values are mentioned in the map_table array.
//
    vtemp = ((temp - t0) * tc + vtemp0 * 100000) / 1000000;
// step size is 8mV
    return abs(vtemp - MBG_TEMP_DEFAULT_TEMP_MV) / MBG_TEMP_STEP_MV;
    }
    static int mbg_tm_set_trip_temp(struct thermal_zone_device *tz, int low_temp,
    int temp)
    {
    struct mbg_tm_chip *chip = thermal_zone_device_priv(tz);
    let mut ret: c_int = 0;
    guard(mutex)(&chip.lock);
// The HW has a limitation that the trip set must be above 25C
    if (temp > MBG_MIN_TRIP_TEMP && temp < MBG_MAX_SUPPORTED_TEMP) {
    ret = regmap_write(chip.map, chip.base + MON2_LVL1_UP_THRESH,
    temp_to_vtemp_mv(temp));
    if (ret < 0)
    return ret;
    ret = regmap_set_bits(chip.map, chip.base + MBG_TEMP_MON2_MISC_CFG,
    MON2_UP_THRESH_EN);
    if (ret < 0)
    return ret;
    } else {
    dev_err(chip.dev, "Set trip b/w 25C and 160C\n");
    ret = regmap_clear_bits(chip.map, chip.base + MBG_TEMP_MON2_MISC_CFG,
    MON2_UP_THRESH_EN);
    return -ERANGE;
    }
//
// Configure the last_temp one degree higher, to ensure the
// violated temp is returned to thermal framework when it reads
// temperature for the first time after the violation happens.
// This is needed to account for the inaccuracy in the conversion
// formula used which leads to the thermal framework setting back
// the same thresholds in case the temperature it reads does not
// show violation.
//
    chip.last_temp = temp + MBG_TEMP_CONSTANT;
    return ret;
    }
    static const struct thermal_zone_device_ops mbg_tm_ops = {
    .get_temp = mbg_tm_get_temp,
    .set_trips = mbg_tm_set_trip_temp,
    };
#[no_mangle]
unsafe extern "C" fn mbg_tm_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mbg_tm_isr(int irq, void *data)
    {
    struct mbg_tm_chip *chip = data;
    int ret, val;
    scoped_guard(mutex, &chip.lock) {
    ret = regmap_read(chip.map, chip.base + MBG_TEMP_MON2_FAULT_STATUS, &val);
    if (ret < 0)
    return IRQ_HANDLED;
    if (FIELD_GET(MON_FAULT_STATUS_MASK, val) == MON_FAULT_LVL1_UPR)
    chip.last_thres_crossed = true;
    }
    if (FIELD_GET(MON_FAULT_STATUS_MASK, val) == MON_FAULT_LVL1_UPR) {
    dev_dbg(chip.dev, "Notifying Thermal, fault status=%d\n", val);
    thermal_zone_device_update(chip.tz_dev, THERMAL_TRIP_VIOLATED);
    } else {
    dev_dbg(chip.dev, "Lvl1 upper threshold not violated, ignoring interrupt\n");
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mbg_tm_probe(pdev: *mut platform_device) -> c_int {
    static int mbg_tm_probe(struct platform_device *pdev)
    {
    struct mbg_tm_chip *chip;
    struct device_node *node = pdev.dev.of_node;
    u32 res;
    int ret;
    chip = devm_kzalloc(&pdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dev = &pdev.dev;
    mutex_init(&chip.lock);
    chip.map = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!chip.map)
    return -ENXIO;
    ret = device_property_read_u32(chip.dev, "reg", &res);
    if (ret < 0)
    return dev_err_probe(chip.dev, ret, "Couldn't read reg property\n");
    chip.base = res;
    chip.irq = platform_get_irq(pdev, 0);
    if (chip.irq < 0)
    return dev_err_probe(chip.dev, chip.irq, "Failed to get irq\n");
    chip.adc = devm_iio_channel_get(&pdev.dev, "thermal");
    if (IS_ERR(chip.adc))
    return dev_err_probe(chip.dev, PTR_ERR(chip.adc), "Failed to get adc channel\n");
    chip.tz_dev = devm_thermal_of_zone_register(chip.dev, 0, chip, &mbg_tm_ops);
    if (IS_ERR(chip.tz_dev))
    return dev_err_probe(chip.dev, PTR_ERR(chip.tz_dev),
    "Failed to register sensor\n");
    return devm_request_threaded_irq(&pdev.dev, chip.irq, core::ptr::null_mut(), mbg_tm_isr, IRQF_ONESHOT,
    node.name, chip);
    }
    static const struct of_device_id mbg_tm_match_table[] = {
    { .compatible = "qcom,pm8775-mbg-tm" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mbg_tm_match_table);
    static struct platform_driver mbg_tm_driver = {
    .driver = {
    .name = "qcom-spmi-mbg-tm",
    .of_match_table = mbg_tm_match_table,
    },
    .probe = mbg_tm_probe,
    };
    module_platform_driver(mbg_tm_driver);
    MODULE_DESCRIPTION("PMIC MBG Temperature monitor driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
