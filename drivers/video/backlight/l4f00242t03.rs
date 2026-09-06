//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/l4f00242t03.c
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
// l4f00242t03.c -- support for Epson L4F00242T03 LCD
//
// Copyright 2007-2009 Freescale Semiconductor, Inc. All Rights Reserved.
//
// Copyright (c) 2009 Alberto Panizzo <maramaopercheseimorto@gmail.com>
// Inspired by Marek Vasut work in l4f00242t03.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct l4f00242t03_priv {
    pub spi: *mut spi_device,
    pub ld: *mut lcd_device,
    pub lcd_state: c_int,
    pub io_reg: *mut regulator,
    pub core_reg: *mut regulator,
    pub reset: *mut gpio_desc,
    pub enable: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn l4f00242t03_reset(gpiod: *mut gpio_desc) {
    static void l4f00242t03_reset(struct gpio_desc *gpiod)
    {
    pr_debug("l4f00242t03_reset.\n");
    gpiod_set_value(gpiod, 1);
    mdelay(100);
    gpiod_set_value(gpiod, 0);
    mdelay(10);	/* tRES >= 100us */
    gpiod_set_value(gpiod, 1);
    mdelay(20);
    }

#[no_mangle]
unsafe extern "C" fn l4f00242t03_lcd_init(spi: *mut spi_device) {
    static void l4f00242t03_lcd_init(struct spi_device *spi)
    {
    struct l4f00242t03_priv *priv = spi_get_drvdata(spi);
    const u16 cmd[] = { 0x36, param(0), 0x3A, param(0x60) };
    int ret;
    dev_dbg(&spi.dev, "initializing LCD\n");
    ret = regulator_set_voltage(priv.io_reg, 1800000, 1800000);
    if (ret) {
    dev_err(&spi.dev, "failed to set the IO regulator voltage.\n");
    return;
    }
    ret = regulator_enable(priv.io_reg);
    if (ret) {
    dev_err(&spi.dev, "failed to enable the IO regulator.\n");
    return;
    }
    ret = regulator_set_voltage(priv.core_reg, 2800000, 2800000);
    if (ret) {
    dev_err(&spi.dev, "failed to set the core regulator voltage.\n");
    regulator_disable(priv.io_reg);
    return;
    }
    ret = regulator_enable(priv.core_reg);
    if (ret) {
    dev_err(&spi.dev, "failed to enable the core regulator.\n");
    regulator_disable(priv.io_reg);
    return;
    }
    l4f00242t03_reset(priv.reset);
    gpiod_set_value(priv.enable, 1);
    msleep(60);
    spi_write(spi, (const u8 *)cmd, ARRAY_SIZE(cmd) * sizeof(u16));
    }
#[no_mangle]
unsafe extern "C" fn l4f00242t03_lcd_powerdown(spi: *mut spi_device) {
    static void l4f00242t03_lcd_powerdown(struct spi_device *spi)
    {
    struct l4f00242t03_priv *priv = spi_get_drvdata(spi);
    dev_dbg(&spi.dev, "Powering down LCD\n");
    gpiod_set_value(priv.enable, 0);
    regulator_disable(priv.io_reg);
    regulator_disable(priv.core_reg);
    }
#[no_mangle]
unsafe extern "C" fn l4f00242t03_lcd_power_get(ld: *mut lcd_device) -> c_int {
    static int l4f00242t03_lcd_power_get(struct lcd_device *ld)
    {
    struct l4f00242t03_priv *priv = lcd_get_data(ld);
    return priv.lcd_state;
    }
#[no_mangle]
unsafe extern "C" fn l4f00242t03_lcd_power_set(ld: *mut lcd_device, power: c_int) -> c_int {
    static int l4f00242t03_lcd_power_set(struct lcd_device *ld, int power)
    {
    struct l4f00242t03_priv *priv = lcd_get_data(ld);
    struct spi_device *spi = priv.spi;
    let mut slpout: u16 = 0x11;
    let mut dison: u16 = 0x29;
    let mut slpin: u16 = 0x10;
    let mut disoff: u16 = 0x28;
    if (power <= LCD_POWER_REDUCED) {
    if (priv.lcd_state <= LCD_POWER_REDUCED) {
// Do nothing, the LCD is running
    } else if (priv.lcd_state < LCD_POWER_OFF) {
    dev_dbg(&spi.dev, "Resuming LCD\n");
    spi_write(spi, (const u8 *)&slpout, sizeof(u16));
    msleep(60);
    spi_write(spi, (const u8 *)&dison, sizeof(u16));
    } else {
// priv->lcd_state == LCD_POWER_OFF
    l4f00242t03_lcd_init(spi);
    priv.lcd_state = LCD_POWER_REDUCED_VSYNC_SUSPEND;
    l4f00242t03_lcd_power_set(priv.ld, power);
    }
    } else if (power < LCD_POWER_OFF) {
    if (priv.lcd_state <= LCD_POWER_REDUCED) {
// Send the display in standby
    dev_dbg(&spi.dev, "Standby the LCD\n");
    spi_write(spi, (const u8 *)&disoff, sizeof(u16));
    msleep(60);
    spi_write(spi, (const u8 *)&slpin, sizeof(u16));
    } else if (priv.lcd_state < LCD_POWER_OFF) {
// Do nothing, the LCD is already in standby
    } else {
// priv->lcd_state == LCD_POWER_OFF
    l4f00242t03_lcd_init(spi);
    priv.lcd_state = LCD_POWER_ON;
    l4f00242t03_lcd_power_set(ld, power);
    }
    } else {
// power == LCD_POWER_OFF
    if (priv.lcd_state != LCD_POWER_OFF) {
// Clear the screen before shutting down
    spi_write(spi, (const u8 *)&disoff, sizeof(u16));
    msleep(60);
    l4f00242t03_lcd_powerdown(spi);
    }
    }
    priv.lcd_state = power;
    return 0;
    }
    static const struct lcd_ops l4f_ops = {
    .set_power	= l4f00242t03_lcd_power_set,
    .get_power	= l4f00242t03_lcd_power_get,
    };
#[no_mangle]
unsafe extern "C" fn l4f00242t03_probe(spi: *mut spi_device) -> c_int {
    static int l4f00242t03_probe(struct spi_device *spi)
    {
    struct l4f00242t03_priv *priv;
    int ret;
    priv = devm_kzalloc(&spi.dev, sizeof(struct l4f00242t03_priv),
    GFP_KERNEL);
    if (priv == core::ptr::null_mut())
    return -ENOMEM;
    spi_set_drvdata(spi, priv);
    spi.bits_per_word = 9;
    ret = spi_setup(spi);
    if (ret < 0)
    return dev_err_probe(&spi.dev, ret, "Unable to setup spi.\n");
    priv.spi = spi;
    priv.reset = devm_gpiod_get(&spi.dev, "reset", GPIOD_OUT_HIGH);
    if (IS_ERR(priv.reset))
    return dev_err_probe(&spi.dev, PTR_ERR(priv.reset),
    "Unable to get the lcd l4f00242t03 reset gpio.\n");
    gpiod_set_consumer_name(priv.reset, "lcd l4f00242t03 reset");
    priv.enable = devm_gpiod_get(&spi.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(priv.enable))
    return dev_err_probe(&spi.dev, PTR_ERR(priv.enable),
    "Unable to get the lcd l4f00242t03 data en gpio.\n");
    gpiod_set_consumer_name(priv.enable, "lcd l4f00242t03 data enable");
    priv.io_reg = devm_regulator_get(&spi.dev, "vdd");
    if (IS_ERR(priv.io_reg))
    return dev_err_probe(&spi.dev, PTR_ERR(priv.io_reg),
    "%s: Unable to get the IO regulator\n",
    __func__);
    priv.core_reg = devm_regulator_get(&spi.dev, "vcore");
    if (IS_ERR(priv.core_reg))
    return dev_err_probe(&spi.dev, PTR_ERR(priv.core_reg),
    "%s: Unable to get the core regulator\n",
    __func__);
    priv.ld = devm_lcd_device_register(&spi.dev, "l4f00242t03", &spi.dev,
    priv, &l4f_ops);
    if (IS_ERR(priv.ld))
    return PTR_ERR(priv.ld);
// Init the LCD
    l4f00242t03_lcd_init(spi);
    priv.lcd_state = LCD_POWER_REDUCED_VSYNC_SUSPEND;
    l4f00242t03_lcd_power_set(priv.ld, LCD_POWER_ON);
    dev_info(&spi.dev, "Epson l4f00242t03 lcd probed.\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn l4f00242t03_remove(spi: *mut spi_device) {
    static void l4f00242t03_remove(struct spi_device *spi)
    {
    struct l4f00242t03_priv *priv = spi_get_drvdata(spi);
    l4f00242t03_lcd_power_set(priv.ld, LCD_POWER_OFF);
    }
#[no_mangle]
unsafe extern "C" fn l4f00242t03_shutdown(spi: *mut spi_device) {
    static void l4f00242t03_shutdown(struct spi_device *spi)
    {
    struct l4f00242t03_priv *priv = spi_get_drvdata(spi);
    if (priv)
    l4f00242t03_lcd_power_set(priv.ld, LCD_POWER_OFF);
    }
    static struct spi_driver l4f00242t03_driver = {
    .driver = {
    .name	= "l4f00242t03",
    },
    .probe		= l4f00242t03_probe,
    .remove		= l4f00242t03_remove,
    .shutdown	= l4f00242t03_shutdown,
    };
    module_spi_driver(l4f00242t03_driver);
    MODULE_AUTHOR("Alberto Panizzo <maramaopercheseimorto@gmail.com>");
    MODULE_DESCRIPTION("EPSON L4F00242T03 LCD");
    MODULE_LICENSE("GPL v2");
