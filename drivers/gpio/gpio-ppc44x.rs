//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ppc44x.c
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
// PPC44x gpio driver
//
// Copyright (c) 2008 Harris Corporation
// Copyright (c) 2008 Sascha Hauer <s.hauer@pengutronix.de>, Pengutronix
// Copyright (c) MontaVista Software, Inc. 2008.
//
// Author: Steve Falco <sfalco@harris.com>
//

// Physical GPIO register layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc44x_gpio {
    pub or: __be32,
    pub tcr: __be32,
    pub osrl: __be32,
    pub osrh: __be32,
    pub tsrl: __be32,
    pub tsrh: __be32,
    pub odr: __be32,
    pub ir: __be32,
    pub rr1: __be32,
    pub rr2: __be32,
    pub rr3: __be32,
    pub reserved1: __be32,
    pub isr1l: __be32,
    pub isr1h: __be32,
    pub isr2l: __be32,
    pub isr2h: __be32,
    pub isr3l: __be32,
    pub isr3h: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc44x_gpio_chip {
    pub chip: gpio_generic_chip,
    pub regs: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn ppc44x_clrbits32(addr: *mut void __iomem, mask: u32) {
    static inline void ppc44x_clrbits32(void __iomem *addr, u32 mask)
    {
    let mut val: u32 = ioread32be(addr);
    val &= ~mask;
    iowrite32be(val, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn ppc44x_setbits32(addr: *mut void __iomem, mask: u32) {
    static inline void ppc44x_setbits32(void __iomem *addr, u32 mask)
    {
    let mut val: u32 = ioread32be(addr);
    val |= mask;
    iowrite32be(val, addr);
    }
//
// GPIO LIB API implementation for GPIOs
//
// There are a maximum of 32 gpios in each gpio controller.
//
    static inline void
    __ppc44x_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct ppc44x_gpio_chip *chip = gpiochip_get_data(gc);
    struct gpio_generic_chip *gen_gc = &chip.chip;
    if (val)
    gen_gc.sdata |= GPIO_MASK(gpio);
    else
    gen_gc.sdata &= ~GPIO_MASK(gpio);
    gpio_generic_write_reg(gen_gc, gen_gc.reg_set, gen_gc.sdata);
    }
#[no_mangle]
unsafe extern "C" fn ppc44x_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int ppc44x_gpio_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct ppc44x_gpio_chip *chip = gpiochip_get_data(gc);
    struct gpio_generic_chip *gen_gc = &chip.chip;
    struct ppc44x_gpio __iomem *regs = chip.regs;
    guard(gpio_generic_lock_irqsave)(gen_gc);
// Disable open-drain function
    ppc44x_clrbits32(&regs.odr, GPIO_MASK(gpio));
// Float the pin
    ppc44x_clrbits32(&regs.tcr, GPIO_MASK(gpio));
    gen_gc.sdir &= ~GPIO_MASK(gpio);
// Bits 0-15 use TSRL/OSRL, bits 16-31 use TSRH/OSRH
    if (gpio < 16) {
    ppc44x_clrbits32(&regs.osrl, GPIO_MASK2(gpio));
    ppc44x_clrbits32(&regs.tsrl, GPIO_MASK2(gpio));
    } else {
    ppc44x_clrbits32(&regs.osrh, GPIO_MASK2(gpio));
    ppc44x_clrbits32(&regs.tsrh, GPIO_MASK2(gpio));
    }
    return 0;
    }
    static int
    ppc44x_gpio_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct ppc44x_gpio_chip *chip = gpiochip_get_data(gc);
    struct gpio_generic_chip *gen_gc = &chip.chip;
    struct ppc44x_gpio __iomem *regs = chip.regs;
    guard(gpio_generic_lock_irqsave)(gen_gc);
// First set initial value
    __ppc44x_gpio_set(gc, gpio, val);
// Disable open-drain function
    ppc44x_clrbits32(&regs.odr, GPIO_MASK(gpio));
// Drive the pin
    ppc44x_setbits32(&regs.tcr, GPIO_MASK(gpio));
    gen_gc.sdir |= GPIO_MASK(gpio);
// Bits 0-15 use TSRL, bits 16-31 use TSRH
    if (gpio < 16) {
    ppc44x_clrbits32(&regs.osrl, GPIO_MASK2(gpio));
    ppc44x_clrbits32(&regs.tsrl, GPIO_MASK2(gpio));
    } else {
    ppc44x_clrbits32(&regs.osrh, GPIO_MASK2(gpio));
    ppc44x_clrbits32(&regs.tsrh, GPIO_MASK2(gpio));
    }
    pr_debug("%s: gpio: %d val: %d\n", __func__, gpio, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ppc44x_gpio_probe(ofdev: *mut platform_device) -> c_int {
    static int ppc44x_gpio_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct ppc44x_gpio __iomem *regs;
    struct ppc44x_gpio_chip *chip;
    struct gpio_generic_chip_config config;
    struct gpio_chip *gc;
    int ret;
    regs = devm_platform_ioremap_resource(ofdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    chip = devm_kzalloc(dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.regs = regs;
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = &regs.ir,
    .set = &regs.or,
    .dirout = &regs.tcr,
    .flags = GPIO_GENERIC_BIG_ENDIAN |
    GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };
    ret = gpio_generic_chip_init(&chip.chip, &config);
    if (ret)
    return ret;
    gc = &chip.chip.gc;
    gc.label = dev_name(dev);
    gc.parent = dev;
    gc.direction_input = ppc44x_gpio_dir_in;
    gc.direction_output = ppc44x_gpio_dir_out;
    return devm_gpiochip_add_data(dev, gc, chip);
    }
    static const struct of_device_id ppc44x_gpio_match[] = {
    {
    .compatible = "ibm,ppc4xx-gpio",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, ppc44x_gpio_match);
    static struct platform_driver ppc44x_gpio_driver = {
    .probe		= ppc44x_gpio_probe,
    .driver		= {
    .name	= "ppc44x-gpio",
    .of_match_table	= ppc44x_gpio_match,
    },
    };
    MODULE_DESCRIPTION("PPC44x gpio driver");
    MODULE_LICENSE("GPL");
    module_platform_driver(ppc44x_gpio_driver);
