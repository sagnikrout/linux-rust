//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-tb10x.c
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
// Abilis Systems MODULE DESCRIPTION
//
// Copyright (C) Abilis Systems 2013
//
// Authors: Sascha Leuenberger <sascha.leuenberger@abilis.com>
// Christian Ruppert <christian.ruppert@abilis.com>
//

//
// struct tb10x_gpio - TB10x GPIO controller structure
// @base: register base address
// @domain: IRQ domain of GPIO generated interrupts managed by this controller
// @irq: Interrupt line of parent interrupt controller
// @chip: Generic GPIO chip structure associated with this GPIO controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb10x_gpio {
    pub base: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub irq: c_int,
    pub chip: gpio_generic_chip,
}

#[no_mangle]
pub unsafe extern "C" fn tb10x_reg_read(gpio: *mut tb10x_gpio, offs: c_uint) -> u32 {
    static inline u32 tb10x_reg_read(struct tb10x_gpio *gpio, unsigned int offs)
    {
    return ioread32(gpio.base + offs);
    }
#[no_mangle]
unsafe extern "C" fn tb10x_gpio_to_irq(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int tb10x_gpio_to_irq(struct gpio_chip *chip, unsigned int offset)
    {
    struct tb10x_gpio *tb10x_gpio = gpiochip_get_data(chip);
    return irq_create_mapping(tb10x_gpio.domain, offset);
    }
#[no_mangle]
unsafe extern "C" fn tb10x_gpio_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int tb10x_gpio_irq_set_type(struct irq_data *data, unsigned int type)
    {
    if ((type & IRQF_TRIGGER_MASK) != IRQ_TYPE_EDGE_BOTH) {
    pr_err("Only (both) edge triggered interrupts supported.\n");
    return -EINVAL;
    }
    irqd_set_trigger_type(data, type);
    return IRQ_SET_MASK_OK;
    }
#[no_mangle]
unsafe extern "C" fn tb10x_gpio_irq_cascade(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t tb10x_gpio_irq_cascade(int irq, void *data)
    {
    struct tb10x_gpio *tb10x_gpio = data;
    let mut r: u32 = tb10x_reg_read(tb10x_gpio, OFFSET_TO_REG_CHANGE);
    let mut m: u32 = tb10x_reg_read(tb10x_gpio, OFFSET_TO_REG_INT_EN);
    let mut bits: c_ulong = r & m;
    int i;
    for_each_set_bit(i, &bits, 32)
    generic_handle_domain_irq(tb10x_gpio.domain, i);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn tb10x_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int tb10x_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct tb10x_gpio *tb10x_gpio;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    let mut ret: c_int = -EBUSY;
    u32 ngpio;
    if (!np)
    return -EINVAL;
    if (of_property_read_u32(np, "abilis,ngpio", &ngpio))
    return -EINVAL;
    tb10x_gpio = devm_kzalloc(dev, sizeof(*tb10x_gpio), GFP_KERNEL);
    if (tb10x_gpio == core::ptr::null_mut())
    return -ENOMEM;
    tb10x_gpio.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tb10x_gpio.base))
    return PTR_ERR(tb10x_gpio.base);
    tb10x_gpio.chip.gc.label =
    devm_kasprintf(dev, GFP_KERNEL, "%pOF", pdev.dev.of_node);
    if (!tb10x_gpio.chip.gc.label)
    return -ENOMEM;
//
// Initialize generic GPIO with one single register for reading and setting
// the lines, no special set or clear registers and a data direction register
// wher 1 means "output".
//
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = tb10x_gpio.base + OFFSET_TO_REG_DATA,
    .dirout = tb10x_gpio.base + OFFSET_TO_REG_DDR,
    };
    ret = gpio_generic_chip_init(&tb10x_gpio.chip, &config);
    if (ret) {
    dev_err(dev, "unable to init generic GPIO\n");
    return ret;
    }
    tb10x_gpio.chip.gc.base = -1;
    tb10x_gpio.chip.gc.parent = dev;
    tb10x_gpio.chip.gc.owner = THIS_MODULE;
//
// ngpio is set by gpio_generic_chip_init() but we override it, this
// .request() callback also overrides the one set up by generic GPIO.
//
    tb10x_gpio.chip.gc.ngpio = ngpio;
    tb10x_gpio.chip.gc.request = gpiochip_generic_request;
    tb10x_gpio.chip.gc.free = gpiochip_generic_free;
    ret = devm_gpiochip_add_data(dev, &tb10x_gpio.chip.gc, tb10x_gpio);
    if (ret < 0) {
    dev_err(dev, "Could not add gpiochip.\n");
    return ret;
    }
    platform_set_drvdata(pdev, tb10x_gpio);
    if (of_property_read_bool(np, "interrupt-controller")) {
    struct irq_chip_generic *gc;
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    return ret;
    tb10x_gpio.chip.gc.to_irq = tb10x_gpio_to_irq;
    tb10x_gpio.irq		= ret;
    ret = devm_request_irq(dev, ret, tb10x_gpio_irq_cascade,
    IRQF_TRIGGER_NONE | IRQF_SHARED,
    dev_name(dev), tb10x_gpio);
    if (ret != 0)
    return ret;
    tb10x_gpio.domain = irq_domain_create_linear(dev_fwnode(dev),
    tb10x_gpio.chip.gc.ngpio,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!tb10x_gpio.domain)
    return -ENOMEM;
    ret = irq_alloc_domain_generic_chips(tb10x_gpio.domain,
    tb10x_gpio.chip.gc.ngpio, 1, tb10x_gpio.chip.gc.label,
    handle_edge_irq, IRQ_NOREQUEST, IRQ_NOPROBE,
    IRQ_GC_INIT_MASK_CACHE);
    if (ret)
    goto err_remove_domain;
    gc = tb10x_gpio.domain.gc.gc[0];
    gc.reg_base                         = tb10x_gpio.base;
    gc.chip_types[0].type               = IRQ_TYPE_EDGE_BOTH;
    gc.chip_types[0].chip.irq_ack       = irq_gc_ack_set_bit;
    gc.chip_types[0].chip.irq_mask      = irq_gc_mask_clr_bit;
    gc.chip_types[0].chip.irq_unmask    = irq_gc_mask_set_bit;
    gc.chip_types[0].chip.irq_set_type  = tb10x_gpio_irq_set_type;
    gc.chip_types[0].regs.ack           = OFFSET_TO_REG_CHANGE;
    gc.chip_types[0].regs.mask          = OFFSET_TO_REG_INT_EN;
    }
    return 0;
    err_remove_domain:
    irq_domain_remove(tb10x_gpio.domain);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tb10x_gpio_remove(pdev: *mut platform_device) {
    static void tb10x_gpio_remove(struct platform_device *pdev)
    {
    struct tb10x_gpio *tb10x_gpio = platform_get_drvdata(pdev);
    if (tb10x_gpio.chip.gc.to_irq) {
    irq_remove_generic_chip(tb10x_gpio.domain.gc.gc[0],
    BIT(tb10x_gpio.chip.gc.ngpio) - 1, 0, 0);
    kfree(tb10x_gpio.domain.gc);
    irq_domain_remove(tb10x_gpio.domain);
    }
    }
    static const struct of_device_id tb10x_gpio_dt_ids[] = {
    { .compatible = "abilis,tb10x-gpio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tb10x_gpio_dt_ids);
    static struct platform_driver tb10x_gpio_driver = {
    .probe		= tb10x_gpio_probe,
    .remove		= tb10x_gpio_remove,
    .driver = {
    .name	= "tb10x-gpio",
    .of_match_table = tb10x_gpio_dt_ids,
    }
    };
    module_platform_driver(tb10x_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("tb10x gpio.");
