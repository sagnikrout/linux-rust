//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-timberdale.c
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
// Timberdale FPGA GPIO driver
// Author: Mocean Laboratories
// Copyright (c) 2009 Intel Corporation
//
// Supports:
// Timberdale FPGA GPIO
//

pub const TGPIOVAL: c_uint = 0x00;
pub const TGPIODIR: c_uint = 0x04;
pub const TGPIO_IER: c_uint = 0x08;
pub const TGPIO_ISR: c_uint = 0x0c;
pub const TGPIO_IPR: c_uint = 0x10;
pub const TGPIO_ICR: c_uint = 0x14;
pub const TGPIO_FLR: c_uint = 0x18;
pub const TGPIO_LVR: c_uint = 0x1c;
pub const TGPIO_VER: c_uint = 0x20;
pub const TGPIO_BFLR: c_uint = 0x24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timbgpio {
    pub membase: *mut void __iomem,
    pub /: *mut *mut spinlock_t lock; / mutual exclusion,
    pub gpio: gpio_chip,
    pub irq_base: c_int,
    pub last_ier: c_ulong,
}

    static int timbgpio_update_bit(struct gpio_chip *gpio, unsigned index,
    unsigned offset, bool enabled)
    {
    struct timbgpio *tgpio = gpiochip_get_data(gpio);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&tgpio.lock, flags);
    reg = ioread32(tgpio.membase + offset);
    if (enabled)
    reg |= (1 << index);
    else
    reg &= ~(1 << index);
    iowrite32(reg, tgpio.membase + offset);
    spin_unlock_irqrestore(&tgpio.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_gpio_direction_input(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int timbgpio_gpio_direction_input(struct gpio_chip *gpio, unsigned nr)
    {
    return timbgpio_update_bit(gpio, nr, TGPIODIR, true);
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_gpio_get(gpio: *mut gpio_chip, nr: unsigned) -> c_int {
    static int timbgpio_gpio_get(struct gpio_chip *gpio, unsigned nr)
    {
    struct timbgpio *tgpio = gpiochip_get_data(gpio);
    u32 value;
    value = ioread32(tgpio.membase + TGPIOVAL);
    return (value & (1 << nr)) ? 1 : 0;
    }
    static int timbgpio_gpio_direction_output(struct gpio_chip *gpio,
    unsigned nr, int val)
    {
    return timbgpio_update_bit(gpio, nr, TGPIODIR, false);
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_gpio_set(gpio: *mut gpio_chip, nr: c_uint, val: c_int) -> c_int {
    static int timbgpio_gpio_set(struct gpio_chip *gpio, unsigned int nr, int val)
    {
    return timbgpio_update_bit(gpio, nr, TGPIOVAL, val != 0);
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_to_irq(gpio: *mut gpio_chip, offset: unsigned) -> c_int {
    static int timbgpio_to_irq(struct gpio_chip *gpio, unsigned offset)
    {
    struct timbgpio *tgpio = gpiochip_get_data(gpio);
    if (tgpio.irq_base <= 0)
    return -EINVAL;
    return tgpio.irq_base + offset;
    }
//
// GPIO IRQ
//
#[no_mangle]
unsafe extern "C" fn timbgpio_irq_disable(d: *mut irq_data) {
    static void timbgpio_irq_disable(struct irq_data *d)
    {
    struct timbgpio *tgpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = d.irq - tgpio.irq_base;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    spin_lock_irqsave(&tgpio.lock, flags);
    tgpio.last_ier &= ~(1UL << offset);
    iowrite32(tgpio.last_ier, tgpio.membase + TGPIO_IER);
    spin_unlock_irqrestore(&tgpio.lock, flags);
    gpiochip_disable_irq(&tgpio.gpio, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_irq_enable(d: *mut irq_data) {
    static void timbgpio_irq_enable(struct irq_data *d)
    {
    struct timbgpio *tgpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = d.irq - tgpio.irq_base;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    gpiochip_enable_irq(&tgpio.gpio, hwirq);
    spin_lock_irqsave(&tgpio.lock, flags);
    tgpio.last_ier |= 1UL << offset;
    iowrite32(tgpio.last_ier, tgpio.membase + TGPIO_IER);
    spin_unlock_irqrestore(&tgpio.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_irq_type(d: *mut irq_data, trigger: unsigned) -> c_int {
    static int timbgpio_irq_type(struct irq_data *d, unsigned trigger)
    {
    struct timbgpio *tgpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = d.irq - tgpio.irq_base;
    unsigned long flags;
    u32 lvr, flr, bflr = 0;
    u32 ver;
    let mut ret: c_int = 0;
    if (offset < 0 || offset >= tgpio.gpio.ngpio)
    return -EINVAL;
    ver = ioread32(tgpio.membase + TGPIO_VER);
    spin_lock_irqsave(&tgpio.lock, flags);
    lvr = ioread32(tgpio.membase + TGPIO_LVR);
    flr = ioread32(tgpio.membase + TGPIO_FLR);
    if (ver > 2)
    bflr = ioread32(tgpio.membase + TGPIO_BFLR);
    if (trigger & IRQ_TYPE_LEVEL_MASK) {
    bflr &= ~(1 << offset);
    flr &= ~(1 << offset);
    if (trigger & IRQ_TYPE_LEVEL_HIGH)
    lvr |= 1 << offset;
    else
    lvr &= ~(1 << offset);
    }
    if ((trigger & IRQ_TYPE_EDGE_BOTH) == IRQ_TYPE_EDGE_BOTH) {
    if (ver < 3) {
    ret = -EINVAL;
    goto out;
    } else {
    flr |= 1 << offset;
    bflr |= 1 << offset;
    }
    } else {
    bflr &= ~(1 << offset);
    flr |= 1 << offset;
    if (trigger & IRQ_TYPE_EDGE_FALLING)
    lvr &= ~(1 << offset);
    else
    lvr |= 1 << offset;
    }
    iowrite32(lvr, tgpio.membase + TGPIO_LVR);
    iowrite32(flr, tgpio.membase + TGPIO_FLR);
    if (ver > 2)
    iowrite32(bflr, tgpio.membase + TGPIO_BFLR);
    iowrite32(1 << offset, tgpio.membase + TGPIO_ICR);
    out:
    spin_unlock_irqrestore(&tgpio.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn timbgpio_irq(desc: *mut irq_desc) {
    static void timbgpio_irq(struct irq_desc *desc)
    {
    struct timbgpio *tgpio = irq_desc_get_handler_data(desc);
    struct irq_data *data = irq_desc_get_irq_data(desc);
    unsigned long ipr;
    int offset;
    data.chip.irq_ack(data);
    ipr = ioread32(tgpio.membase + TGPIO_IPR);
    iowrite32(ipr, tgpio.membase + TGPIO_ICR);
//
// Some versions of the hardware trash the IER register if more than
// one interrupt is received simultaneously.
//
    iowrite32(0, tgpio.membase + TGPIO_IER);
    for_each_set_bit(offset, &ipr, tgpio.gpio.ngpio)
    generic_handle_irq(timbgpio_to_irq(&tgpio.gpio, offset));
    iowrite32(tgpio.last_ier, tgpio.membase + TGPIO_IER);
    }
    static const struct irq_chip timbgpio_irqchip = {
    .name		= "GPIO",
    .irq_enable	= timbgpio_irq_enable,
    .irq_disable	= timbgpio_irq_disable,
    .irq_set_type	= timbgpio_irq_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn timbgpio_probe(pdev: *mut platform_device) -> c_int {
    static int timbgpio_probe(struct platform_device *pdev)
    {
    int err, i;
    struct device *dev = &pdev.dev;
    struct gpio_chip *gc;
    struct timbgpio *tgpio;
    let mut irq: c_int = platform_get_irq(pdev, 0);
    tgpio = devm_kzalloc(dev, sizeof(*tgpio), GFP_KERNEL);
    if (!tgpio)
    return -ENOMEM;
    gc = &tgpio.gpio;
    err = device_property_read_u32(dev, "irq-base", &tgpio.irq_base);
    if (err)
    return err;
    err = device_property_read_u32(dev, "gpio-base", &gc.base);
    if (err)
    return err;
    spin_lock_init(&tgpio.lock);
    tgpio.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(tgpio.membase))
    return PTR_ERR(tgpio.membase);
    gc.label = dev_name(&pdev.dev);
    gc.owner = THIS_MODULE;
    gc.parent = &pdev.dev;
    gc.direction_input = timbgpio_gpio_direction_input;
    gc.get = timbgpio_gpio_get;
    gc.direction_output = timbgpio_gpio_direction_output;
    gc.set = timbgpio_gpio_set;
    gc.to_irq = (irq >= 0 && tgpio.irq_base > 0) ? timbgpio_to_irq : core::ptr::null_mut();
    gc.dbg_show = core::ptr::null_mut();
    gc.can_sleep = false;
    err = devm_gpiochip_add_data(&pdev.dev, gc, tgpio);
    if (err)
    return err;
    if (gc.ngpio > 32)
    return dev_err_probe(dev, -EINVAL, "Invalid number of pins\n");
// make sure to disable interrupts
    iowrite32(0x0, tgpio.membase + TGPIO_IER);
    if (irq < 0 || tgpio.irq_base <= 0)
    return 0;
    for (i = 0; i < gc.ngpio; i++) {
    irq_set_chip_and_handler(tgpio.irq_base + i,
    &timbgpio_irqchip, handle_simple_irq);
    irq_set_chip_data(tgpio.irq_base + i, tgpio);
    irq_clear_status_flags(tgpio.irq_base + i, IRQ_NOREQUEST | IRQ_NOPROBE);
    }
    irq_set_chained_handler_and_data(irq, timbgpio_irq, tgpio);
    return 0;
    }
    static struct platform_driver timbgpio_platform_driver = {
    .driver = {
    .name			= DRIVER_NAME,
    .suppress_bind_attrs	= true,
    },
    .probe		= timbgpio_probe,
    };
// --------------------------------------------------------------------------
    builtin_platform_driver(timbgpio_platform_driver);
