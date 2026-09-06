//! Automatically rewritten from C to Rust
//! Source: drivers/soc/fsl/qe/gpio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// QUICC Engine GPIOs
//
// Copyright (c) MontaVista Software, Inc. 2008.
//
// Author: Anton Vorontsov <avorontsov@ru.mvista.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_gpio_chip {
    pub np: *mut device_node,
    pub gc: gpio_chip,
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
// shadowed data register to clear/set bits safely
    pub cpdata: u32,
// saved_regs used to restore dedicated functions
    pub saved_regs: qe_pio_regs,
}

#[no_mangle]
unsafe extern "C" fn qe_gpio_save_regs(qe_gc: *mut qe_gpio_chip) {
    static void qe_gpio_save_regs(struct qe_gpio_chip *qe_gc)
    {
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    qe_gc.cpdata = ioread32be(&regs.cpdata);
    qe_gc.saved_regs.cpdata = qe_gc.cpdata;
    qe_gc.saved_regs.cpdir1 = ioread32be(&regs.cpdir1);
    qe_gc.saved_regs.cpdir2 = ioread32be(&regs.cpdir2);
    qe_gc.saved_regs.cppar1 = ioread32be(&regs.cppar1);
    qe_gc.saved_regs.cppar2 = ioread32be(&regs.cppar2);
    qe_gc.saved_regs.cpodr = ioread32be(&regs.cpodr);
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int qe_gpio_get(struct gpio_chip *gc, unsigned int gpio)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    let mut pin_mask: u32 = PIN_MASK(gpio);
    return !!(ioread32be(&regs.cpdata) & pin_mask);
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_set(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int qe_gpio_set(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    unsigned long flags;
    let mut pin_mask: u32 = PIN_MASK(gpio);
    spin_lock_irqsave(&qe_gc.lock, flags);
    if (val)
    qe_gc.cpdata |= pin_mask;
    else
    qe_gc.cpdata &= ~pin_mask;
    iowrite32be(qe_gc.cpdata, &regs.cpdata);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    return 0;
    }
    static int qe_gpio_set_multiple(struct gpio_chip *gc,
    unsigned long *mask, unsigned long *bits)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    unsigned long flags;
    int i;
    spin_lock_irqsave(&qe_gc.lock, flags);
    for (i = 0; i < gc.ngpio; i++) {
    if (*mask == 0)
    break;
    if (__test_and_clear_bit(i, mask)) {
    if (test_bit(i, bits))
    qe_gc.cpdata |= PIN_MASK(i);
    else
    qe_gc.cpdata &= ~PIN_MASK(i);
    }
    }
    iowrite32be(qe_gc.cpdata, &regs.cpdata);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_dir_in(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int qe_gpio_dir_in(struct gpio_chip *gc, unsigned int gpio)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    unsigned long flags;
    spin_lock_irqsave(&qe_gc.lock, flags);
    __par_io_config_pin(qe_gc.regs, gpio, QE_PIO_DIR_IN, 0, 0, 0);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_dir_out(gc: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    static int qe_gpio_dir_out(struct gpio_chip *gc, unsigned int gpio, int val)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    unsigned long flags;
    qe_gpio_set(gc, gpio, val);
    spin_lock_irqsave(&qe_gc.lock, flags);
    __par_io_config_pin(qe_gc.regs, gpio, QE_PIO_DIR_OUT, 0, 0, 0);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int qe_gpio_get_direction(struct gpio_chip *gc, unsigned int gpio)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    unsigned long flags;
    u32 val, mask;
    spin_lock_irqsave(&qe_gc.lock, flags);
    if (gpio < QE_PIO_PINS / 2)
    val = ioread32be(&regs.cpdir1);
    else
    val = ioread32be(&regs.cpdir2);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    mask = (u32)QE_PIO_DIR_OUT << (QE_PIO_PINS - 2 - (gpio % (QE_PIO_PINS / 2)) * 2);
    if (val & mask)
    return GPIO_LINE_DIRECTION_OUT;
    else
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn qe_gpio_to_irq(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int qe_gpio_to_irq(struct gpio_chip *gc, unsigned int gpio)
    {
    struct qe_gpio_chip *qe_gc = gpiochip_get_data(gc);
    struct of_phandle_args oirq;
    struct irq_domain *domain;
    int ret;
    oirq.np = qe_gc.np;
    oirq.args_count = 2;
    oirq.args[0] = gpio;
    oirq.args[1] = 0;
    ret = of_irq_parse_raw(core::ptr::null_mut(), &oirq);
    if (ret)
    return ret;
    domain = irq_find_host(oirq.np);
    if (!domain)
    return -EPROBE_DEFER;
    return irq_create_of_mapping(&oirq);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_pin {
//
// The qe_gpio_chip name is unfortunate, we should change that to
// something like qe_pio_controller. Someday.
//
    pub controller: *mut qe_gpio_chip,
    pub num: c_int,
}

//
// qe_pin_request - Request a QE pin
// @dev:	device to get the pin from
// @index:	index of the pin in the device tree
// Context:	non-atomic
//
// This function return qe_pin so that you could use it with the rest of
// the QE Pin Multiplexing API.
//
    struct qe_pin *qe_pin_request(struct device *dev, int index)
    {
    struct qe_pin *qe_pin;
    struct gpio_chip *gc;
    struct gpio_desc *gpiod;
    int gpio_num;
    int err;
    qe_pin = kzalloc_obj(*qe_pin);
    if (!qe_pin) {
    dev_dbg(dev, "%s: can't allocate memory\n", __func__);
    return ERR_PTR(-ENOMEM);
    }
//
// Request gpio as nonexclusive as it was likely reserved by the
// caller, and we are not planning on controlling it, we only need
// the descriptor to the to the gpio chip structure.
//
    gpiod = gpiod_get_index(dev, core::ptr::null_mut(), index,
    GPIOD_ASIS | GPIOD_FLAGS_BIT_NONEXCLUSIVE);
    err = PTR_ERR_OR_ZERO(gpiod);
    if (err)
    goto err0;
    gc = gpiod_to_chip(gpiod);
    gpio_num = desc_to_gpio(gpiod);
// We no longer need this descriptor
    gpiod_put(gpiod);
    if (WARN_ON(!gc)) {
    err = -ENODEV;
    goto err0;
    }
    qe_pin.controller = gpiochip_get_data(gc);
//
// FIXME: this gets the local offset on the gpio_chip so that the driver
// can manipulate pin control settings through its custom API. The real
// solution is to create a real pin control driver for this.
//
    qe_pin.num = gpio_num - gc.base;
    if (!fwnode_device_is_compatible(gc.fwnode, "fsl,mpc8323-qe-pario-bank")) {
    dev_dbg(dev, "%s: tried to get a non-qe pin\n", __func__);
    err = -EINVAL;
    goto err0;
    }
    return qe_pin;
    err0:
    kfree(qe_pin);
    dev_dbg(dev, "%s failed with status %d\n", __func__, err);
    return ERR_PTR(err);
    }
    EXPORT_SYMBOL(qe_pin_request);
//
// qe_pin_free - Free a pin
// @qe_pin:	pointer to the qe_pin structure
// Context:	any
//
// This function frees the qe_pin structure and makes a pin available
// for further qe_pin_request() calls.
//
#[no_mangle]
pub unsafe extern "C" fn qe_pin_free(qe_pin: *mut qe_pin) {
    void qe_pin_free(struct qe_pin *qe_pin)
    {
    kfree(qe_pin);
    }
    EXPORT_SYMBOL(qe_pin_free);
//
// qe_pin_set_dedicated - Revert a pin to a dedicated peripheral function mode
// @qe_pin:	pointer to the qe_pin structure
// Context:	any
//
// This function resets a pin to a dedicated peripheral function that
// has been set up by the firmware.
//
#[no_mangle]
pub unsafe extern "C" fn qe_pin_set_dedicated(qe_pin: *mut qe_pin) {
    void qe_pin_set_dedicated(struct qe_pin *qe_pin)
    {
    struct qe_gpio_chip *qe_gc = qe_pin.controller;
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    struct qe_pio_regs *sregs = &qe_gc.saved_regs;
    let mut pin: c_int = qe_pin.num;
    let mut mask1: u32 = 1 << (QE_PIO_PINS - (pin + 1));
    let mut mask2: u32 = 0x3 << (QE_PIO_PINS - (pin % (QE_PIO_PINS / 2) + 1) * 2);
    let mut second_reg: bool = pin > (QE_PIO_PINS / 2) - 1;
    unsigned long flags;
    spin_lock_irqsave(&qe_gc.lock, flags);
    if (second_reg) {
    qe_clrsetbits_be32(&regs.cpdir2, mask2,
    sregs.cpdir2 & mask2);
    qe_clrsetbits_be32(&regs.cppar2, mask2,
    sregs.cppar2 & mask2);
    } else {
    qe_clrsetbits_be32(&regs.cpdir1, mask2,
    sregs.cpdir1 & mask2);
    qe_clrsetbits_be32(&regs.cppar1, mask2,
    sregs.cppar1 & mask2);
    }
    if (sregs.cpdata & mask1)
    qe_gc.cpdata |= mask1;
    else
    qe_gc.cpdata &= ~mask1;
    iowrite32be(qe_gc.cpdata, &regs.cpdata);
    qe_clrsetbits_be32(&regs.cpodr, mask1, sregs.cpodr & mask1);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    }
    EXPORT_SYMBOL(qe_pin_set_dedicated);
//
// qe_pin_set_gpio - Set a pin to the GPIO mode
// @qe_pin:	pointer to the qe_pin structure
// Context:	any
//
// This function sets a pin to the GPIO mode.
//
#[no_mangle]
pub unsafe extern "C" fn qe_pin_set_gpio(qe_pin: *mut qe_pin) {
    void qe_pin_set_gpio(struct qe_pin *qe_pin)
    {
    struct qe_gpio_chip *qe_gc = qe_pin.controller;
    struct qe_pio_regs __iomem *regs = qe_gc.regs;
    unsigned long flags;
    spin_lock_irqsave(&qe_gc.lock, flags);
// Let's make it input by default, GPIO API is able to change that.
    __par_io_config_pin(regs, qe_pin.num, QE_PIO_DIR_IN, 0, 0, 0);
    spin_unlock_irqrestore(&qe_gc.lock, flags);
    }
    EXPORT_SYMBOL(qe_pin_set_gpio);
#[no_mangle]
unsafe extern "C" fn qe_gpio_probe(ofdev: *mut platform_device) -> c_int {
    static int qe_gpio_probe(struct platform_device *ofdev)
    {
    struct device *dev = &ofdev.dev;
    struct device_node *np = dev.of_node;
    struct qe_gpio_chip *qe_gc;
    struct gpio_chip *gc;
    qe_gc = devm_kzalloc(dev, sizeof(*qe_gc), GFP_KERNEL);
    if (!qe_gc)
    return -ENOMEM;
    qe_gc.np = np;
    spin_lock_init(&qe_gc.lock);
    gc = &qe_gc.gc;
    gc.base = -1;
    gc.ngpio = QE_PIO_PINS;
    gc.direction_input = qe_gpio_dir_in;
    gc.direction_output = qe_gpio_dir_out;
    gc.get_direction = qe_gpio_get_direction;
    gc.get = qe_gpio_get;
    gc.set = qe_gpio_set;
    gc.set_multiple = qe_gpio_set_multiple;
    gc.to_irq = qe_gpio_to_irq;
    gc.parent = dev;
    gc.owner = THIS_MODULE;
    gc.label = devm_kasprintf(dev, GFP_KERNEL, "%pOF", np);
    if (!gc.label)
    return -ENOMEM;
    qe_gc.regs = devm_of_iomap(dev, np, 0, core::ptr::null_mut());
    if (IS_ERR(qe_gc.regs))
    return PTR_ERR(qe_gc.regs);
    qe_gpio_save_regs(qe_gc);
    return devm_gpiochip_add_data(dev, gc, qe_gc);
    }
    static const struct of_device_id qe_gpio_match[] = {
    {
    .compatible = "fsl,mpc8323-qe-pario-bank",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, qe_gpio_match);
    static struct platform_driver qe_gpio_driver = {
    .probe		= qe_gpio_probe,
    .driver		= {
    .name	= "qe-gpio",
    .of_match_table	= qe_gpio_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn qe_gpio_init() -> int __init {
    static int __init qe_gpio_init(void)
    {
    return platform_driver_register(&qe_gpio_driver);
    }
    arch_initcall(qe_gpio_init);
