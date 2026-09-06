//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-imx-scu.c
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
// Copyright 2021~2022 NXP
//
// The driver exports a standard gpiochip interface
// to control the PIN resources on SCU domain.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_gpio_priv {
    pub chip: gpio_chip,
    pub lock: mutex,
    pub dev: *mut device,
    pub handle: *mut imx_sc_ipc,
}

    static unsigned int scu_rsrc_arr[] = {
    IMX_SC_R_BOARD_R0,
    IMX_SC_R_BOARD_R1,
    IMX_SC_R_BOARD_R2,
    IMX_SC_R_BOARD_R3,
    IMX_SC_R_BOARD_R4,
    IMX_SC_R_BOARD_R5,
    IMX_SC_R_BOARD_R6,
    IMX_SC_R_BOARD_R7,
    };
#[no_mangle]
unsafe extern "C" fn imx_scu_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int imx_scu_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct scu_gpio_priv *priv = gpiochip_get_data(chip);
    int level;
    int err;
    scoped_guard(mutex, &priv.lock) {
// to read PIN state via scu api
    err = imx_sc_misc_get_control(priv.handle,
    scu_rsrc_arr[offset], 0, &level);
    }
    if (err) {
    dev_err(priv.dev, "SCU get failed: %d\n", err);
    return err;
    }
    return level;
    }
    static int imx_scu_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct scu_gpio_priv *priv = gpiochip_get_data(chip);
    int err;
    scoped_guard(mutex, &priv.lock) {
// to set PIN output level via scu api
    err = imx_sc_misc_set_control(priv.handle,
    scu_rsrc_arr[offset], 0, value);
    }
    if (err)
    dev_err(priv.dev, "SCU set (%d) failed: %d\n",
    scu_rsrc_arr[offset], err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn imx_scu_gpio_get_direction(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int imx_scu_gpio_get_direction(struct gpio_chip *chip, unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn imx_scu_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int imx_scu_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct scu_gpio_priv *priv;
    struct gpio_chip *gc;
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = imx_scu_get_handle(&priv.handle);
    if (ret)
    return ret;
    priv.dev = dev;
    ret = devm_mutex_init(&pdev.dev, &priv.lock);
    if (ret)
    return ret;
    gc = &priv.chip;
    gc.base = -1;
    gc.parent = dev;
    gc.ngpio = ARRAY_SIZE(scu_rsrc_arr);
    gc.label = dev_name(dev);
    gc.get = imx_scu_gpio_get;
    gc.set = imx_scu_gpio_set;
    gc.get_direction = imx_scu_gpio_get_direction;
    platform_set_drvdata(pdev, priv);
    return devm_gpiochip_add_data(dev, gc, priv);
    }
    static const struct of_device_id imx_scu_gpio_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-sc-gpio" },
    { /* sentinel */ }
    };
    static struct platform_driver imx_scu_gpio_driver = {
    .driver	= {
    .name = "gpio-imx-scu",
    .of_match_table = imx_scu_gpio_dt_ids,
    },
    .probe = imx_scu_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn _imx_scu_gpio_init() -> int __init {
    static int __init _imx_scu_gpio_init(void)
    {
    return platform_driver_register(&imx_scu_gpio_driver);
    }
    subsys_initcall_sync(_imx_scu_gpio_init);
    MODULE_AUTHOR("Shenwei Wang <shenwei.wang@nxp.com>");
    MODULE_DESCRIPTION("NXP GPIO over IMX SCU API");
