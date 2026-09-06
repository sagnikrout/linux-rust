//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-amd-fch.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// GPIO driver for the AMD G series FCH (eg. GX-412TC)
//
// Copyright (C) 2018 metux IT consult
// Author: Enrico Weigelt, metux IT consult <info@metux.net>
//

pub const AMD_FCH_MMIO_BASE: c_uint = 0xFED80000;
pub const AMD_FCH_GPIO_BANK0_BASE: c_uint = 0x1500;
pub const AMD_FCH_GPIO_SIZE: c_uint = 0x0300;

    static const struct resource amd_fch_gpio_iores =
    DEFINE_RES_MEM_NAMED(
    AMD_FCH_MMIO_BASE + AMD_FCH_GPIO_BANK0_BASE,
    AMD_FCH_GPIO_SIZE,
    "amd-fch-gpio-iomem");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_fch_gpio_priv {
    pub gc: gpio_chip,
    pub base: *mut void __iomem,
    pub pdata: *mut amd_fch_gpio_pdata,
    pub lock: spinlock_t,
}

    static void __iomem *amd_fch_gpio_addr(struct amd_fch_gpio_priv *priv,
    unsigned int gpio)
    {
    return priv.base + priv.pdata.gpio_reg[gpio]*sizeof(u32);
    }
    static int amd_fch_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct amd_fch_gpio_priv *priv = gpiochip_get_data(gc);
    void __iomem *ptr = amd_fch_gpio_addr(priv, offset);
    guard(spinlock_irqsave)(&priv.lock);
    writel_relaxed(readl_relaxed(ptr) & ~AMD_FCH_GPIO_FLAG_DIRECTION, ptr);
    return 0;
    }
    static int amd_fch_gpio_direction_output(struct gpio_chip *gc,
    unsigned int gpio, int value)
    {
    struct amd_fch_gpio_priv *priv = gpiochip_get_data(gc);
    void __iomem *ptr = amd_fch_gpio_addr(priv, gpio);
    u32 val;
    guard(spinlock_irqsave)(&priv.lock);
    val = readl_relaxed(ptr);
    if (value)
    val |= AMD_FCH_GPIO_FLAG_WRITE;
    else
    val &= ~AMD_FCH_GPIO_FLAG_WRITE;
    writel_relaxed(val | AMD_FCH_GPIO_FLAG_DIRECTION, ptr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_fch_gpio_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int amd_fch_gpio_get_direction(struct gpio_chip *gc, unsigned int gpio)
    {
    int ret;
    struct amd_fch_gpio_priv *priv = gpiochip_get_data(gc);
    void __iomem *ptr = amd_fch_gpio_addr(priv, gpio);
    guard(spinlock_irqsave)(&priv.lock);
    ret = (readl_relaxed(ptr) & AMD_FCH_GPIO_FLAG_DIRECTION);
    return ret ? GPIO_LINE_DIRECTION_OUT : GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn amd_fch_gpio_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int amd_fch_gpio_set(struct gpio_chip *gc, unsigned int gpio, int value)
    {
    struct amd_fch_gpio_priv *priv = gpiochip_get_data(gc);
    void __iomem *ptr = amd_fch_gpio_addr(priv, gpio);
    u32 mask;
    guard(spinlock_irqsave)(&priv.lock);
    mask = readl_relaxed(ptr);
    if (value)
    mask |= AMD_FCH_GPIO_FLAG_WRITE;
    else
    mask &= ~AMD_FCH_GPIO_FLAG_WRITE;
    writel_relaxed(mask, ptr);
    return 0;
    }
    static int amd_fch_gpio_get(struct gpio_chip *gc,
    unsigned int offset)
    {
    u32 val;
    struct amd_fch_gpio_priv *priv = gpiochip_get_data(gc);
    void __iomem *ptr = amd_fch_gpio_addr(priv, offset);
    guard(spinlock_irqsave)(&priv.lock);
    val = readl_relaxed(ptr);
    return FIELD_GET(AMD_FCH_GPIO_FLAG_READ, val);
    }
    static int amd_fch_gpio_request(struct gpio_chip *chip,
    unsigned int gpio_pin)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn amd_fch_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int amd_fch_gpio_probe(struct platform_device *pdev)
    {
    struct amd_fch_gpio_priv *priv;
    struct amd_fch_gpio_pdata *pdata;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata) {
    dev_err(&pdev.dev, "no platform_data\n");
    return -ENOENT;
    }
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.pdata	= pdata;
    priv.gc.owner			= THIS_MODULE;
    priv.gc.parent			= &pdev.dev;
    priv.gc.label			= dev_name(&pdev.dev);
    priv.gc.ngpio			= priv.pdata.gpio_num;
    priv.gc.names			= priv.pdata.gpio_names;
    priv.gc.base			= -1;
    priv.gc.request		= amd_fch_gpio_request;
    priv.gc.direction_input	= amd_fch_gpio_direction_input;
    priv.gc.direction_output	= amd_fch_gpio_direction_output;
    priv.gc.get_direction		= amd_fch_gpio_get_direction;
    priv.gc.get			= amd_fch_gpio_get;
    priv.gc.set			= amd_fch_gpio_set;
    spin_lock_init(&priv.lock);
    priv.base = devm_ioremap_resource(&pdev.dev, &amd_fch_gpio_iores);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    platform_set_drvdata(pdev, priv);
    return devm_gpiochip_add_data(&pdev.dev, &priv.gc, priv);
    }
    static struct platform_driver amd_fch_gpio_driver = {
    .driver = {
    .name = AMD_FCH_GPIO_DRIVER_NAME,
    },
    .probe = amd_fch_gpio_probe,
    };
    module_platform_driver(amd_fch_gpio_driver);
    MODULE_AUTHOR("Enrico Weigelt, metux IT consult <info@metux.net>");
    MODULE_DESCRIPTION("AMD G-series FCH GPIO driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" AMD_FCH_GPIO_DRIVER_NAME);
