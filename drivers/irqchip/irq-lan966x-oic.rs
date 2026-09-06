//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-lan966x-oic.c
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
// Driver for the Microchip LAN966x outbound interrupt controller
//
// Copyright (c) 2024 Technology Inc. and its subsidiaries.
//
// Authors:
// Horatiu Vultur <horatiu.vultur@microchip.com>
// Clément Léger <clement.leger@bootlin.com>
// Herve Codina <herve.codina@bootlin.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_oic_chip_regs {
    pub reg_off_ena_set: c_int,
    pub reg_off_ena_clr: c_int,
    pub reg_off_sticky: c_int,
    pub reg_off_ident: c_int,
    pub reg_off_map: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_oic_data {
    pub regs: *mut void __iomem,
    pub irq: c_int,
}

pub const LAN966X_OIC_NR_IRQ: c_int = 86;
// Interrupt sticky status
pub const LAN966X_OIC_INTR_STICKY: c_uint = 0x30;
pub const LAN966X_OIC_INTR_STICKY1: c_uint = 0x34;
pub const LAN966X_OIC_INTR_STICKY2: c_uint = 0x38;
// Interrupt enable
pub const LAN966X_OIC_INTR_ENA: c_uint = 0x48;
pub const LAN966X_OIC_INTR_ENA1: c_uint = 0x4c;
pub const LAN966X_OIC_INTR_ENA2: c_uint = 0x50;
// Atomic clear of interrupt enable
pub const LAN966X_OIC_INTR_ENA_CLR: c_uint = 0x54;
pub const LAN966X_OIC_INTR_ENA_CLR1: c_uint = 0x58;
pub const LAN966X_OIC_INTR_ENA_CLR2: c_uint = 0x5c;
// Atomic set of interrupt
pub const LAN966X_OIC_INTR_ENA_SET: c_uint = 0x60;
pub const LAN966X_OIC_INTR_ENA_SET1: c_uint = 0x64;
pub const LAN966X_OIC_INTR_ENA_SET2: c_uint = 0x68;
// Mapping of source to destination interrupts (_n = 0..8)

// Currently active interrupt sources per destination (_n = 0..8)

#[no_mangle]
unsafe extern "C" fn lan966x_oic_irq_startup(data: *mut irq_data) -> c_uint {
    static unsigned int lan966x_oic_irq_startup(struct irq_data *data)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(data);
    struct irq_chip_type *ct = irq_data_get_chip_type(data);
    struct lan966x_oic_chip_regs *chip_regs = gc.private;
    u32 map;
    scoped_guard (raw_spinlock, &gc.lock) {
// Map the source interrupt to the destination
    map = irq_reg_readl(gc, chip_regs.reg_off_map);
    map |= data.mask;
    irq_reg_writel(gc, map, chip_regs.reg_off_map);
    }
    ct.chip.irq_ack(data);
    ct.chip.irq_unmask(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_irq_shutdown(data: *mut irq_data) {
    static void lan966x_oic_irq_shutdown(struct irq_data *data)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(data);
    struct irq_chip_type *ct = irq_data_get_chip_type(data);
    struct lan966x_oic_chip_regs *chip_regs = gc.private;
    u32 map;
    ct.chip.irq_mask(data);
    guard(raw_spinlock)(&gc.lock);
// Unmap the interrupt
    map = irq_reg_readl(gc, chip_regs.reg_off_map);
    map &= ~data.mask;
    irq_reg_writel(gc, map, chip_regs.reg_off_map);
    }
    static int lan966x_oic_irq_set_type(struct irq_data *data,
    unsigned int flow_type)
    {
    if (flow_type != IRQ_TYPE_LEVEL_HIGH) {
    pr_err("lan966x oic doesn't support flow type %d\n", flow_type);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_irq_handler_domain(d: *mut irq_domain, first_irq: u32) {
    static void lan966x_oic_irq_handler_domain(struct irq_domain *d, u32 first_irq)
    {
    struct irq_chip_generic *gc = irq_get_domain_generic_chip(d, first_irq);
    struct lan966x_oic_chip_regs *chip_regs = gc.private;
    unsigned long ident;
    unsigned int hwirq;
    ident = irq_reg_readl(gc, chip_regs.reg_off_ident);
    if (!ident)
    return;
    for_each_set_bit(hwirq, &ident, 32)
    generic_handle_domain_irq(d, hwirq + first_irq);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_irq_handler(desc: *mut irq_desc) {
    static void lan966x_oic_irq_handler(struct irq_desc *desc)
    {
    struct irq_domain *d = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    chained_irq_enter(chip, desc);
    lan966x_oic_irq_handler_domain(d, 0);
    lan966x_oic_irq_handler_domain(d, 32);
    lan966x_oic_irq_handler_domain(d, 64);
    chained_irq_exit(chip, desc);
    }
    static struct lan966x_oic_chip_regs lan966x_oic_chip_regs[3] = {
    {
    .reg_off_ena_set = LAN966X_OIC_INTR_ENA_SET,
    .reg_off_ena_clr = LAN966X_OIC_INTR_ENA_CLR,
    .reg_off_sticky  = LAN966X_OIC_INTR_STICKY,
    .reg_off_ident   = LAN966X_OIC_DST_INTR_IDENT(0),
    .reg_off_map     = LAN966X_OIC_DST_INTR_MAP(0),
    }, {
    .reg_off_ena_set = LAN966X_OIC_INTR_ENA_SET1,
    .reg_off_ena_clr = LAN966X_OIC_INTR_ENA_CLR1,
    .reg_off_sticky  = LAN966X_OIC_INTR_STICKY1,
    .reg_off_ident   = LAN966X_OIC_DST_INTR_IDENT1(0),
    .reg_off_map     = LAN966X_OIC_DST_INTR_MAP1(0),
    }, {
    .reg_off_ena_set = LAN966X_OIC_INTR_ENA_SET2,
    .reg_off_ena_clr = LAN966X_OIC_INTR_ENA_CLR2,
    .reg_off_sticky  = LAN966X_OIC_INTR_STICKY2,
    .reg_off_ident   = LAN966X_OIC_DST_INTR_IDENT2(0),
    .reg_off_map     = LAN966X_OIC_DST_INTR_MAP2(0),
    }
    };
#[no_mangle]
unsafe extern "C" fn lan966x_oic_chip_init(gc: *mut irq_chip_generic) -> c_int {
    static int lan966x_oic_chip_init(struct irq_chip_generic *gc)
    {
    struct lan966x_oic_data *lan966x_oic = gc.domain.host_data;
    struct lan966x_oic_chip_regs *chip_regs;
    chip_regs = &lan966x_oic_chip_regs[gc.irq_base / 32];
    gc.reg_base = lan966x_oic.regs;
    gc.chip_types[0].regs.enable = chip_regs.reg_off_ena_set;
    gc.chip_types[0].regs.disable = chip_regs.reg_off_ena_clr;
    gc.chip_types[0].regs.ack = chip_regs.reg_off_sticky;
    gc.chip_types[0].chip.irq_startup = lan966x_oic_irq_startup;
    gc.chip_types[0].chip.irq_shutdown = lan966x_oic_irq_shutdown;
    gc.chip_types[0].chip.irq_set_type = lan966x_oic_irq_set_type;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_disable_reg;
    gc.chip_types[0].chip.irq_unmask = irq_gc_unmask_enable_reg;
    gc.chip_types[0].chip.irq_ack = irq_gc_ack_set_bit;
    gc.private = chip_regs;
// Disable all interrupts handled by this chip
    irq_reg_writel(gc, ~0U, chip_regs.reg_off_ena_clr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_chip_exit(gc: *mut irq_chip_generic) {
    static void lan966x_oic_chip_exit(struct irq_chip_generic *gc)
    {
// Disable and ack all interrupts handled by this chip
    irq_reg_writel(gc, ~0U, gc.chip_types[0].regs.disable);
    irq_reg_writel(gc, ~0U, gc.chip_types[0].regs.ack);
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_domain_init(d: *mut irq_domain) -> c_int {
    static int lan966x_oic_domain_init(struct irq_domain *d)
    {
    struct lan966x_oic_data *lan966x_oic = d.host_data;
    irq_set_chained_handler_and_data(lan966x_oic.irq, lan966x_oic_irq_handler, d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_domain_exit(d: *mut irq_domain) {
    static void lan966x_oic_domain_exit(struct irq_domain *d)
    {
    struct lan966x_oic_data *lan966x_oic = d.host_data;
    irq_set_chained_handler_and_data(lan966x_oic.irq, core::ptr::null_mut(), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn lan966x_oic_probe(pdev: *mut platform_device) -> c_int {
    static int lan966x_oic_probe(struct platform_device *pdev)
    {
    struct irq_domain_chip_generic_info dgc_info = {
    .name		= "lan966x-oic",
    .handler	= handle_level_irq,
    .irqs_per_chip	= 32,
    .num_ct		= 1,
    .init		= lan966x_oic_chip_init,
    .exit		= lan966x_oic_chip_exit,
    };
    struct irq_domain_info d_info = {
    .fwnode		= of_fwnode_handle(pdev.dev.of_node),
    .domain_flags	= IRQ_DOMAIN_FLAG_DESTROY_GC,
    .size		= LAN966X_OIC_NR_IRQ,
    .hwirq_max	= LAN966X_OIC_NR_IRQ,
    .ops		= &irq_generic_chip_ops,
    .dgc_info	= &dgc_info,
    .init		= lan966x_oic_domain_init,
    .exit		= lan966x_oic_domain_exit,
    };
    struct lan966x_oic_data *lan966x_oic;
    struct device *dev = &pdev.dev;
    struct irq_domain *domain;
    lan966x_oic = devm_kmalloc(dev, sizeof(*lan966x_oic), GFP_KERNEL);
    if (!lan966x_oic)
    return -ENOMEM;
    lan966x_oic.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lan966x_oic.regs))
    return dev_err_probe(dev, PTR_ERR(lan966x_oic.regs),
    "failed to map resource\n");
    lan966x_oic.irq = platform_get_irq(pdev, 0);
    if (lan966x_oic.irq < 0)
    return dev_err_probe(dev, lan966x_oic.irq, "failed to get the IRQ\n");
    d_info.host_data = lan966x_oic;
    domain = devm_irq_domain_instantiate(dev, &d_info);
    if (IS_ERR(domain))
    return dev_err_probe(dev, PTR_ERR(domain),
    "failed to instantiate the IRQ domain\n");
    return 0;
    }
    static const struct of_device_id lan966x_oic_of_match[] = {
    { .compatible = "microchip,lan966x-oic" },
    {} /* sentinel */
    };
    MODULE_DEVICE_TABLE(of, lan966x_oic_of_match);
    static struct platform_driver lan966x_oic_driver = {
    .probe = lan966x_oic_probe,
    .driver = {
    .name = "lan966x-oic",
    .of_match_table = lan966x_oic_of_match,
    },
    };
    module_platform_driver(lan966x_oic_driver);
    MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
    MODULE_DESCRIPTION("Microchip LAN966x OIC driver");
    MODULE_LICENSE("GPL");
