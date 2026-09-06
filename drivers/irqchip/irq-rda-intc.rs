//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-rda-intc.c
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
// RDA8810PL SoC irqchip driver
//
// Copyright RDA Microelectronics Company Limited
// Copyright (c) 2017 Andreas Färber
// Copyright (c) 2018 Manivannan Sadhasivam
//

pub const RDA_INTC_FINALSTATUS: c_uint = 0x00;
pub const RDA_INTC_MASK_SET: c_uint = 0x08;
pub const RDA_INTC_MASK_CLR: c_uint = 0x0c;
pub const RDA_IRQ_MASK_ALL: c_uint = 0xFFFFFFFF;
pub const RDA_NR_IRQS: c_int = 32;
    static void __iomem *rda_intc_base;
    static struct irq_domain *rda_irq_domain;
#[no_mangle]
unsafe extern "C" fn rda_intc_mask_irq(d: *mut irq_data) {
    static void rda_intc_mask_irq(struct irq_data *d)
    {
    writel_relaxed(BIT(d.hwirq), rda_intc_base + RDA_INTC_MASK_CLR);
    }
#[no_mangle]
unsafe extern "C" fn rda_intc_unmask_irq(d: *mut irq_data) {
    static void rda_intc_unmask_irq(struct irq_data *d)
    {
    writel_relaxed(BIT(d.hwirq), rda_intc_base + RDA_INTC_MASK_SET);
    }
#[no_mangle]
unsafe extern "C" fn rda_intc_set_type(data: *mut irq_data, flow_type: c_uint) -> c_int {
    static int rda_intc_set_type(struct irq_data *data, unsigned int flow_type)
    {
// Hardware supports only level triggered interrupts
    if ((flow_type & (IRQF_TRIGGER_HIGH | IRQF_TRIGGER_LOW)) == flow_type)
    return 0;
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn rda_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry rda_handle_irq(struct pt_regs *regs)
    {
    let mut stat: u32 = readl_relaxed(rda_intc_base + RDA_INTC_FINALSTATUS);
    u32 hwirq;
    while (stat) {
    hwirq = __fls(stat);
    generic_handle_domain_irq(rda_irq_domain, hwirq);
    stat &= ~BIT(hwirq);
    }
    }
    static struct irq_chip rda_irq_chip = {
    .name		= "rda-intc",
    .irq_mask	= rda_intc_mask_irq,
    .irq_unmask	= rda_intc_unmask_irq,
    .irq_set_type	= rda_intc_set_type,
    };
    static int rda_irq_map(struct irq_domain *d,
    unsigned int virq, irq_hw_number_t hw)
    {
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &rda_irq_chip, handle_level_irq);
    irq_set_chip_data(virq, d.host_data);
    irq_set_probe(virq);
    return 0;
    }
    static const struct irq_domain_ops rda_irq_domain_ops = {
    .map = rda_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };
    static int __init rda8810_intc_init(struct device_node *node,
    struct device_node *parent)
    {
    rda_intc_base = of_io_request_and_map(node, 0, "rda-intc");
    if (IS_ERR(rda_intc_base))
    return PTR_ERR(rda_intc_base);
// Mask all interrupt sources
    writel_relaxed(RDA_IRQ_MASK_ALL, rda_intc_base + RDA_INTC_MASK_CLR);
    rda_irq_domain = irq_domain_create_linear(&node.fwnode, RDA_NR_IRQS,
    &rda_irq_domain_ops,
    rda_intc_base);
    if (!rda_irq_domain) {
    iounmap(rda_intc_base);
    return -ENOMEM;
    }
    set_handle_irq(rda_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(rda_intc, "rda,8810pl-intc", rda8810_intc_init);
