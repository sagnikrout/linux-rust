//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-spear-spics.c
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
// SPEAr platform SPI chipselect abstraction over gpiolib
//
// Copyright (C) 2012 ST Microelectronics
// Shiraz Hashim <shiraz.linux.kernel@gmail.com>
//

// maximum chipselects
pub const NUM_OF_GPIO: c_int = 4;
//
// Provision is available on some SPEAr SoCs to control ARM PL022 spi cs
// through system registers. This register lies outside spi (pl022)
// address space into system registers.
//
// It provides control for spi chip select lines so that any chipselect
// (out of 4 possible chipselects in pl022) can be made low to select
// the particular slave.
//
// struct spear_spics - represents spi chip select control
// @base: base address
// @perip_cfg: configuration register
// @sw_enable_bit: bit to enable s/w control over chipselects
// @cs_value_bit: bit to program high or low chipselect
// @cs_enable_mask: mask to select bits required to select chipselect
// @cs_enable_shift: bit pos of cs_enable_mask
// @use_count: use count of a spi controller cs lines
// @last_off: stores last offset caller of set_value()
// @chip: gpio_chip abstraction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spear_spics {
    pub base: *mut void __iomem,
    pub perip_cfg: u32,
    pub sw_enable_bit: u32,
    pub cs_value_bit: u32,
    pub cs_enable_mask: u32,
    pub cs_enable_shift: u32,
    pub use_count: c_ulong,
    pub last_off: c_int,
    pub chip: gpio_chip,
}

    static int spics_set_value(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    struct spear_spics *spics = gpiochip_get_data(chip);
    u32 tmp;
// select chip select from register
    tmp = readl_relaxed(spics.base + spics.perip_cfg);
    if (spics.last_off != offset) {
    spics.last_off = offset;
    tmp &= ~(spics.cs_enable_mask << spics.cs_enable_shift);
    tmp |= offset << spics.cs_enable_shift;
    }
// toggle chip select line
    tmp &= ~(0x1 << spics.cs_value_bit);
    tmp |= value << spics.cs_value_bit;
    writel_relaxed(tmp, spics.base + spics.perip_cfg);
    return 0;
    }
    static int spics_direction_output(struct gpio_chip *chip, unsigned offset,
    int value)
    {
    return spics_set_value(chip, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn spics_request(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int spics_request(struct gpio_chip *chip, unsigned offset)
    {
    struct spear_spics *spics = gpiochip_get_data(chip);
    u32 tmp;
    if (!spics.use_count++) {
    tmp = readl_relaxed(spics.base + spics.perip_cfg);
    tmp |= 0x1 << spics.sw_enable_bit;
    tmp |= 0x1 << spics.cs_value_bit;
    writel_relaxed(tmp, spics.base + spics.perip_cfg);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spics_free(chip: *mut gpio_chip, offset: unsigned) {
    static void spics_free(struct gpio_chip *chip, unsigned offset)
    {
    struct spear_spics *spics = gpiochip_get_data(chip);
    u32 tmp;
    if (!--spics.use_count) {
    tmp = readl_relaxed(spics.base + spics.perip_cfg);
    tmp &= ~(0x1 << spics.sw_enable_bit);
    writel_relaxed(tmp, spics.base + spics.perip_cfg);
    }
    }
#[no_mangle]
unsafe extern "C" fn spics_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int spics_gpio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct spear_spics *spics;
    spics = devm_kzalloc(&pdev.dev, sizeof(*spics), GFP_KERNEL);
    if (!spics)
    return -ENOMEM;
    spics.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(spics.base))
    return PTR_ERR(spics.base);
    if (of_property_read_u32(np, "st-spics,peripcfg-reg",
    &spics.perip_cfg))
    goto err_dt_data;
    if (of_property_read_u32(np, "st-spics,sw-enable-bit",
    &spics.sw_enable_bit))
    goto err_dt_data;
    if (of_property_read_u32(np, "st-spics,cs-value-bit",
    &spics.cs_value_bit))
    goto err_dt_data;
    if (of_property_read_u32(np, "st-spics,cs-enable-mask",
    &spics.cs_enable_mask))
    goto err_dt_data;
    if (of_property_read_u32(np, "st-spics,cs-enable-shift",
    &spics.cs_enable_shift))
    goto err_dt_data;
    spics.chip.ngpio = NUM_OF_GPIO;
    spics.chip.base = -1;
    spics.chip.request = spics_request;
    spics.chip.free = spics_free;
    spics.chip.direction_output = spics_direction_output;
    spics.chip.set = spics_set_value;
    spics.chip.label = dev_name(&pdev.dev);
    spics.chip.parent = &pdev.dev;
    spics.chip.owner = THIS_MODULE;
    spics.last_off = -1;
    return devm_gpiochip_add_data(&pdev.dev, &spics.chip, spics);
    err_dt_data:
    dev_err(&pdev.dev, "DT probe failed\n");
    return -EINVAL;
    }
    static const struct of_device_id spics_gpio_of_match[] = {
    { .compatible = "st,spear-spics-gpio" },
    {}
    };
    static struct platform_driver spics_gpio_driver = {
    .probe = spics_gpio_probe,
    .driver = {
    .name = "spear-spics-gpio",
    .of_match_table = spics_gpio_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn spics_gpio_init() -> int __init {
    static int __init spics_gpio_init(void)
    {
    return platform_driver_register(&spics_gpio_driver);
    }
    subsys_initcall(spics_gpio_init);
