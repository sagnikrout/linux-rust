//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/thermal_mmio.c
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
// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_mmio {
    pub mmio_base: *mut void __iomem,
    pub mmio_base): *mut *mut u32 (read_mmio)(void __iomem,
    pub mask: u32,
    pub factor: c_int,
}

#[no_mangle]
unsafe extern "C" fn thermal_mmio_readb(mmio_base: *mut void __iomem) -> u32 {
    static u32 thermal_mmio_readb(void __iomem *mmio_base)
    {
    return readb(mmio_base);
    }
#[no_mangle]
unsafe extern "C" fn thermal_mmio_get_temperature(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int thermal_mmio_get_temperature(struct thermal_zone_device *tz, int *temp)
    {
    int t;
    struct thermal_mmio *sensor = thermal_zone_device_priv(tz);
    t = sensor.read_mmio(sensor.mmio_base) & sensor.mask;
    t *= sensor.factor;
// temp = t;
    return 0;
    }
    static const struct thermal_zone_device_ops thermal_mmio_ops = {
    .get_temp = thermal_mmio_get_temperature,
    };
#[no_mangle]
unsafe extern "C" fn thermal_mmio_probe(pdev: *mut platform_device) -> c_int {
    static int thermal_mmio_probe(struct platform_device *pdev)
    {
    struct thermal_mmio *sensor;
    int (*sensor_init_func)(struct platform_device *pdev,
    struct thermal_mmio *sensor);
    struct thermal_zone_device *thermal_zone;
    int ret;
    int temperature;
    sensor = devm_kzalloc(&pdev.dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.mmio_base = devm_platform_get_and_ioremap_resource(pdev, 0, core::ptr::null_mut());
    if (IS_ERR(sensor.mmio_base))
    return PTR_ERR(sensor.mmio_base);
    sensor_init_func = device_get_match_data(&pdev.dev);
    if (sensor_init_func) {
    ret = sensor_init_func(pdev, sensor);
    if (ret) {
    dev_err(&pdev.dev,
    "failed to initialize sensor (%d)\n",
    ret);
    return ret;
    }
    }
    thermal_zone = devm_thermal_of_zone_register(&pdev.dev,
    0,
    sensor,
    &thermal_mmio_ops);
    if (IS_ERR(thermal_zone)) {
    dev_err(&pdev.dev,
    "failed to register sensor (%ld)\n",
    PTR_ERR(thermal_zone));
    return PTR_ERR(thermal_zone);
    }
    thermal_mmio_get_temperature(thermal_zone, &temperature);
    dev_info(&pdev.dev,
    "thermal mmio sensor %s registered, current temperature: %d\n",
    pdev.name, temperature);
    return 0;
    }
    static int al_thermal_init(struct platform_device *pdev,
    struct thermal_mmio *sensor)
    {
    sensor.read_mmio = thermal_mmio_readb;
    sensor.mask = 0xff;
    sensor.factor = 1000;
    return 0;
    }
    static const struct of_device_id thermal_mmio_id_table[] = {
    { .compatible = "amazon,al-thermal", .data = al_thermal_init},
    {}
    };
    MODULE_DEVICE_TABLE(of, thermal_mmio_id_table);
    static struct platform_driver thermal_mmio_driver = {
    .probe = thermal_mmio_probe,
    .driver = {
    .name = "thermal-mmio",
    .of_match_table = thermal_mmio_id_table,
    },
    };
    module_platform_driver(thermal_mmio_driver);
    MODULE_AUTHOR("Talel Shenhar <talel@amazon.com>");
    MODULE_DESCRIPTION("Thermal MMIO Driver");
    MODULE_LICENSE("GPL v2");
