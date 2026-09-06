//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-em.c
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
// Emma Mobile GPIO Support - GIO
//
// Copyright (C) 2012 Magnus Damm
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em_gio_priv {
    pub base0: *mut void __iomem,
    pub base1: *mut void __iomem,
    pub sense_lock: spinlock_t,
    pub pdev: *mut platform_device,
    pub gpio_chip: gpio_chip,
    pub irq_chip: irq_chip,
    pub irq_domain: *mut irq_domain,
}

pub const GIO_E1: c_uint = 0x00;
pub const GIO_E0: c_uint = 0x04;
pub const GIO_EM: c_uint = 0x04;
pub const GIO_OL: c_uint = 0x08;
pub const GIO_OH: c_uint = 0x0c;
pub const GIO_I: c_uint = 0x10;
pub const GIO_IIA: c_uint = 0x14;
pub const GIO_IEN: c_uint = 0x18;
pub const GIO_IDS: c_uint = 0x1c;
pub const GIO_IIM: c_uint = 0x1c;
pub const GIO_RAW: c_uint = 0x20;
pub const GIO_MST: c_uint = 0x24;
pub const GIO_IIR: c_uint = 0x28;
pub const GIO_IDT0: c_uint = 0x40;
pub const GIO_IDT1: c_uint = 0x44;
pub const GIO_IDT2: c_uint = 0x48;
pub const GIO_IDT3: c_uint = 0x4c;
pub const GIO_RAWBL: c_uint = 0x50;
pub const GIO_RAWBH: c_uint = 0x54;
pub const GIO_IRBL: c_uint = 0x58;
pub const GIO_IRBH: c_uint = 0x5c;

#[no_mangle]
pub unsafe extern "C" fn em_gio_read(p: *mut em_gio_priv, offs: c_int) -> c_ulong {
    static inline unsigned long em_gio_read(struct em_gio_priv *p, int offs)
    {
    if (offs < GIO_IDT0)
    return ioread32(p.base0 + offs);
    else
    return ioread32(p.base1 + (offs - GIO_IDT0));
    }
    static inline void em_gio_write(struct em_gio_priv *p, int offs,
    unsigned long value)
    {
    if (offs < GIO_IDT0)
    iowrite32(value, p.base0 + offs);
    else
    iowrite32(value, p.base1 + (offs - GIO_IDT0));
    }
#[no_mangle]
unsafe extern "C" fn em_gio_irq_disable(d: *mut irq_data) {
    static void em_gio_irq_disable(struct irq_data *d)
    {
    struct em_gio_priv *p = irq_data_get_irq_chip_data(d);
    em_gio_write(p, GIO_IDS, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn em_gio_irq_enable(d: *mut irq_data) {
    static void em_gio_irq_enable(struct irq_data *d)
    {
    struct em_gio_priv *p = irq_data_get_irq_chip_data(d);
    em_gio_write(p, GIO_IEN, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn em_gio_irq_reqres(d: *mut irq_data) -> c_int {
    static int em_gio_irq_reqres(struct irq_data *d)
    {
    struct em_gio_priv *p = irq_data_get_irq_chip_data(d);
    int ret;
    ret = gpiochip_lock_as_irq(&p.gpio_chip, irqd_to_hwirq(d));
    if (ret) {
    dev_err(p.gpio_chip.parent,
    "unable to lock HW IRQ %lu for IRQ\n",
    irqd_to_hwirq(d));
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_gio_irq_relres(d: *mut irq_data) {
    static void em_gio_irq_relres(struct irq_data *d)
    {
    struct em_gio_priv *p = irq_data_get_irq_chip_data(d);
    gpiochip_unlock_as_irq(&p.gpio_chip, irqd_to_hwirq(d));
    }

    static unsigned char em_gio_sense_table[IRQ_TYPE_SENSE_MASK + 1] = {
    [IRQ_TYPE_EDGE_RISING] = GIO_ASYNC(0x00),
    [IRQ_TYPE_EDGE_FALLING] = GIO_ASYNC(0x01),
    [IRQ_TYPE_LEVEL_HIGH] = GIO_ASYNC(0x02),
    [IRQ_TYPE_LEVEL_LOW] = GIO_ASYNC(0x03),
    [IRQ_TYPE_EDGE_BOTH] = GIO_ASYNC(0x04),
    };
#[no_mangle]
unsafe extern "C" fn em_gio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int em_gio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    let mut value: c_uchar = em_gio_sense_table[type & IRQ_TYPE_SENSE_MASK];
    struct em_gio_priv *p = irq_data_get_irq_chip_data(d);
    unsigned int reg, offset, shift;
    unsigned long flags;
    unsigned long tmp;
    if (!value)
    return -EINVAL;
    offset = irqd_to_hwirq(d);
    pr_debug("gio: sense irq = %d, mode = %d\n", offset, value);
// 8 x 4 bit fields in 4 IDT registers
    reg = GIO_IDT(offset >> 3);
    shift = (offset & 0x07) << 4;
    spin_lock_irqsave(&p.sense_lock, flags);
// disable the interrupt in IIA
    tmp = em_gio_read(p, GIO_IIA);
    tmp &= ~BIT(offset);
    em_gio_write(p, GIO_IIA, tmp);
// change the sense setting in IDT
    tmp = em_gio_read(p, reg);
    tmp &= ~(0xf << shift);
    tmp |= value << shift;
    em_gio_write(p, reg, tmp);
// clear pending interrupts
    em_gio_write(p, GIO_IIR, BIT(offset));
// enable the interrupt in IIA
    tmp = em_gio_read(p, GIO_IIA);
    tmp |= BIT(offset);
    em_gio_write(p, GIO_IIA, tmp);
    spin_unlock_irqrestore(&p.sense_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_gio_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t em_gio_irq_handler(int irq, void *dev_id)
    {
    struct em_gio_priv *p = dev_id;
    unsigned long pending;
    unsigned int offset, irqs_handled = 0;
    while ((pending = em_gio_read(p, GIO_MST))) {
    offset = __ffs(pending);
    em_gio_write(p, GIO_IIR, BIT(offset));
    generic_handle_domain_irq(p.irq_domain, offset);
    irqs_handled++;
    }
    return irqs_handled ? IRQ_HANDLED : IRQ_NONE;
    }
    static inline struct em_gio_priv *gpio_to_priv(struct gpio_chip *chip)
    {
    return gpiochip_get_data(chip);
    }
#[no_mangle]
unsafe extern "C" fn em_gio_direction_input(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int em_gio_direction_input(struct gpio_chip *chip, unsigned offset)
    {
    em_gio_write(gpio_to_priv(chip), GIO_E0, BIT(offset));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_gio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int em_gio_get(struct gpio_chip *chip, unsigned offset)
    {
    return !!(em_gio_read(gpio_to_priv(chip), GIO_I) & BIT(offset));
    }
    static void __em_gio_set(struct gpio_chip *chip, unsigned int reg,
    unsigned shift, int value)
    {
// upper 16 bits contains mask and lower 16 actual value
    em_gio_write(gpio_to_priv(chip), reg,
    (BIT(shift + 16)) | (value << shift));
    }
#[no_mangle]
unsafe extern "C" fn em_gio_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int em_gio_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
// output is split into two registers
    if (offset < 16)
    __em_gio_set(chip, GIO_OL, offset, value);
    else
    __em_gio_set(chip, GIO_OH, offset - 16, value);
    return 0;
    }
    static int em_gio_direction_output(struct gpio_chip *chip, unsigned offset,
    int value)
    {
// write GPIO value to output before selecting output mode of pin
    em_gio_set(chip, offset, value);
    em_gio_write(gpio_to_priv(chip), GIO_E1, BIT(offset));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn em_gio_to_irq(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int em_gio_to_irq(struct gpio_chip *chip, unsigned offset)
    {
    return irq_create_mapping(gpio_to_priv(chip).irq_domain, offset);
    }
#[no_mangle]
unsafe extern "C" fn em_gio_free(chip: *mut gpio_chip, offset: unsigned) {
    static void em_gio_free(struct gpio_chip *chip, unsigned offset)
    {
    pinctrl_gpio_free(chip, offset);
// Set the GPIO as an input to ensure that the next GPIO request won't
// drive the GPIO pin as an output.
//
    em_gio_direction_input(chip, offset);
    }
    static int em_gio_irq_domain_map(struct irq_domain *h, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct em_gio_priv *p = h.host_data;
    pr_debug("gio: map hw irq = %d, irq = %d\n", (int)hwirq, irq);
    irq_set_chip_data(irq, h.host_data);
    irq_set_chip_and_handler(irq, &p.irq_chip, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops em_gio_irq_domain_ops = {
    .map	= em_gio_irq_domain_map,
    .xlate	= irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn em_gio_irq_domain_remove(data: *mut c_void) {
    static void em_gio_irq_domain_remove(void *data)
    {
    struct irq_domain *domain = data;
    irq_domain_remove(domain);
    }
#[no_mangle]
unsafe extern "C" fn em_gio_probe(pdev: *mut platform_device) -> c_int {
    static int em_gio_probe(struct platform_device *pdev)
    {
    struct em_gio_priv *p;
    struct gpio_chip *gpio_chip;
    struct irq_chip *irq_chip;
    struct device *dev = &pdev.dev;
    const char *name = dev_name(dev);
    unsigned int ngpios;
    int irq[2], ret;
    p = devm_kzalloc(dev, sizeof(*p), GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    p.pdev = pdev;
    platform_set_drvdata(pdev, p);
    spin_lock_init(&p.sense_lock);
    irq[0] = platform_get_irq(pdev, 0);
    if (irq[0] < 0)
    return irq[0];
    irq[1] = platform_get_irq(pdev, 1);
    if (irq[1] < 0)
    return irq[1];
    p.base0 = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(p.base0))
    return PTR_ERR(p.base0);
    p.base1 = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(p.base1))
    return PTR_ERR(p.base1);
    if (of_property_read_u32(dev.of_node, "ngpios", &ngpios)) {
    dev_err(dev, "Missing ngpios OF property\n");
    return -EINVAL;
    }
    gpio_chip = &p.gpio_chip;
    gpio_chip.direction_input = em_gio_direction_input;
    gpio_chip.get = em_gio_get;
    gpio_chip.direction_output = em_gio_direction_output;
    gpio_chip.set = em_gio_set;
    gpio_chip.to_irq = em_gio_to_irq;
    gpio_chip.request = pinctrl_gpio_request;
    gpio_chip.free = em_gio_free;
    gpio_chip.label = name;
    gpio_chip.parent = dev;
    gpio_chip.owner = THIS_MODULE;
    gpio_chip.base = -1;
    gpio_chip.ngpio = ngpios;
    irq_chip = &p.irq_chip;
    irq_chip.name = "gpio-em";
    irq_chip.irq_mask = em_gio_irq_disable;
    irq_chip.irq_unmask = em_gio_irq_enable;
    irq_chip.irq_set_type = em_gio_irq_set_type;
    irq_chip.irq_request_resources = em_gio_irq_reqres;
    irq_chip.irq_release_resources = em_gio_irq_relres;
    irq_chip.flags	= IRQCHIP_SKIP_SET_WAKE | IRQCHIP_MASK_ON_SUSPEND;
    p.irq_domain = irq_domain_create_simple(dev_fwnode(dev), ngpios, 0,
    &em_gio_irq_domain_ops, p);
    if (!p.irq_domain) {
    dev_err(dev, "cannot initialize irq domain\n");
    return -ENXIO;
    }
    ret = devm_add_action_or_reset(dev, em_gio_irq_domain_remove,
    p.irq_domain);
    if (ret)
    return ret;
    if (devm_request_irq(dev, irq[0], em_gio_irq_handler, 0, name, p)) {
    dev_err(dev, "failed to request low IRQ\n");
    return -ENOENT;
    }
    if (devm_request_irq(dev, irq[1], em_gio_irq_handler, 0, name, p)) {
    dev_err(dev, "failed to request high IRQ\n");
    return -ENOENT;
    }
    ret = devm_gpiochip_add_data(dev, gpio_chip, p);
    if (ret) {
    dev_err(dev, "failed to add GPIO controller\n");
    return ret;
    }
    return 0;
    }
    static const struct of_device_id em_gio_dt_ids[] = {
    { .compatible = "renesas,em-gio", },
    {},
    };
    MODULE_DEVICE_TABLE(of, em_gio_dt_ids);
    static struct platform_driver em_gio_device_driver = {
    .probe		= em_gio_probe,
    .driver		= {
    .name	= "em_gio",
    .of_match_table = em_gio_dt_ids,
    }
    };
#[no_mangle]
unsafe extern "C" fn em_gio_init() -> int __init {
    static int __init em_gio_init(void)
    {
    return platform_driver_register(&em_gio_device_driver);
    }
    postcore_initcall(em_gio_init);
#[no_mangle]
unsafe extern "C" fn em_gio_exit() -> void __exit {
    static void __exit em_gio_exit(void)
    {
    platform_driver_unregister(&em_gio_device_driver);
    }
    module_exit(em_gio_exit);
    MODULE_AUTHOR("Magnus Damm");
    MODULE_DESCRIPTION("Renesas Emma Mobile GIO Driver");
    MODULE_LICENSE("GPL v2");
