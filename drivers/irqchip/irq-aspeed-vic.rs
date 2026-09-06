//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-aspeed-vic.c
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
// Copyright (C) 2015 - Ben Herrenschmidt, IBM Corp.
//
// Driver for Aspeed "new" VIC as found in SoC generation 3 and later
//
// Based on irq-vic.c:
//
// Copyright (C) 1999 - 2003 ARM Limited
// Copyright (C) 2000 Deep Blue Solutions Ltd
//

// These definitions correspond to the "new mapping" of the
// register set that interleaves "high" and "low". The offsets
// below are for the "low" register, add 4 to get to the high one
//
pub const AVIC_IRQ_STATUS: c_uint = 0x00;
pub const AVIC_FIQ_STATUS: c_uint = 0x08;
pub const AVIC_RAW_STATUS: c_uint = 0x10;
pub const AVIC_INT_SELECT: c_uint = 0x18;
pub const AVIC_INT_ENABLE: c_uint = 0x20;
pub const AVIC_INT_ENABLE_CLR: c_uint = 0x28;
pub const AVIC_INT_TRIGGER: c_uint = 0x30;
pub const AVIC_INT_TRIGGER_CLR: c_uint = 0x38;
pub const AVIC_INT_SENSE: c_uint = 0x40;
pub const AVIC_INT_DUAL_EDGE: c_uint = 0x48;
pub const AVIC_INT_EVENT: c_uint = 0x50;
pub const AVIC_EDGE_CLR: c_uint = 0x58;
pub const AVIC_EDGE_STATUS: c_uint = 0x60;
pub const NUM_IRQS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_vic {
    pub base: *mut void __iomem,
    pub edge_sources: [u32; 2],
    pub dom: *mut irq_domain,
}

    static struct aspeed_vic *system_avic;
#[no_mangle]
unsafe extern "C" fn vic_init_hw(vic: *mut aspeed_vic) {
    static void vic_init_hw(struct aspeed_vic *vic)
    {
    u32 sense;
// Disable all interrupts
    writel(0xffffffff, vic.base + AVIC_INT_ENABLE_CLR);
    writel(0xffffffff, vic.base + AVIC_INT_ENABLE_CLR + 4);
// Make sure no soft trigger is on
    writel(0xffffffff, vic.base + AVIC_INT_TRIGGER_CLR);
    writel(0xffffffff, vic.base + AVIC_INT_TRIGGER_CLR + 4);
// Set everything to be IRQ
    writel(0, vic.base + AVIC_INT_SELECT);
    writel(0, vic.base + AVIC_INT_SELECT + 4);
// Some interrupts have a programmable high/low level trigger
// (4 GPIO direct inputs), for now we assume this was configured
// by firmware. We read which ones are edge now.
//
    sense = readl(vic.base + AVIC_INT_SENSE);
    vic.edge_sources[0] = ~sense;
    sense = readl(vic.base + AVIC_INT_SENSE + 4);
    vic.edge_sources[1] = ~sense;
// Clear edge detection latches
    writel(0xffffffff, vic.base + AVIC_EDGE_CLR);
    writel(0xffffffff, vic.base + AVIC_EDGE_CLR + 4);
    }
#[no_mangle]
unsafe extern "C" fn avic_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry avic_handle_irq(struct pt_regs *regs)
    {
    struct aspeed_vic *vic = system_avic;
    u32 stat, irq;
    for (;;) {
    irq = 0;
    stat = readl_relaxed(vic.base + AVIC_IRQ_STATUS);
    if (!stat) {
    stat = readl_relaxed(vic.base + AVIC_IRQ_STATUS + 4);
    irq = 32;
    }
    if (stat == 0)
    break;
    irq += ffs(stat) - 1;
    generic_handle_domain_irq(vic.dom, irq);
    }
    }
#[no_mangle]
unsafe extern "C" fn avic_ack_irq(d: *mut irq_data) {
    static void avic_ack_irq(struct irq_data *d)
    {
    struct aspeed_vic *vic = irq_data_get_irq_chip_data(d);
    let mut sidx: c_uint = d.hwirq >> 5;
    let mut sbit: c_uint = 1u << (d.hwirq & 0x1f);
// Clear edge latch for edge interrupts, nop for level
    if (vic.edge_sources[sidx] & sbit)
    writel(sbit, vic.base + AVIC_EDGE_CLR + sidx * 4);
    }
#[no_mangle]
unsafe extern "C" fn avic_mask_irq(d: *mut irq_data) {
    static void avic_mask_irq(struct irq_data *d)
    {
    struct aspeed_vic *vic = irq_data_get_irq_chip_data(d);
    let mut sidx: c_uint = d.hwirq >> 5;
    let mut sbit: c_uint = 1u << (d.hwirq & 0x1f);
    writel(sbit, vic.base + AVIC_INT_ENABLE_CLR + sidx * 4);
    }
#[no_mangle]
unsafe extern "C" fn avic_unmask_irq(d: *mut irq_data) {
    static void avic_unmask_irq(struct irq_data *d)
    {
    struct aspeed_vic *vic = irq_data_get_irq_chip_data(d);
    let mut sidx: c_uint = d.hwirq >> 5;
    let mut sbit: c_uint = 1u << (d.hwirq & 0x1f);
    writel(sbit, vic.base + AVIC_INT_ENABLE + sidx * 4);
    }
// For level irq, faster than going through a nop "ack" and mask
#[no_mangle]
unsafe extern "C" fn avic_mask_ack_irq(d: *mut irq_data) {
    static void avic_mask_ack_irq(struct irq_data *d)
    {
    struct aspeed_vic *vic = irq_data_get_irq_chip_data(d);
    let mut sidx: c_uint = d.hwirq >> 5;
    let mut sbit: c_uint = 1u << (d.hwirq & 0x1f);
// First mask
    writel(sbit, vic.base + AVIC_INT_ENABLE_CLR + sidx * 4);
// Then clear edge latch for edge interrupts
    if (vic.edge_sources[sidx] & sbit)
    writel(sbit, vic.base + AVIC_EDGE_CLR + sidx * 4);
    }
    static struct irq_chip avic_chip = {
    .name		= "AVIC",
    .irq_ack	= avic_ack_irq,
    .irq_mask	= avic_mask_irq,
    .irq_unmask	= avic_unmask_irq,
    .irq_mask_ack	= avic_mask_ack_irq,
    };
    static int avic_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hwirq)
    {
    struct aspeed_vic *vic = d.host_data;
    let mut sidx: c_uint = hwirq >> 5;
    let mut sbit: c_uint = 1u << (hwirq & 0x1f);
// Check if interrupt exists
    if (sidx > 1)
    return -EPERM;
    if (vic.edge_sources[sidx] & sbit)
    irq_set_chip_and_handler(irq, &avic_chip, handle_edge_irq);
    else
    irq_set_chip_and_handler(irq, &avic_chip, handle_level_irq);
    irq_set_chip_data(irq, vic);
    irq_set_probe(irq);
    return 0;
    }
    static const struct irq_domain_ops avic_dom_ops = {
    .map = avic_map,
    .xlate = irq_domain_xlate_onetwocell,
    };
    static int __init avic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    void __iomem *regs;
    struct aspeed_vic *vic;
    if (WARN(parent, "non-root Aspeed VIC not supported"))
    return -EINVAL;
    if (WARN(system_avic, "duplicate Aspeed VIC not supported"))
    return -EINVAL;
    regs = of_iomap(node, 0);
    if (WARN_ON(!regs))
    return -EIO;
    vic = kzalloc_obj(struct aspeed_vic);
    if (WARN_ON(!vic)) {
    iounmap(regs);
    return -ENOMEM;
    }
    vic.base = regs;
// Initialize sources, all masked
    vic_init_hw(vic);
// Ready to receive interrupts
    system_avic = vic;
    set_handle_irq(avic_handle_irq);
// Register our domain
    vic.dom = irq_domain_create_simple(of_fwnode_handle(node), NUM_IRQS, 0,
    &avic_dom_ops, vic);
    return 0;
    }
    IRQCHIP_DECLARE(ast2400_vic, "aspeed,ast2400-vic", avic_of_init);
    IRQCHIP_DECLARE(ast2500_vic, "aspeed,ast2500-vic", avic_of_init);
