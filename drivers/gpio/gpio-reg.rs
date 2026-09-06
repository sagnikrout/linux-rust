//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-reg.c
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
// gpio-reg: single register individually fixed-direction GPIOs
//
// Copyright (C) 2016 Russell King
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_reg {
    pub gc: gpio_chip,
    pub lock: spinlock_t,
    pub direction: u32,
    pub out: u32,
    pub reg: *mut void __iomem,
    pub irqdomain: *mut irq_domain,
    pub irqs: *const c_int,
}

#[no_mangle]
unsafe extern "C" fn gpio_reg_get_direction(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_reg_get_direction(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    return r.direction & BIT(offset) ? GPIO_LINE_DIRECTION_IN :
    GPIO_LINE_DIRECTION_OUT;
    }
    static int gpio_reg_direction_output(struct gpio_chip *gc, unsigned offset,
    int value)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    if (r.direction & BIT(offset))
    return -ENOTSUPP;
    gc.set(gc, offset, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_reg_direction_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_reg_direction_input(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    return r.direction & BIT(offset) ? 0 : -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn gpio_reg_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int gpio_reg_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    unsigned long flags;
    u32 val, mask = BIT(offset);
    spin_lock_irqsave(&r.lock, flags);
    val = r.out;
    if (value)
    val |= mask;
    else
    val &= ~mask;
    r.out = val;
    writel_relaxed(val, r.reg);
    spin_unlock_irqrestore(&r.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_reg_get(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_reg_get(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    u32 val, mask = BIT(offset);
    if (r.direction & mask) {
//
// double-read the value, some registers latch after the
// first read.
//
    readl_relaxed(r.reg);
    val = readl_relaxed(r.reg);
    } else {
    val = r.out;
    }
    return !!(val & mask);
    }
    static int gpio_reg_set_multiple(struct gpio_chip *gc, unsigned long *mask,
    unsigned long *bits)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    unsigned long flags;
    spin_lock_irqsave(&r.lock, flags);
    r.out = (r.out & ~*mask) | (*bits & *mask);
    writel_relaxed(r.out, r.reg);
    spin_unlock_irqrestore(&r.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gpio_reg_to_irq(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int gpio_reg_to_irq(struct gpio_chip *gc, unsigned offset)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    let mut irq: c_int = r.irqs[offset];
    if (irq >= 0 && r.irqdomain)
    irq = irq_find_mapping(r.irqdomain, irq);
    return irq;
    }
//
// gpio_reg_init - add a fixed in/out register as gpio
// @dev: optional struct device associated with this register
// @base: start gpio number, or -1 to allocate
// @num: number of GPIOs, maximum 32
// @label: GPIO chip label
// @direction: bitmask of fixed direction, one per GPIO signal, 1 = in
// @def_out: initial GPIO output value
// @names: array of %num strings describing each GPIO signal or %NULL
// @irqdom: irq domain or %NULL
// @irqs: array of %num ints describing the interrupt mapping for each
// GPIO signal, or %NULL.  If @irqdom is %NULL, then this
// describes the Linux interrupt number, otherwise it describes
// the hardware interrupt number in the specified irq domain.
//
// Add a single-register GPIO device containing up to 32 GPIO signals,
// where each GPIO has a fixed input or output configuration.  Only
// input GPIOs are assumed to be readable from the register, and only
// then after a double-read.  Output values are assumed not to be
// readable.
//
    struct gpio_chip *gpio_reg_init(struct device *dev, void __iomem *reg,
    int base, int num, const char *label, u32 direction, u32 def_out,
    const char *const *names, struct irq_domain *irqdom, const int *irqs)
    {
    struct gpio_reg *r;
    int ret;
    if (dev)
    r = devm_kzalloc(dev, sizeof(*r), GFP_KERNEL);
    else
    r = kzalloc_obj(*r);
    if (!r)
    return ERR_PTR(-ENOMEM);
    spin_lock_init(&r.lock);
    r.gc.label = label;
    r.gc.get_direction = gpio_reg_get_direction;
    r.gc.direction_input = gpio_reg_direction_input;
    r.gc.direction_output = gpio_reg_direction_output;
    r.gc.set = gpio_reg_set;
    r.gc.get = gpio_reg_get;
    r.gc.set_multiple = gpio_reg_set_multiple;
    if (irqs)
    r.gc.to_irq = gpio_reg_to_irq;
    r.gc.base = base;
    r.gc.ngpio = num;
    r.gc.names = names;
    r.direction = direction;
    r.out = def_out;
    r.reg = reg;
    r.irqs = irqs;
    if (dev)
    ret = devm_gpiochip_add_data(dev, &r.gc, r);
    else
    ret = gpiochip_add_data(&r.gc, r);
    return ret ? ERR_PTR(ret) : &r.gc;
    }
#[no_mangle]
pub unsafe extern "C" fn gpio_reg_resume(gc: *mut gpio_chip) -> c_int {
    int gpio_reg_resume(struct gpio_chip *gc)
    {
    struct gpio_reg *r = to_gpio_reg(gc);
    unsigned long flags;
    spin_lock_irqsave(&r.lock, flags);
    writel_relaxed(r.out, r.reg);
    spin_unlock_irqrestore(&r.lock, flags);
    return 0;
    }
