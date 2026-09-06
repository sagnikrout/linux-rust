//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mc33880.c
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
// MC33880 high-side/low-side switch GPIO driver
// Copyright (c) 2009 Intel Corporation
//
// Supports:
// Freescale MC33880 high-side/low-side switch
//

//
// Pin configurations, see MAX7301 datasheet page 6
//
pub const PIN_CONFIG_MASK: c_uint = 0x03;
pub const PIN_CONFIG_IN_PULLUP: c_uint = 0x03;
pub const PIN_CONFIG_IN_WO_PULLUP: c_uint = 0x02;
pub const PIN_CONFIG_OUT: c_uint = 0x01;
pub const PIN_NUMBER: c_int = 8;
//
// Some registers must be read back to modify.
// To save time we cache them here in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc33880 {
    pub /: *mut *mut mutex lock; / protect from simultaneous accesses,
    pub port_config: u8,
    pub chip: gpio_chip,
    pub spi: *mut spi_device,
}

#[no_mangle]
unsafe extern "C" fn mc33880_write_config(mc: *mut mc33880) -> c_int {
    static int mc33880_write_config(struct mc33880 *mc)
    {
    return spi_write(mc.spi, &mc.port_config, sizeof(mc.port_config));
    }
#[no_mangle]
unsafe extern "C" fn __mc33880_set(mc: *mut mc33880, offset: unsigned, value: c_int) -> c_int {
    static int __mc33880_set(struct mc33880 *mc, unsigned offset, int value)
    {
    if (value)
    mc.port_config |= 1 << offset;
    else
    mc.port_config &= ~(1 << offset);
    return mc33880_write_config(mc);
    }
#[no_mangle]
unsafe extern "C" fn mc33880_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int mc33880_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct mc33880 *mc = gpiochip_get_data(chip);
    int ret;
    mutex_lock(&mc.lock);
    ret = __mc33880_set(mc, offset, value);
    mutex_unlock(&mc.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc33880_probe(spi: *mut spi_device) -> c_int {
    static int mc33880_probe(struct spi_device *spi)
    {
    struct mc33880 *mc;
    struct mc33880_platform_data *pdata;
    int ret;
    pdata = dev_get_platdata(&spi.dev);
    if (!pdata || !pdata.base) {
    dev_dbg(&spi.dev, "incorrect or missing platform data\n");
    return -EINVAL;
    }
//
// bits_per_word cannot be configured in platform data
//
    spi.bits_per_word = 8;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    mc = devm_kzalloc(&spi.dev, sizeof(struct mc33880), GFP_KERNEL);
    if (!mc)
    return -ENOMEM;
    mutex_init(&mc.lock);
    spi_set_drvdata(spi, mc);
    mc.spi = spi;
    mc.chip.label = DRIVER_NAME;
    mc.chip.set = mc33880_set;
    mc.chip.base = pdata.base;
    mc.chip.ngpio = PIN_NUMBER;
    mc.chip.can_sleep = true;
    mc.chip.parent = &spi.dev;
    mc.chip.owner = THIS_MODULE;
    mc.port_config = 0x00;
// write twice, because during initialisation the first setting
// is just for testing SPI communication, and the second is the
// "real" configuration
//
    ret = mc33880_write_config(mc);
    mc.port_config = 0x00;
    if (!ret)
    ret = mc33880_write_config(mc);
    if (ret) {
    dev_err(&spi.dev, "Failed writing to " DRIVER_NAME ": %d\n",
    ret);
    goto exit_destroy;
    }
    ret = gpiochip_add_data(&mc.chip, mc);
    if (ret)
    goto exit_destroy;
    return ret;
    exit_destroy:
    mutex_destroy(&mc.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mc33880_remove(spi: *mut spi_device) {
    static void mc33880_remove(struct spi_device *spi)
    {
    struct mc33880 *mc;
    mc = spi_get_drvdata(spi);
    gpiochip_remove(&mc.chip);
    mutex_destroy(&mc.lock);
    }
    static struct spi_driver mc33880_driver = {
    .driver = {
    .name		= DRIVER_NAME,
    },
    .probe		= mc33880_probe,
    .remove		= mc33880_remove,
    };
#[no_mangle]
unsafe extern "C" fn mc33880_init() -> int __init {
    static int __init mc33880_init(void)
    {
    return spi_register_driver(&mc33880_driver);
    }
// register after spi postcore initcall and before
// subsys initcalls that may rely on these GPIOs
//
    subsys_initcall(mc33880_init);
#[no_mangle]
unsafe extern "C" fn mc33880_exit() -> void __exit {
    static void __exit mc33880_exit(void)
    {
    spi_unregister_driver(&mc33880_driver);
    }
    module_exit(mc33880_exit);
    MODULE_AUTHOR("Mocean Laboratories <info@mocean-labs.com>");
    MODULE_DESCRIPTION("MC33880 high-side/low-side switch GPIO driver");
    MODULE_LICENSE("GPL v2");
