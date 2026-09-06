//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-creg-snps.c
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
// Synopsys CREG (Control REGisters) GPIO driver
//
// Copyright (C) 2018 Synopsys
// Author: Eugeniy Paltsev <Eugeniy.Paltsev@synopsys.com>

pub const MAX_GPIO: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct creg_layout {
    pub ngpio: u8,
    pub shift: [u8; MAX_GPIO],
    pub on: [u8; MAX_GPIO],
    pub off: [u8; MAX_GPIO],
    pub bit_per_gpio: [u8; MAX_GPIO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct creg_gpio {
    pub gc: gpio_chip,
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
    pub layout: *const creg_layout,
}

#[no_mangle]
unsafe extern "C" fn creg_gpio_set(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int creg_gpio_set(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct creg_gpio *hcg = gpiochip_get_data(gc);
    const struct creg_layout *layout = hcg.layout;
    u32 reg, reg_shift, value;
    unsigned long flags;
    int i;
    value = val ? hcg.layout.on[offset] : hcg.layout.off[offset];
    reg_shift = layout.shift[offset];
    for (i = 0; i < offset; i++)
    reg_shift += layout.bit_per_gpio[i] + layout.shift[i];
    spin_lock_irqsave(&hcg.lock, flags);
    reg = readl(hcg.regs);
    reg &= ~(GENMASK(layout.bit_per_gpio[i] - 1, 0) << reg_shift);
    reg |=  (value << reg_shift);
    writel(reg, hcg.regs);
    spin_unlock_irqrestore(&hcg.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn creg_gpio_dir_out(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int creg_gpio_dir_out(struct gpio_chip *gc, unsigned int offset, int val)
    {
    return creg_gpio_set(gc, offset, val);
    }
    static int creg_gpio_validate_pg(struct device *dev, struct creg_gpio *hcg,
    int i)
    {
    const struct creg_layout *layout = hcg.layout;
    if (layout.bit_per_gpio[i] < 1 || layout.bit_per_gpio[i] > 8)
    return -EINVAL;
// Check that on value fits its placeholder
    if (GENMASK(31, layout.bit_per_gpio[i]) & layout.on[i])
    return -EINVAL;
// Check that off value fits its placeholder
    if (GENMASK(31, layout.bit_per_gpio[i]) & layout.off[i])
    return -EINVAL;
    if (layout.on[i] == layout.off[i])
    return -EINVAL;
    return 0;
    }
    static int creg_gpio_validate(struct device *dev, struct creg_gpio *hcg,
    u32 ngpios)
    {
    let mut reg_len: u32 = 0;
    int i;
    if (hcg.layout.ngpio < 1 || hcg.layout.ngpio > MAX_GPIO)
    return -EINVAL;
    if (ngpios < 1 || ngpios > hcg.layout.ngpio) {
    dev_err(dev, "ngpios must be in [1:%u]\n", hcg.layout.ngpio);
    return -EINVAL;
    }
    for (i = 0; i < hcg.layout.ngpio; i++) {
    if (creg_gpio_validate_pg(dev, hcg, i))
    return -EINVAL;
    reg_len += hcg.layout.shift[i] + hcg.layout.bit_per_gpio[i];
    }
// Check that we fit in 32 bit register
    if (reg_len > 32)
    return -EINVAL;
    return 0;
    }
    static const struct creg_layout hsdk_cs_ctl = {
    .ngpio		= 10,
    .shift		= { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
    .off		= { 2, 2, 2, 2, 2, 2, 2, 2, 2, 2 },
    .on		= { 3, 3, 3, 3, 3, 3, 3, 3, 3, 3 },
    .bit_per_gpio	= { 2, 2, 2, 2, 2, 2, 2, 2, 2, 2 }
    };
    static const struct creg_layout axs10x_flsh_cs_ctl = {
    .ngpio		= 1,
    .shift		= { 0 },
    .off		= { 1 },
    .on		= { 3 },
    .bit_per_gpio	= { 2 }
    };
    static const struct of_device_id creg_gpio_ids[] = {
    {
    .compatible = "snps,creg-gpio-axs10x",
    .data = &axs10x_flsh_cs_ctl
    }, {
    .compatible = "snps,creg-gpio-hsdk",
    .data = &hsdk_cs_ctl
    }, { /* sentinel */ }
    };
#[no_mangle]
unsafe extern "C" fn creg_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int creg_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct creg_gpio *hcg;
    u32 ngpios;
    int ret;
    hcg = devm_kzalloc(dev, sizeof(struct creg_gpio), GFP_KERNEL);
    if (!hcg)
    return -ENOMEM;
    hcg.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hcg.regs))
    return PTR_ERR(hcg.regs);
    hcg.layout = device_get_match_data(dev);
    if (!hcg.layout)
    return -EINVAL;
    ret = of_property_read_u32(dev.of_node, "ngpios", &ngpios);
    if (ret)
    return ret;
    ret = creg_gpio_validate(dev, hcg, ngpios);
    if (ret)
    return ret;
    spin_lock_init(&hcg.lock);
    hcg.gc.parent = dev;
    hcg.gc.label = dev_name(dev);
    hcg.gc.base = -1;
    hcg.gc.ngpio = ngpios;
    hcg.gc.set = creg_gpio_set;
    hcg.gc.direction_output = creg_gpio_dir_out;
    ret = devm_gpiochip_add_data(dev, &hcg.gc, hcg);
    if (ret)
    return ret;
    dev_info(dev, "GPIO controller with %d gpios probed\n", ngpios);
    return 0;
    }
    static struct platform_driver creg_gpio_snps_driver = {
    .driver = {
    .name = "snps-creg-gpio",
    .of_match_table = creg_gpio_ids,
    },
    .probe  = creg_gpio_probe,
    };
    builtin_platform_driver(creg_gpio_snps_driver);
