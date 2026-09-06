//! Automatically rewritten from C to Rust
//! Source: drivers/gnss/ubx.c
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
// u-blox GNSS receiver driver
//
// Copyright (C) 2018 Johan Hovold <johan@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubx_data {
    pub vcc: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn ubx_set_active(gserial: *mut gnss_serial) -> c_int {
    static int ubx_set_active(struct gnss_serial *gserial)
    {
    struct ubx_data *data = gnss_serial_get_drvdata(gserial);
    int ret;
    ret = regulator_enable(data.vcc);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ubx_set_standby(gserial: *mut gnss_serial) -> c_int {
    static int ubx_set_standby(struct gnss_serial *gserial)
    {
    struct ubx_data *data = gnss_serial_get_drvdata(gserial);
    int ret;
    ret = regulator_disable(data.vcc);
    if (ret)
    return ret;
    return 0;
    }
    static int ubx_set_power(struct gnss_serial *gserial,
    enum gnss_serial_pm_state state)
    {
    switch (state) {
    case GNSS_SERIAL_ACTIVE:
    return ubx_set_active(gserial);
    case GNSS_SERIAL_OFF:
    case GNSS_SERIAL_STANDBY:
    return ubx_set_standby(gserial);
    }
    return -EINVAL;
    }
    static const struct gnss_serial_ops ubx_gserial_ops = {
    .set_power = ubx_set_power,
    };
#[no_mangle]
unsafe extern "C" fn ubx_probe(serdev: *mut serdev_device) -> c_int {
    static int ubx_probe(struct serdev_device *serdev)
    {
    struct gnss_serial *gserial;
    struct gpio_desc *safeboot;
    struct gpio_desc *reset;
    struct ubx_data *data;
    int ret;
    gserial = gnss_serial_allocate(serdev, sizeof(*data));
    if (IS_ERR(gserial)) {
    ret = PTR_ERR(gserial);
    return ret;
    }
    gserial.ops = &ubx_gserial_ops;
    gserial.gdev.type = GNSS_TYPE_UBX;
    data = gnss_serial_get_drvdata(gserial);
    data.vcc = devm_regulator_get(&serdev.dev, "vcc");
    if (IS_ERR(data.vcc)) {
    ret = PTR_ERR(data.vcc);
    goto err_free_gserial;
    }
    ret = devm_regulator_get_enable_optional(&serdev.dev, "v-bckp");
    if (ret < 0 && ret != -ENODEV)
    goto err_free_gserial;
// Deassert safeboot
    safeboot = devm_gpiod_get_optional(&serdev.dev, "safeboot", GPIOD_OUT_LOW);
    if (IS_ERR(safeboot)) {
    ret = PTR_ERR(safeboot);
    goto err_free_gserial;
    }
// Deassert reset
    reset = devm_gpiod_get_optional(&serdev.dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(reset)) {
    ret = PTR_ERR(reset);
    goto err_free_gserial;
    }
    ret = gnss_serial_register(gserial);
    if (ret)
    goto err_free_gserial;
    return 0;
    err_free_gserial:
    gnss_serial_free(gserial);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ubx_remove(serdev: *mut serdev_device) {
    static void ubx_remove(struct serdev_device *serdev)
    {
    struct gnss_serial *gserial = serdev_device_get_drvdata(serdev);
    gnss_serial_deregister(gserial);
    gnss_serial_free(gserial);
    }

    static const struct of_device_id ubx_of_match[] = {
    { .compatible = "u-blox,neo-6m" },
    { .compatible = "u-blox,neo-8" },
    { .compatible = "u-blox,neo-m8" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ubx_of_match);

    static struct serdev_device_driver ubx_driver = {
    .driver	= {
    .name		= "gnss-ubx",
    .of_match_table	= of_match_ptr(ubx_of_match),
    .pm		= &gnss_serial_pm_ops,
    },
    .probe	= ubx_probe,
    .remove	= ubx_remove,
    };
    module_serdev_device_driver(ubx_driver);
    MODULE_AUTHOR("Johan Hovold <johan@kernel.org>");
    MODULE_DESCRIPTION("u-blox GNSS receiver driver");
    MODULE_LICENSE("GPL v2");
