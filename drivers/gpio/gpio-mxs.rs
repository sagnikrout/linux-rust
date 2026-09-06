//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-mxs.c
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
// MXS GPIO support. (c) 2008 Daniel Mack <daniel@caiaq.de>
// Copyright 2008 Juergen Beisert, kernel@pengutronix.de
//
// Based on code from Freescale,
// Copyright (C) 2004-2010 Freescale Semiconductor, Inc. All Rights Reserved.

pub const MXS_SET: c_uint = 0x4;
pub const MXS_CLR: c_uint = 0x8;

pub const GPIO_INT_FALL_EDGE: c_uint = 0x0;
pub const GPIO_INT_LOW_LEV: c_uint = 0x1;
pub const GPIO_INT_RISE_EDGE: c_uint = 0x2;
pub const GPIO_INT_HIGH_LEV: c_uint = 0x3;

    enum mxs_gpio_id {
    IMX23_GPIO,
    IMX28_GPIO,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxs_gpio_port {
    pub base: *mut void __iomem,
    pub id: c_int,
    pub irq: c_int,
    pub domain: *mut irq_domain,
    pub chip: gpio_generic_chip,
    pub dev: *mut device,
    pub devid: enum mxs_gpio_id,
    pub both_edges: u32,
}

#[no_mangle]
pub unsafe extern "C" fn is_imx23_gpio(port: *mut mxs_gpio_port) -> c_int {
    static inline int is_imx23_gpio(struct mxs_gpio_port *port)
    {
    return port.devid == IMX23_GPIO;
    }
// Note: This driver assumes 32 GPIOs are handled in one register
#[no_mangle]
unsafe extern "C" fn mxs_gpio_set_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int mxs_gpio_set_irq_type(struct irq_data *d, unsigned int type)
    {
    u32 val;
    let mut pin_mask: u32 = 1 << d.hwirq;
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct irq_chip_type *ct = irq_data_get_chip_type(d);
    struct mxs_gpio_port *port = gc.private;
    void __iomem *pin_addr;
    int edge;
    if (!(ct.type & type))
    if (irq_setup_alt_chip(d, type))
    return -EINVAL;
    port.both_edges &= ~pin_mask;
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    val = readl(port.base + PINCTRL_DIN(port)) & pin_mask;
    if (val)
    edge = GPIO_INT_FALL_EDGE;
    else
    edge = GPIO_INT_RISE_EDGE;
    port.both_edges |= pin_mask;
    break;
    case IRQ_TYPE_EDGE_RISING:
    edge = GPIO_INT_RISE_EDGE;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    edge = GPIO_INT_FALL_EDGE;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    edge = GPIO_INT_LOW_LEV;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    edge = GPIO_INT_HIGH_LEV;
    break;
    default:
    return -EINVAL;
    }
// set level or edge
    pin_addr = port.base + PINCTRL_IRQLEV(port);
    if (edge & GPIO_INT_LEV_MASK) {
    writel(pin_mask, pin_addr + MXS_SET);
    writel(pin_mask, port.base + PINCTRL_IRQEN(port) + MXS_SET);
    } else {
    writel(pin_mask, pin_addr + MXS_CLR);
    writel(pin_mask, port.base + PINCTRL_PIN2IRQ(port) + MXS_SET);
    }
// set polarity
    pin_addr = port.base + PINCTRL_IRQPOL(port);
    if (edge & GPIO_INT_POL_MASK)
    writel(pin_mask, pin_addr + MXS_SET);
    else
    writel(pin_mask, pin_addr + MXS_CLR);
    writel(pin_mask, port.base + PINCTRL_IRQSTAT(port) + MXS_CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxs_flip_edge(port: *mut mxs_gpio_port, gpio: u32) {
    static void mxs_flip_edge(struct mxs_gpio_port *port, u32 gpio)
    {
    u32 bit, val, edge;
    void __iomem *pin_addr;
    bit = 1 << gpio;
    pin_addr = port.base + PINCTRL_IRQPOL(port);
    val = readl(pin_addr);
    edge = val & bit;
    if (edge)
    writel(bit, pin_addr + MXS_CLR);
    else
    writel(bit, pin_addr + MXS_SET);
    }
// MXS has one interrupt *per* gpio port
#[no_mangle]
unsafe extern "C" fn mxs_gpio_irq_handler(desc: *mut irq_desc) {
    static void mxs_gpio_irq_handler(struct irq_desc *desc)
    {
    u32 irq_stat;
    struct mxs_gpio_port *port = irq_desc_get_handler_data(desc);
    desc.irq_data.chip.irq_ack(&desc.irq_data);
    irq_stat = readl(port.base + PINCTRL_IRQSTAT(port)) &
    readl(port.base + PINCTRL_IRQEN(port));
    while (irq_stat != 0) {
    let mut irqoffset: c_int = fls(irq_stat) - 1;
    if (port.both_edges & (1 << irqoffset))
    mxs_flip_edge(port, irqoffset);
    generic_handle_domain_irq(port.domain, irqoffset);
    irq_stat &= ~(1 << irqoffset);
    }
    }
//
// Set interrupt number "irq" in the GPIO as a wake-up source.
// While system is running, all registered GPIO interrupts need to have
// wake-up enabled. When system is suspended, only selected GPIO interrupts
// need to have wake-up enabled.
// @param  irq          interrupt source number
// @param  enable       enable as wake-up if equal to non-zero
// @return       This function returns 0 on success.
//
#[no_mangle]
unsafe extern "C" fn mxs_gpio_set_wake_irq(d: *mut irq_data, enable: c_uint) -> c_int {
    static int mxs_gpio_set_wake_irq(struct irq_data *d, unsigned int enable)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(d);
    struct mxs_gpio_port *port = gc.private;
    if (enable)
    enable_irq_wake(port.irq);
    else
    disable_irq_wake(port.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mxs_gpio_init_gc(port: *mut mxs_gpio_port, irq_base: c_int) -> c_int {
    static int mxs_gpio_init_gc(struct mxs_gpio_port *port, int irq_base)
    {
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    int rv;
    gc = devm_irq_alloc_generic_chip(port.dev, "gpio-mxs", 2, irq_base,
    port.base, handle_level_irq);
    if (!gc)
    return -ENOMEM;
    gc.private = port;
    ct = &gc.chip_types[0];
    ct.type = IRQ_TYPE_LEVEL_MASK;
    ct.chip.irq_ack = irq_gc_ack_set_bit;
    ct.chip.irq_mask = irq_gc_mask_disable_reg;
    ct.chip.irq_unmask = irq_gc_unmask_enable_reg;
    ct.chip.irq_set_type = mxs_gpio_set_irq_type;
    ct.chip.irq_set_wake = mxs_gpio_set_wake_irq;
    ct.chip.flags = IRQCHIP_SET_TYPE_MASKED;
    ct.regs.ack = PINCTRL_IRQSTAT(port) + MXS_CLR;
    ct.regs.enable = PINCTRL_PIN2IRQ(port) + MXS_SET;
    ct.regs.disable = PINCTRL_PIN2IRQ(port) + MXS_CLR;
    ct = &gc.chip_types[1];
    ct.type = IRQ_TYPE_EDGE_BOTH;
    ct.chip.irq_ack = irq_gc_ack_set_bit;
    ct.chip.irq_mask = irq_gc_mask_disable_reg;
    ct.chip.irq_unmask = irq_gc_unmask_enable_reg;
    ct.chip.irq_set_type = mxs_gpio_set_irq_type;
    ct.chip.irq_set_wake = mxs_gpio_set_wake_irq;
    ct.chip.flags = IRQCHIP_SET_TYPE_MASKED;
    ct.regs.ack = PINCTRL_IRQSTAT(port) + MXS_CLR;
    ct.regs.enable = PINCTRL_IRQEN(port) + MXS_SET;
    ct.regs.disable = PINCTRL_IRQEN(port) + MXS_CLR;
    ct.handler = handle_level_irq;
    rv = devm_irq_setup_generic_chip(port.dev, gc, IRQ_MSK(32),
    IRQ_GC_INIT_NESTED_LOCK,
    IRQ_NOREQUEST, 0);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn mxs_gpio_to_irq(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int mxs_gpio_to_irq(struct gpio_chip *gc, unsigned int offset)
    {
    struct mxs_gpio_port *port = gpiochip_get_data(gc);
    return irq_find_mapping(port.domain, offset);
    }
#[no_mangle]
unsafe extern "C" fn mxs_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int mxs_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    struct mxs_gpio_port *port = gpiochip_get_data(gc);
    let mut mask: u32 = 1 << offset;
    u32 dir;
    dir = readl(port.base + PINCTRL_DOE(port));
    if (dir & mask)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
    static const struct of_device_id mxs_gpio_dt_ids[] = {
    { .compatible = "fsl,imx23-gpio", .data = (void *) IMX23_GPIO, },
    { .compatible = "fsl,imx28-gpio", .data = (void *) IMX28_GPIO, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mxs_gpio_dt_ids);
#[no_mangle]
unsafe extern "C" fn mxs_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int mxs_gpio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct gpio_generic_chip_config config;
    struct device_node *parent;
    static void __iomem *base;
    struct mxs_gpio_port *port;
    int irq_base;
    int err;
    port = devm_kzalloc(&pdev.dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    port.id = of_alias_get_id(np, "gpio");
    if (port.id < 0)
    return port.id;
    port.devid = (uintptr_t)of_device_get_match_data(&pdev.dev);
    port.dev = &pdev.dev;
    port.irq = platform_get_irq(pdev, 0);
    if (port.irq < 0)
    return port.irq;
//
// map memory region only once, as all the gpio ports
// share the same one
//
    if (!base) {
    parent = of_get_parent(np);
    base = of_iomap(parent, 0);
    of_node_put(parent);
    if (!base)
    return -EADDRNOTAVAIL;
    }
    port.base = base;
// initially disable the interrupts
    writel(0, port.base + PINCTRL_PIN2IRQ(port));
    writel(0, port.base + PINCTRL_IRQEN(port));
// clear address has to be used to clear IRQSTAT bits
    writel(~0U, port.base + PINCTRL_IRQSTAT(port) + MXS_CLR);
    irq_base = devm_irq_alloc_descs(&pdev.dev, -1, 0, 32, numa_node_id());
    if (irq_base < 0) {
    err = irq_base;
    goto out_iounmap;
    }
    port.domain = irq_domain_create_legacy(dev_fwnode(&pdev.dev), 32, irq_base, 0,
    &irq_domain_simple_ops, core::ptr::null_mut());
    if (!port.domain) {
    err = -ENODEV;
    goto out_iounmap;
    }
// gpio-mxs can be a generic irq chip
    err = mxs_gpio_init_gc(port, irq_base);
    if (err < 0)
    goto out_irqdomain_remove;
// setup one handler for each entry
    irq_set_chained_handler_and_data(port.irq, mxs_gpio_irq_handler,
    port);
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 4,
    .dat = port.base + PINCTRL_DIN(port),
    .set = port.base + PINCTRL_DOUT(port) + MXS_SET,
    .clr = port.base + PINCTRL_DOUT(port) + MXS_CLR,
    .dirout = port.base + PINCTRL_DOE(port),
    };
    err = gpio_generic_chip_init(&port.chip, &config);
    if (err)
    goto out_irqdomain_remove;
    port.chip.gc.to_irq = mxs_gpio_to_irq;
    port.chip.gc.get_direction = mxs_gpio_get_direction;
    port.chip.gc.base = port.id * 32;
    err = gpiochip_add_data(&port.chip.gc, port);
    if (err)
    goto out_irqdomain_remove;
    return 0;
    out_irqdomain_remove:
    irq_domain_remove(port.domain);
    out_iounmap:
    iounmap(port.base);
    return err;
    }
    static struct platform_driver mxs_gpio_driver = {
    .driver		= {
    .name	= "gpio-mxs",
    .of_match_table = mxs_gpio_dt_ids,
    .suppress_bind_attrs = true,
    },
    .probe		= mxs_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn mxs_gpio_init() -> int __init {
    static int __init mxs_gpio_init(void)
    {
    return platform_driver_register(&mxs_gpio_driver);
    }
    postcore_initcall(mxs_gpio_init);
    MODULE_AUTHOR("Freescale Semiconductor, "
    "Daniel Mack <danielncaiaq.de>, "
    "Juergen Beisert <kernel@pengutronix.de>");
    MODULE_DESCRIPTION("Freescale MXS GPIO");
