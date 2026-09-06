//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/st/st_thermal_memmap.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ST Thermal Sensor Driver for memory mapped sensors.
// Author: Ajit Pal Singh <ajitpal.singh@st.com>
//
// Copyright (C) 2003-2014 STMicroelectronics (R&D) Limited
//

pub const STIH416_MPE_CONF: c_uint = 0x0;
pub const STIH416_MPE_STATUS: c_uint = 0x4;
pub const STIH416_MPE_INT_THRESH: c_uint = 0x8;
pub const STIH416_MPE_INT_EN: c_uint = 0xC;
// Power control bits for the memory mapped thermal sensor

    static const struct reg_field st_mmap_thermal_regfields[MAX_REGFIELDS] = {
//
// According to the STIH416 MPE temp sensor data sheet -
// the PDN (Power Down Bit) and SRSTN (Soft Reset Bit) need to be
// written simultaneously for powering on and off the temperature
// sensor. regmap_update_bits() will be used to update the register.
//
    [INT_THRESH_HI]	= REG_FIELD(STIH416_MPE_INT_THRESH,	0,  7),
    [DCORRECT]	= REG_FIELD(STIH416_MPE_CONF,		5,  9),
    [OVERFLOW]	= REG_FIELD(STIH416_MPE_STATUS,		9,  9),
    [DATA]		= REG_FIELD(STIH416_MPE_STATUS,		11, 18),
    [INT_ENABLE]	= REG_FIELD(STIH416_MPE_INT_EN,		0,  0),
    };
#[no_mangle]
unsafe extern "C" fn st_mmap_thermal_trip_handler(irq: c_int, sdata: *mut c_void) -> irqreturn_t {
    static irqreturn_t st_mmap_thermal_trip_handler(int irq, void *sdata)
    {
    struct st_thermal_sensor *sensor = sdata;
    thermal_zone_device_update(sensor.thermal_dev,
    THERMAL_EVENT_UNSPECIFIED);
    return IRQ_HANDLED;
    }
// Private ops for the Memory Mapped based thermal sensors
    static int st_mmap_power_ctrl(struct st_thermal_sensor *sensor,
    enum st_thermal_power_state power_state)
    {
    let mut mask: c_uint = (THERMAL_PDN | THERMAL_SRSTN);
    let mut val: c_uint = power_state ? mask : 0;
    return regmap_update_bits(sensor.regmap, STIH416_MPE_CONF, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn st_mmap_alloc_regfields(sensor: *mut st_thermal_sensor) -> c_int {
    static int st_mmap_alloc_regfields(struct st_thermal_sensor *sensor)
    {
    struct device *dev = sensor.dev;
    struct regmap *regmap = sensor.regmap;
    const struct reg_field *reg_fields = sensor.cdata.reg_fields;
    sensor.int_thresh_hi = devm_regmap_field_alloc(dev, regmap,
    reg_fields[INT_THRESH_HI]);
    sensor.int_enable = devm_regmap_field_alloc(dev, regmap,
    reg_fields[INT_ENABLE]);
    if (IS_ERR(sensor.int_thresh_hi) || IS_ERR(sensor.int_enable)) {
    dev_err(dev, "failed to alloc mmap regfields\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st_mmap_enable_irq(sensor: *mut st_thermal_sensor) -> c_int {
    static int st_mmap_enable_irq(struct st_thermal_sensor *sensor)
    {
    int ret;
// Set upper critical threshold
    ret = regmap_field_write(sensor.int_thresh_hi,
    sensor.cdata.crit_temp -
    sensor.cdata.temp_adjust_val);
    if (ret)
    return ret;
    return regmap_field_write(sensor.int_enable, 1);
    }
#[no_mangle]
unsafe extern "C" fn st_mmap_register_enable_irq(sensor: *mut st_thermal_sensor) -> c_int {
    static int st_mmap_register_enable_irq(struct st_thermal_sensor *sensor)
    {
    struct device *dev = sensor.dev;
    struct platform_device *pdev = to_platform_device(dev);
    int ret;
    sensor.irq = platform_get_irq(pdev, 0);
    if (sensor.irq < 0)
    return sensor.irq;
    ret = devm_request_threaded_irq(dev, sensor.irq,
    core::ptr::null_mut(), st_mmap_thermal_trip_handler,
    IRQF_TRIGGER_RISING | IRQF_ONESHOT,
    dev.driver.name, sensor);
    if (ret)
    return ret;
    return st_mmap_enable_irq(sensor);
    }
    static const struct regmap_config st_416mpe_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn st_mmap_regmap_init(sensor: *mut st_thermal_sensor) -> c_int {
    static int st_mmap_regmap_init(struct st_thermal_sensor *sensor)
    {
    struct device *dev = sensor.dev;
    struct platform_device *pdev = to_platform_device(dev);
    sensor.mmio_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(sensor.mmio_base))
    return PTR_ERR(sensor.mmio_base);
    sensor.regmap = devm_regmap_init_mmio(dev, sensor.mmio_base,
    &st_416mpe_regmap_config);
    if (IS_ERR(sensor.regmap)) {
    dev_err(dev, "failed to initialise regmap\n");
    return PTR_ERR(sensor.regmap);
    }
    return 0;
    }
    static const struct st_thermal_sensor_ops st_mmap_sensor_ops = {
    .power_ctrl		= st_mmap_power_ctrl,
    .alloc_regfields	= st_mmap_alloc_regfields,
    .regmap_init		= st_mmap_regmap_init,
    .register_enable_irq	= st_mmap_register_enable_irq,
    .enable_irq		= st_mmap_enable_irq,
    };
// Compatible device data stih407 thermal sensor
    static const struct st_thermal_compat_data st_407_cdata = {
    .reg_fields		= st_mmap_thermal_regfields,
    .ops			= &st_mmap_sensor_ops,
    .calibration_val	= 16,
    .temp_adjust_val	= -95,
    .crit_temp		= 120,
    };
    static const struct of_device_id st_mmap_thermal_of_match[] = {
    { .compatible = "st,stih407-thermal",     .data = &st_407_cdata },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, st_mmap_thermal_of_match);
#[no_mangle]
unsafe extern "C" fn st_mmap_probe(pdev: *mut platform_device) -> c_int {
    static int st_mmap_probe(struct platform_device *pdev)
    {
    return st_thermal_register(pdev,  st_mmap_thermal_of_match);
    }
#[no_mangle]
unsafe extern "C" fn st_mmap_remove(pdev: *mut platform_device) {
    static void st_mmap_remove(struct platform_device *pdev)
    {
    st_thermal_unregister(pdev);
    }
    static struct platform_driver st_mmap_thermal_driver = {
    .driver = {
    .name	= "st_thermal_mmap",
    .pm     = pm_sleep_ptr(&st_thermal_pm_ops),
    .of_match_table = st_mmap_thermal_of_match,
    },
    .probe		= st_mmap_probe,
    .remove		= st_mmap_remove,
    };
    module_platform_driver(st_mmap_thermal_driver);
    MODULE_AUTHOR("STMicroelectronics (R&D) Limited <ajitpal.singh@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics STi SoC Thermal Sensor Driver");
    MODULE_LICENSE("GPL v2");
