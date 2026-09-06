//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-sch.c
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


// SPDX-License-Identifier: GPL-2.0
//
// GPIO interface for Intel Poulsbo SCH
//
// Copyright (c) 2010 CompuLab Ltd
// Author: Denis Turischev <denis@compulab.co.il>
//

pub const GEN: c_uint = 0x00;
pub const GIO: c_uint = 0x04;
pub const GLV: c_uint = 0x08;
pub const GTPE: c_uint = 0x0c;
pub const GTNE: c_uint = 0x10;
pub const GGPE: c_uint = 0x14;
pub const GSMI: c_uint = 0x18;
pub const GTS: c_uint = 0x1c;
pub const CORE_BANK_OFFSET: c_uint = 0x00;
pub const RESUME_BANK_OFFSET: c_uint = 0x20;
//
// iLB datasheet describes GPE0BLK registers, in particular GPE0E.GPIO bit.
// Document Number: 328195-001
//
pub const GPE0E_GPIO: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sch_gpio {
    pub chip: gpio_chip,
    pub regs: *mut void __iomem,
    pub lock: raw_spinlock_t,
    pub resume_base: c_ushort,
// GPE handling
    pub gpe: u32,
    pub gpe_handler: acpi_gpe_handler,
}

    static unsigned int sch_gpio_offset(struct sch_gpio *sch, unsigned int gpio,
    unsigned int reg)
    {
    let mut base: c_uint = CORE_BANK_OFFSET;
    if (gpio >= sch.resume_base) {
    gpio -= sch.resume_base;
    base = RESUME_BANK_OFFSET;
    }
    return base + reg + gpio / 8;
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_bit(sch: *mut sch_gpio, gpio: c_uint) -> c_uint {
    static unsigned int sch_gpio_bit(struct sch_gpio *sch, unsigned int gpio)
    {
    if (gpio >= sch.resume_base)
    gpio -= sch.resume_base;
    return gpio % 8;
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_reg_get(sch: *mut sch_gpio, gpio: c_uint, reg: c_uint) -> c_int {
    static int sch_gpio_reg_get(struct sch_gpio *sch, unsigned int gpio, unsigned int reg)
    {
    unsigned short offset, bit;
    u8 reg_val;
    offset = sch_gpio_offset(sch, gpio, reg);
    bit = sch_gpio_bit(sch, gpio);
    reg_val = !!(ioread8(sch.regs + offset) & BIT(bit));
    return reg_val;
    }
    static void sch_gpio_reg_set(struct sch_gpio *sch, unsigned int gpio, unsigned int reg,
    int val)
    {
    unsigned short offset, bit;
    u8 reg_val;
    offset = sch_gpio_offset(sch, gpio, reg);
    bit = sch_gpio_bit(sch, gpio);
    reg_val = ioread8(sch.regs + offset);
    if (val)
    reg_val |= BIT(bit);
    else
    reg_val &= ~BIT(bit);
    iowrite8(reg_val, sch.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_direction_in(gc: *mut gpio_chip, gpio_num: c_uint) -> c_int {
    static int sch_gpio_direction_in(struct gpio_chip *gc, unsigned int gpio_num)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    unsigned long flags;
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GIO, 1);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_get(gc: *mut gpio_chip, gpio_num: c_uint) -> c_int {
    static int sch_gpio_get(struct gpio_chip *gc, unsigned int gpio_num)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    return sch_gpio_reg_get(sch, gpio_num, GLV);
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_set(gc: *mut gpio_chip, gpio_num: c_uint, val: c_int) -> c_int {
    static int sch_gpio_set(struct gpio_chip *gc, unsigned int gpio_num, int val)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    unsigned long flags;
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GLV, val);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    return 0;
    }
    static int sch_gpio_direction_out(struct gpio_chip *gc, unsigned int gpio_num,
    int val)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    unsigned long flags;
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GIO, 0);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
//
// according to the datasheet, writing to the level register has no
// effect when GPIO is programmed as input.
// Actually the level register is read-only when configured as input.
// Thus presetting the output level before switching to output is _NOT_ possible.
// Hence we set the level after configuring the GPIO as output.
// But we cannot prevent a short low pulse if direction is set to high
// and an external pull-up is connected.
//
    return sch_gpio_set(gc, gpio_num, val);
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_get_direction(gc: *mut gpio_chip, gpio_num: c_uint) -> c_int {
    static int sch_gpio_get_direction(struct gpio_chip *gc, unsigned int gpio_num)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    if (sch_gpio_reg_get(sch, gpio_num, GIO))
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
    static const struct gpio_chip sch_gpio_chip = {
    .label			= "sch_gpio",
    .owner			= THIS_MODULE,
    .direction_input	= sch_gpio_direction_in,
    .get			= sch_gpio_get,
    .direction_output	= sch_gpio_direction_out,
    .set			= sch_gpio_set,
    .get_direction		= sch_gpio_get_direction,
    };
#[no_mangle]
unsafe extern "C" fn sch_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int sch_irq_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sch_gpio *sch = gpiochip_get_data(gc);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    int rising, falling;
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_RISING:
    rising = 1;
    falling = 0;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    rising = 0;
    falling = 1;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    rising = 1;
    falling = 1;
    break;
    default:
    return -EINVAL;
    }
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GTPE, rising);
    sch_gpio_reg_set(sch, gpio_num, GTNE, falling);
    irq_set_handler_locked(d, handle_edge_irq);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sch_irq_ack(d: *mut irq_data) {
    static void sch_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct sch_gpio *sch = gpiochip_get_data(gc);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GTS, 1);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sch_irq_mask_unmask(gc: *mut gpio_chip, gpio_num: irq_hw_number_t, val: c_int) {
    static void sch_irq_mask_unmask(struct gpio_chip *gc, irq_hw_number_t gpio_num, int val)
    {
    struct sch_gpio *sch = gpiochip_get_data(gc);
    unsigned long flags;
    raw_spin_lock_irqsave(&sch.lock, flags);
    sch_gpio_reg_set(sch, gpio_num, GGPE, val);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn sch_irq_mask(d: *mut irq_data) {
    static void sch_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    sch_irq_mask_unmask(gc, gpio_num, 0);
    gpiochip_disable_irq(gc, gpio_num);
    }
#[no_mangle]
unsafe extern "C" fn sch_irq_unmask(d: *mut irq_data) {
    static void sch_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, gpio_num);
    sch_irq_mask_unmask(gc, gpio_num, 1);
    }
    static const struct irq_chip sch_irqchip = {
    .name = "sch_gpio",
    .irq_ack = sch_irq_ack,
    .irq_mask = sch_irq_mask,
    .irq_unmask = sch_irq_unmask,
    .irq_set_type = sch_irq_type,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn sch_gpio_gpe_handler(gpe_device: acpi_handle, gpe: u32, context: *mut c_void) -> u32 {
    static u32 sch_gpio_gpe_handler(acpi_handle gpe_device, u32 gpe, void *context)
    {
    struct sch_gpio *sch = context;
    struct gpio_chip *gc = &sch.chip;
    unsigned long core_status, resume_status;
    unsigned long pending;
    unsigned long flags;
    int offset;
    u32 ret;
    raw_spin_lock_irqsave(&sch.lock, flags);
    core_status = ioread32(sch.regs + CORE_BANK_OFFSET + GTS);
    resume_status = ioread32(sch.regs + RESUME_BANK_OFFSET + GTS);
    raw_spin_unlock_irqrestore(&sch.lock, flags);
    pending = (resume_status << sch.resume_base) | core_status;
    for_each_set_bit(offset, &pending, sch.chip.ngpio)
    generic_handle_domain_irq(gc.irq.domain, offset);
// Set returning value depending on whether we handled an interrupt
    ret = pending ? ACPI_INTERRUPT_HANDLED : ACPI_INTERRUPT_NOT_HANDLED;
// Acknowledge GPE to ACPICA
    ret |= ACPI_REENABLE_GPE;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_remove_gpe_handler(data: *mut c_void) {
    static void sch_gpio_remove_gpe_handler(void *data)
    {
    struct sch_gpio *sch = data;
    acpi_disable_gpe(core::ptr::null_mut(), sch.gpe);
    acpi_remove_gpe_handler(core::ptr::null_mut(), sch.gpe, sch.gpe_handler);
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_install_gpe_handler(sch: *mut sch_gpio) -> c_int {
    static int sch_gpio_install_gpe_handler(struct sch_gpio *sch)
    {
    struct device *dev = sch.chip.parent;
    acpi_status status;
    status = acpi_install_gpe_handler(core::ptr::null_mut(), sch.gpe, ACPI_GPE_LEVEL_TRIGGERED,
    sch.gpe_handler, sch);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "Failed to install GPE handler for %u: %s\n",
    sch.gpe, acpi_format_exception(status));
    return -ENODEV;
    }
    status = acpi_enable_gpe(core::ptr::null_mut(), sch.gpe);
    if (ACPI_FAILURE(status)) {
    dev_err(dev, "Failed to enable GPE handler for %u: %s\n",
    sch.gpe, acpi_format_exception(status));
    acpi_remove_gpe_handler(core::ptr::null_mut(), sch.gpe, sch.gpe_handler);
    return -ENODEV;
    }
    return devm_add_action_or_reset(dev, sch_gpio_remove_gpe_handler, sch);
    }
#[no_mangle]
unsafe extern "C" fn sch_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int sch_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct gpio_irq_chip *girq;
    struct sch_gpio *sch;
    struct resource *res;
    void __iomem *regs;
    int ret;
    sch = devm_kzalloc(dev, sizeof(*sch), GFP_KERNEL);
    if (!sch)
    return -ENOMEM;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    return -EBUSY;
    regs = devm_ioport_map(dev, res.start, resource_size(res));
    if (!regs)
    return -EBUSY;
    sch.regs = regs;
    raw_spin_lock_init(&sch.lock);
    sch.chip = sch_gpio_chip;
    sch.chip.label = dev_name(dev);
    sch.chip.parent = dev;
    switch (pdev.id) {
    case PCI_DEVICE_ID_INTEL_SCH_LPC:
    sch.resume_base = 10;
    sch.chip.ngpio = 14;
//
// GPIO[6:0] enabled by default
// GPIO7 is configured by the CMC as SLPIOVR
// Enable GPIO[9:8] core powered gpios explicitly
//
    sch_gpio_reg_set(sch, 8, GEN, 1);
    sch_gpio_reg_set(sch, 9, GEN, 1);
//
// SUS_GPIO[2:0] enabled by default
// Enable SUS_GPIO3 resume powered gpio explicitly
//
    sch_gpio_reg_set(sch, 13, GEN, 1);
    break;
    case PCI_DEVICE_ID_INTEL_ITC_LPC:
    sch.resume_base = 5;
    sch.chip.ngpio = 14;
    break;
    case PCI_DEVICE_ID_INTEL_CENTERTON_ILB:
    sch.resume_base = 21;
    sch.chip.ngpio = 30;
    break;
    case PCI_DEVICE_ID_INTEL_QUARK_X1000_ILB:
    sch.resume_base = 2;
    sch.chip.ngpio = 8;
    break;
    default:
    return -ENODEV;
    }
    girq = &sch.chip.irq;
    gpio_irq_chip_set_chip(girq, &sch_irqchip);
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.parent_handler = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_bad_irq;
// GPE setup is optional
    sch.gpe = GPE0E_GPIO;
    sch.gpe_handler = sch_gpio_gpe_handler;
    ret = sch_gpio_install_gpe_handler(sch);
    if (ret)
    dev_warn(dev, "Can't setup GPE, no IRQ support\n");
    return devm_gpiochip_add_data(dev, &sch.chip, sch);
    }
    static struct platform_driver sch_gpio_driver = {
    .driver = {
    .name = "sch_gpio",
    },
    .probe		= sch_gpio_probe,
    };
    module_platform_driver(sch_gpio_driver);
    MODULE_AUTHOR("Denis Turischev <denis@compulab.co.il>");
    MODULE_DESCRIPTION("GPIO interface for Intel Poulsbo SCH");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:sch_gpio");
