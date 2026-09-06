//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-ep93xx.c
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
// Generic EP93xx GPIO handling
//
// Copyright (c) 2008 Ryan Mallon
// Copyright (c) 2011 H Hartley Sweeten <hsweeten@visionengravers.com>
//
// Based on code originally from:
// linux/arch/arm/mach-ep93xx/core.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_gpio_irq_chip {
    pub base: *mut void __iomem,
    pub int_unmasked: u8,
    pub int_enabled: u8,
    pub int_type1: u8,
    pub int_type2: u8,
    pub int_debounce: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_gpio_chip {
    pub base: *mut void __iomem,
    pub chip: gpio_generic_chip,
    pub eic: *mut ep93xx_gpio_irq_chip,
}

    static struct ep93xx_gpio_chip *to_ep93xx_gpio_chip(struct gpio_chip *gc)
    {
    return container_of(to_gpio_generic_chip(gc), struct ep93xx_gpio_chip, chip);
    }
    static struct ep93xx_gpio_irq_chip *to_ep93xx_gpio_irq_chip(struct gpio_chip *gc)
    {
    struct ep93xx_gpio_chip *egc = to_ep93xx_gpio_chip(gc);
    return egc.eic;
    }
//
// Interrupt handling for EP93xx on-chip GPIOs
//
pub const EP93XX_INT_TYPE1_OFFSET: c_uint = 0x00;
pub const EP93XX_INT_TYPE2_OFFSET: c_uint = 0x04;
pub const EP93XX_INT_EOI_OFFSET: c_uint = 0x08;
pub const EP93XX_INT_EN_OFFSET: c_uint = 0x0c;
pub const EP93XX_INT_STATUS_OFFSET: c_uint = 0x10;
pub const EP93XX_INT_RAW_STATUS_OFFSET: c_uint = 0x14;
pub const EP93XX_INT_DEBOUNCE_OFFSET: c_uint = 0x18;
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_update_int_params(eic: *mut ep93xx_gpio_irq_chip) {
    static void ep93xx_gpio_update_int_params(struct ep93xx_gpio_irq_chip *eic)
    {
    writeb_relaxed(0, eic.base + EP93XX_INT_EN_OFFSET);
    writeb_relaxed(eic.int_type2,
    eic.base + EP93XX_INT_TYPE2_OFFSET);
    writeb_relaxed(eic.int_type1,
    eic.base + EP93XX_INT_TYPE1_OFFSET);
    writeb_relaxed(eic.int_unmasked & eic.int_enabled,
    eic.base + EP93XX_INT_EN_OFFSET);
    }
    static void ep93xx_gpio_int_debounce(struct gpio_chip *gc,
    unsigned int offset, bool enable)
    {
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut port_mask: c_int = BIT(offset);
    if (enable)
    eic.int_debounce |= port_mask;
    else
    eic.int_debounce &= ~port_mask;
    writeb(eic.int_debounce, eic.base + EP93XX_INT_DEBOUNCE_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_ab_irq_handler(gc: *mut gpio_chip) -> u32 {
    static u32 ep93xx_gpio_ab_irq_handler(struct gpio_chip *gc)
    {
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    unsigned long stat;
    int offset;
    stat = readb(eic.base + EP93XX_INT_STATUS_OFFSET);
    for_each_set_bit(offset, &stat, 8)
    generic_handle_domain_irq(gc.irq.domain, offset);
    return stat;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_ab_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ep93xx_ab_irq_handler(int irq, void *dev_id)
    {
    return IRQ_RETVAL(ep93xx_gpio_ab_irq_handler(dev_id));
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_f_irq_handler(desc: *mut irq_desc) {
    static void ep93xx_gpio_f_irq_handler(struct irq_desc *desc)
    {
    struct irq_chip *irqchip = irq_desc_get_chip(desc);
    struct gpio_chip *gc = irq_desc_get_handler_data(desc);
    struct gpio_irq_chip *gic = &gc.irq;
    let mut parent: c_uint = irq_desc_get_irq(desc);
    unsigned int i;
    chained_irq_enter(irqchip, desc);
    for (i = 0; i < gic.num_parents; i++)
    if (gic.parents[i] == parent)
    break;
    if (i < gic.num_parents)
    generic_handle_domain_irq(gc.irq.domain, i);
    chained_irq_exit(irqchip, desc);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_irq_ack(d: *mut irq_data) {
    static void ep93xx_gpio_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut port_mask: c_int = BIT(irqd_to_hwirq(d));
    if (irqd_get_trigger_type(d) == IRQ_TYPE_EDGE_BOTH) {
    eic.int_type2 ^= port_mask; /* switch edge direction */
    ep93xx_gpio_update_int_params(eic);
    }
    writeb(port_mask, eic.base + EP93XX_INT_EOI_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_irq_mask_ack(d: *mut irq_data) {
    static void ep93xx_gpio_irq_mask_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    let mut port_mask: c_int = BIT(hwirq);
    if (irqd_get_trigger_type(d) == IRQ_TYPE_EDGE_BOTH)
    eic.int_type2 ^= port_mask; /* switch edge direction */
    eic.int_unmasked &= ~port_mask;
    ep93xx_gpio_update_int_params(eic);
    writeb(port_mask, eic.base + EP93XX_INT_EOI_OFFSET);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_irq_mask(d: *mut irq_data) {
    static void ep93xx_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    eic.int_unmasked &= ~BIT(hwirq);
    ep93xx_gpio_update_int_params(eic);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_irq_unmask(d: *mut irq_data) {
    static void ep93xx_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    eic.int_unmasked |= BIT(hwirq);
    ep93xx_gpio_update_int_params(eic);
    }
//
// gpio_int_type1 controls whether the interrupt is level (0) or
// edge (1) triggered, while gpio_int_type2 controls whether it
// triggers on low/falling (0) or high/rising (1).
//
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_irq_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int ep93xx_gpio_irq_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct ep93xx_gpio_irq_chip *eic = to_ep93xx_gpio_irq_chip(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    let mut port_mask: c_int = BIT(hwirq);
    irq_flow_handler_t handler;
    gc.direction_input(gc, hwirq);
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    eic.int_type1 |= port_mask;
    eic.int_type2 |= port_mask;
    handler = handle_edge_irq;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    eic.int_type1 |= port_mask;
    eic.int_type2 &= ~port_mask;
    handler = handle_edge_irq;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    eic.int_type1 &= ~port_mask;
    eic.int_type2 |= port_mask;
    handler = handle_level_irq;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    eic.int_type1 &= ~port_mask;
    eic.int_type2 &= ~port_mask;
    handler = handle_level_irq;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    eic.int_type1 |= port_mask;
// set initial polarity based on current input level
    if (gc.get(gc, hwirq))
    eic.int_type2 &= ~port_mask; /* falling */
    else
    eic.int_type2 |= port_mask; /* rising */
    handler = handle_edge_irq;
    break;
    default:
    return -EINVAL;
    }
    irq_set_handler_locked(d, handler);
    eic.int_enabled |= port_mask;
    ep93xx_gpio_update_int_params(eic);
    return 0;
    }
    static int ep93xx_gpio_set_config(struct gpio_chip *gc, unsigned offset,
    unsigned long config)
    {
    u32 debounce;
    if (pinconf_to_config_param(config) != PIN_CONFIG_INPUT_DEBOUNCE)
    return -ENOTSUPP;
    debounce = pinconf_to_config_argument(config);
    ep93xx_gpio_int_debounce(gc, offset, debounce ? true : false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    static void ep93xx_irq_print_chip(struct irq_data *data, struct seq_file *p)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    seq_puts(p, dev_name(gc.parent));
    }
    static const struct irq_chip gpio_eic_irq_chip = {
    .name			= "ep93xx-gpio-eic",
    .irq_ack		= ep93xx_gpio_irq_ack,
    .irq_mask		= ep93xx_gpio_irq_mask,
    .irq_unmask		= ep93xx_gpio_irq_unmask,
    .irq_mask_ack	= ep93xx_gpio_irq_mask_ack,
    .irq_set_type	= ep93xx_gpio_irq_type,
    .irq_print_chip	= ep93xx_irq_print_chip,
    .flags			= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int ep93xx_setup_irqs(struct platform_device *pdev,
    struct ep93xx_gpio_chip *egc)
    {
    struct gpio_chip *gc = &egc.chip.gc;
    struct device *dev = &pdev.dev;
    struct gpio_irq_chip *girq = &gc.irq;
    int ret, irq, i;
    void __iomem *intr;
    intr = devm_platform_ioremap_resource_byname(pdev, "intr");
    if (IS_ERR(intr))
    return PTR_ERR(intr);
    gc.set_config = ep93xx_gpio_set_config;
    egc.eic = devm_kzalloc(dev, sizeof(*egc.eic), GFP_KERNEL);
    if (!egc.eic)
    return -ENOMEM;
    egc.eic.base = intr;
    gpio_irq_chip_set_chip(girq, &gpio_eic_irq_chip);
    girq.num_parents = platform_irq_count(pdev);
    if (girq.num_parents == 0)
    return -EINVAL;
    girq.parents = devm_kcalloc(dev, girq.num_parents, sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    if (girq.num_parents == 1) { /* A/B irqchips */
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, ep93xx_ab_irq_handler,
    IRQF_SHARED, gc.label, gc);
    if (ret)
    return dev_err_probe(dev, ret, "requesting IRQ: %d\n", irq);
    girq.parents[0] = irq;
    } else { /* F irqchip */
    girq.parent_handler = ep93xx_gpio_f_irq_handler;
    for (i = 0; i < girq.num_parents; i++) {
    irq = platform_get_irq_optional(pdev, i);
    if (irq < 0)
    continue;
    girq.parents[i] = irq;
    }
    girq.map = girq.parents;
    }
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_bad_irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int ep93xx_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct ep93xx_gpio_chip *egc;
    struct gpio_chip *gc;
    void __iomem *data;
    void __iomem *dir;
    int ret;
    egc = devm_kzalloc(&pdev.dev, sizeof(*egc), GFP_KERNEL);
    if (!egc)
    return -ENOMEM;
    data = devm_platform_ioremap_resource_byname(pdev, "data");
    if (IS_ERR(data))
    return PTR_ERR(data);
    dir = devm_platform_ioremap_resource_byname(pdev, "dir");
    if (IS_ERR(dir))
    return PTR_ERR(dir);
    gc = &egc.chip.gc;
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 1,
    .dat = data,
    .dirout = dir,
    };
    ret = gpio_generic_chip_init(&egc.chip, &config);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "unable to init generic GPIO\n");
    gc.label = dev_name(&pdev.dev);
    if (platform_irq_count(pdev) > 0) {
    dev_dbg(&pdev.dev, "setting up irqs for %s\n", dev_name(&pdev.dev));
    ret = ep93xx_setup_irqs(pdev, egc);
    if (ret)
    dev_err_probe(&pdev.dev, ret, "setup irqs failed");
    }
    return devm_gpiochip_add_data(&pdev.dev, gc, egc);
    }
    static const struct of_device_id ep93xx_gpio_match[] = {
    { .compatible = "cirrus,ep9301-gpio" },
    { /* sentinel */ }
    };
    static struct platform_driver ep93xx_gpio_driver = {
    .driver		= {
    .name	= "gpio-ep93xx",
    .of_match_table = ep93xx_gpio_match,
    },
    .probe		= ep93xx_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn ep93xx_gpio_init() -> int __init {
    static int __init ep93xx_gpio_init(void)
    {
    return platform_driver_register(&ep93xx_gpio_driver);
    }
    postcore_initcall(ep93xx_gpio_init);
    MODULE_AUTHOR("Ryan Mallon <ryan@bluewatersys.com> "
    "H Hartley Sweeten <hsweeten@visionengravers.com>");
    MODULE_DESCRIPTION("EP93XX GPIO driver");
    MODULE_LICENSE("GPL");
