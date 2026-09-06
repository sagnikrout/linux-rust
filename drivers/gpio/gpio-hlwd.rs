//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-hlwd.c
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
// Copyright (C) 2008-2009 The GameCube Linux Team
// Copyright (C) 2008,2009 Albert Herranz
// Copyright (C) 2017-2018 Jonathan Neuschäfer
//
// Nintendo Wii (Hollywood) GPIO driver

//
// Register names and offsets courtesy of WiiBrew:
// https://wiibrew.org/wiki/Hardware/Hollywood_GPIOs
//
// Note that for most registers, there are two versions:
// - HW_GPIOB_* Is always accessible by the Broadway PowerPC core, but does
// always give access to all GPIO lines
// - HW_GPIO_* Is only accessible by the Broadway PowerPC code if the memory
// firewall (AHBPROT) in the Hollywood chipset has been configured to allow
// such access.
//
// The ownership of each GPIO line can be configured in the HW_GPIO_OWNER
// register: A one bit configures the line for access via the HW_GPIOB_
// registers, a zero bit indicates access via HW_GPIO_*. This driver uses
// HW_GPIOB_*.
//
pub const HW_GPIOB_OUT: c_uint = 0x00;
pub const HW_GPIOB_DIR: c_uint = 0x04;
pub const HW_GPIOB_IN: c_uint = 0x08;
pub const HW_GPIOB_INTLVL: c_uint = 0x0c;
pub const HW_GPIOB_INTFLAG: c_uint = 0x10;
pub const HW_GPIOB_INTMASK: c_uint = 0x14;
pub const HW_GPIOB_INMIR: c_uint = 0x18;
pub const HW_GPIO_ENABLE: c_uint = 0x1c;
pub const HW_GPIO_OUT: c_uint = 0x20;
pub const HW_GPIO_DIR: c_uint = 0x24;
pub const HW_GPIO_IN: c_uint = 0x28;
pub const HW_GPIO_INTLVL: c_uint = 0x2c;
pub const HW_GPIO_INTFLAG: c_uint = 0x30;
pub const HW_GPIO_INTMASK: c_uint = 0x34;
pub const HW_GPIO_INMIR: c_uint = 0x38;
pub const HW_GPIO_OWNER: c_uint = 0x3c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlwd_gpio {
    pub gpioc: gpio_generic_chip,
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub irq: c_int,
    pub edge_emulation: u32,
    pub falling_edge: u32 rising_edge,,
}

#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irqhandler(desc: *mut irq_desc) {
    static void hlwd_gpio_irqhandler(struct irq_desc *desc)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_desc_get_handler_data(desc));
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long pending;
    int hwirq;
    u32 emulated_pending;
    scoped_guard(gpio_generic_lock_irqsave, &hlwd.gpioc) {
    pending = ioread32be(hlwd.regs + HW_GPIOB_INTFLAG);
    pending &= ioread32be(hlwd.regs + HW_GPIOB_INTMASK);
// Treat interrupts due to edge trigger emulation separately
    emulated_pending = hlwd.edge_emulation & pending;
    pending &= ~emulated_pending;
    if (emulated_pending) {
    u32 level, rising, falling;
    level = ioread32be(hlwd.regs + HW_GPIOB_INTLVL);
    rising = level & emulated_pending;
    falling = ~level & emulated_pending;
// Invert the levels
    iowrite32be(level ^ emulated_pending,
    hlwd.regs + HW_GPIOB_INTLVL);
// Ack all emulated-edge interrupts
    iowrite32be(emulated_pending, hlwd.regs + HW_GPIOB_INTFLAG);
// Signal interrupts only on the correct edge
    rising &= hlwd.rising_edge;
    falling &= hlwd.falling_edge;
// Mark emulated interrupts as pending
    pending |= rising | falling;
    }
    }
    chained_irq_enter(chip, desc);
    for_each_set_bit(hwirq, &pending, 32)
    generic_handle_domain_irq(hlwd.gpioc.gc.irq.domain, hwirq);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_ack(data: *mut irq_data) {
    static void hlwd_gpio_irq_ack(struct irq_data *data)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_data_get_irq_chip_data(data));
    iowrite32be(BIT(data.hwirq), hlwd.regs + HW_GPIOB_INTFLAG);
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_mask(data: *mut irq_data) {
    static void hlwd_gpio_irq_mask(struct irq_data *data)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_data_get_irq_chip_data(data));
    u32 mask;
    scoped_guard(gpio_generic_lock_irqsave, &hlwd.gpioc) {
    mask = ioread32be(hlwd.regs + HW_GPIOB_INTMASK);
    mask &= ~BIT(data.hwirq);
    iowrite32be(mask, hlwd.regs + HW_GPIOB_INTMASK);
    }
    gpiochip_disable_irq(&hlwd.gpioc.gc, irqd_to_hwirq(data));
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_unmask(data: *mut irq_data) {
    static void hlwd_gpio_irq_unmask(struct irq_data *data)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_data_get_irq_chip_data(data));
    u32 mask;
    gpiochip_enable_irq(&hlwd.gpioc.gc, irqd_to_hwirq(data));
    guard(gpio_generic_lock_irqsave)(&hlwd.gpioc);
    mask = ioread32be(hlwd.regs + HW_GPIOB_INTMASK);
    mask |= BIT(data.hwirq);
    iowrite32be(mask, hlwd.regs + HW_GPIOB_INTMASK);
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_enable(data: *mut irq_data) {
    static void hlwd_gpio_irq_enable(struct irq_data *data)
    {
    hlwd_gpio_irq_ack(data);
    hlwd_gpio_irq_unmask(data);
    }
    static void hlwd_gpio_irq_setup_emulation(struct hlwd_gpio *hlwd, int hwirq,
    unsigned int flow_type)
    {
    u32 level, state;
// Set the trigger level to the inactive level
    level = ioread32be(hlwd.regs + HW_GPIOB_INTLVL);
    state = ioread32be(hlwd.regs + HW_GPIOB_IN) & BIT(hwirq);
    level &= ~BIT(hwirq);
    level |= state ^ BIT(hwirq);
    iowrite32be(level, hlwd.regs + HW_GPIOB_INTLVL);
    hlwd.edge_emulation |= BIT(hwirq);
    hlwd.rising_edge &= ~BIT(hwirq);
    hlwd.falling_edge &= ~BIT(hwirq);
    if (flow_type & IRQ_TYPE_EDGE_RISING)
    hlwd.rising_edge |= BIT(hwirq);
    if (flow_type & IRQ_TYPE_EDGE_FALLING)
    hlwd.falling_edge |= BIT(hwirq);
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_set_type(data: *mut irq_data, flow_type: c_uint) -> c_int {
    static int hlwd_gpio_irq_set_type(struct irq_data *data, unsigned int flow_type)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_data_get_irq_chip_data(data));
    u32 level;
    guard(gpio_generic_lock_irqsave)(&hlwd.gpioc);
    hlwd.edge_emulation &= ~BIT(data.hwirq);
    switch (flow_type) {
    case IRQ_TYPE_LEVEL_HIGH:
    level = ioread32be(hlwd.regs + HW_GPIOB_INTLVL);
    level |= BIT(data.hwirq);
    iowrite32be(level, hlwd.regs + HW_GPIOB_INTLVL);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    level = ioread32be(hlwd.regs + HW_GPIOB_INTLVL);
    level &= ~BIT(data.hwirq);
    iowrite32be(level, hlwd.regs + HW_GPIOB_INTLVL);
    break;
    case IRQ_TYPE_EDGE_RISING:
    case IRQ_TYPE_EDGE_FALLING:
    case IRQ_TYPE_EDGE_BOTH:
    hlwd_gpio_irq_setup_emulation(hlwd, data.hwirq, flow_type);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    static void hlwd_gpio_irq_print_chip(struct irq_data *data, struct seq_file *p)
    {
    struct hlwd_gpio *hlwd =
    gpiochip_get_data(irq_data_get_irq_chip_data(data));
    seq_puts(p, dev_name(hlwd.dev));
    }
    static const struct irq_chip hlwd_gpio_irq_chip = {
    .irq_mask = hlwd_gpio_irq_mask,
    .irq_unmask = hlwd_gpio_irq_unmask,
    .irq_enable = hlwd_gpio_irq_enable,
    .irq_set_type = hlwd_gpio_irq_set_type,
    .irq_print_chip = hlwd_gpio_irq_print_chip,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn hlwd_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int hlwd_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct hlwd_gpio *hlwd;
    u32 ngpios;
    int res;
    hlwd = devm_kzalloc(&pdev.dev, sizeof(*hlwd), GFP_KERNEL);
    if (!hlwd)
    return -ENOMEM;
    hlwd.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hlwd.regs))
    return PTR_ERR(hlwd.regs);
    hlwd.dev = &pdev.dev;
//
// Claim all GPIOs using the OWNER register. This will not work on
// systems where the AHBPROT memory firewall hasn't been configured to
// permit PPC access to HW_GPIO_*.
//
// Note that this has to happen before gpio_generic_chip_init() reads
// the HW_GPIOB_OUT and HW_GPIOB_DIR, because otherwise it reads the
// wrong values.
//
    iowrite32be(0xffffffff, hlwd.regs + HW_GPIO_OWNER);
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 4,
    .dat = hlwd.regs + HW_GPIOB_IN,
    .set = hlwd.regs + HW_GPIOB_OUT,
    .dirout = hlwd.regs + HW_GPIOB_DIR,
    .flags = GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };
    res = gpio_generic_chip_init(&hlwd.gpioc, &config);
    if (res < 0) {
    dev_warn(&pdev.dev, "failed to initialize generic GPIO chip: %d\n", res);
    return res;
    }
    res = of_property_read_u32(pdev.dev.of_node, "ngpios", &ngpios);
    if (res)
    ngpios = 32;
    hlwd.gpioc.gc.ngpio = ngpios;
// Mask and ack all interrupts
    iowrite32be(0, hlwd.regs + HW_GPIOB_INTMASK);
    iowrite32be(0xffffffff, hlwd.regs + HW_GPIOB_INTFLAG);
//
// If this GPIO controller is not marked as an interrupt controller in
// the DT, skip interrupt support.
//
    if (of_property_read_bool(pdev.dev.of_node, "interrupt-controller")) {
    struct gpio_irq_chip *girq;
    hlwd.irq = platform_get_irq(pdev, 0);
    if (hlwd.irq < 0) {
    dev_info(&pdev.dev, "platform_get_irq returned %d\n",
    hlwd.irq);
    return hlwd.irq;
    }
    girq = &hlwd.gpioc.gc.irq;
    gpio_irq_chip_set_chip(girq, &hlwd_gpio_irq_chip);
    girq.parent_handler = hlwd_gpio_irqhandler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = hlwd.irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_level_irq;
    }
    return devm_gpiochip_add_data(&pdev.dev, &hlwd.gpioc.gc, hlwd);
    }
    static const struct of_device_id hlwd_gpio_match[] = {
    { .compatible = "nintendo,hollywood-gpio", },
    {},
    };
    MODULE_DEVICE_TABLE(of, hlwd_gpio_match);
    static struct platform_driver hlwd_gpio_driver = {
    .driver	= {
    .name		= "gpio-hlwd",
    .of_match_table	= hlwd_gpio_match,
    },
    .probe	= hlwd_gpio_probe,
    };
    module_platform_driver(hlwd_gpio_driver);
    MODULE_AUTHOR("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
    MODULE_DESCRIPTION("Nintendo Wii GPIO driver");
    MODULE_LICENSE("GPL");
