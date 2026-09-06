//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-wpcm450-aic.c
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
// Copyright 2021 Jonathan Neuschäfer

pub const AIC_GEN: c_uint = 0x84	/* Interrupt group enable control register */;
pub const AIC_GRSR: c_uint = 0x88	/* Interrupt group raw status register */;
pub const AIC_IRSR: c_uint = 0x100	/* Interrupt raw status register */;
pub const AIC_IASR: c_uint = 0x104	/* Interrupt active status register */;
pub const AIC_ISR: c_uint = 0x108	/* Interrupt status register */;
pub const AIC_IPER: c_uint = 0x10c	/* Interrupt priority encoding register */;
pub const AIC_ISNR: c_uint = 0x110	/* Interrupt source number register */;
pub const AIC_IMR: c_uint = 0x114	/* Interrupt mask register */;
pub const AIC_OISR: c_uint = 0x118	/* Output interrupt status register */;
pub const AIC_MECR: c_uint = 0x120	/* Mask enable command register */;
pub const AIC_MDCR: c_uint = 0x124	/* Mask disable command register */;
pub const AIC_SSCR: c_uint = 0x128	/* Source set command register */;
pub const AIC_SCCR: c_uint = 0x12c	/* Source clear command register */;
pub const AIC_EOSCR: c_uint = 0x130	/* End of service command register */;

pub const AIC_SCR_PRIORITY_MASK: c_uint = 0x7;
pub const AIC_NUM_IRQS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpcm450_aic {
    pub regs: *mut void __iomem,
    pub domain: *mut irq_domain,
}

    static struct wpcm450_aic *aic;
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_init_hw() {
    static void wpcm450_aic_init_hw(void)
    {
    int i;
// Disable (mask) all interrupts
    writel(0xffffffff, aic.regs + AIC_MDCR);
//
// Make sure the interrupt controller is ready to serve new interrupts.
// Reading from IPER indicates that the nIRQ signal may be deasserted,
// and writing to EOSCR indicates that interrupt handling has finished.
//
    readl(aic.regs + AIC_IPER);
    writel(0, aic.regs + AIC_EOSCR);
// Initialize trigger mode and priority of each interrupt source
    for (i = 0; i < AIC_NUM_IRQS; i++)
    writel(AIC_SCR_SRCTYPE_HIGH_LEVEL | AIC_SCR_PRIORITY(7),
    aic.regs + AIC_SCR(i));
    }
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry wpcm450_aic_handle_irq(struct pt_regs *regs)
    {
    int hwirq;
// Determine the interrupt source
// Read IPER to signal that nIRQ can be de-asserted
    hwirq = readl(aic.regs + AIC_IPER) / 4;
    generic_handle_domain_irq(aic.domain, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_eoi(d: *mut irq_data) {
    static void wpcm450_aic_eoi(struct irq_data *d)
    {
// Signal end-of-service
    writel(0, aic.regs + AIC_EOSCR);
    }
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_mask(d: *mut irq_data) {
    static void wpcm450_aic_mask(struct irq_data *d)
    {
    let mut mask: c_uint = BIT(d.hwirq);
// Disable (mask) the interrupt
    writel(mask, aic.regs + AIC_MDCR);
    }
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_unmask(d: *mut irq_data) {
    static void wpcm450_aic_unmask(struct irq_data *d)
    {
    let mut mask: c_uint = BIT(d.hwirq);
// Enable (unmask) the interrupt
    writel(mask, aic.regs + AIC_MECR);
    }
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_set_type(d: *mut irq_data, flow_type: c_uint) -> c_int {
    static int wpcm450_aic_set_type(struct irq_data *d, unsigned int flow_type)
    {
//
// The hardware supports high/low level, as well as rising/falling edge
// modes, and the DT binding accommodates for that, but as long as
// other modes than high level mode are not used and can't be tested,
// they are rejected in this driver.
//
    if ((flow_type & IRQ_TYPE_SENSE_MASK) != IRQ_TYPE_LEVEL_HIGH)
    return -EINVAL;
    return 0;
    }
    static struct irq_chip wpcm450_aic_chip = {
    .name = "wpcm450-aic",
    .irq_eoi = wpcm450_aic_eoi,
    .irq_mask = wpcm450_aic_mask,
    .irq_unmask = wpcm450_aic_unmask,
    .irq_set_type = wpcm450_aic_set_type,
    };
#[no_mangle]
unsafe extern "C" fn wpcm450_aic_map(d: *mut irq_domain, irq: c_uint, hwirq: irq_hw_number_t) -> c_int {
    static int wpcm450_aic_map(struct irq_domain *d, unsigned int irq, irq_hw_number_t hwirq)
    {
    if (hwirq >= AIC_NUM_IRQS)
    return -EPERM;
    irq_set_chip_and_handler(irq, &wpcm450_aic_chip, handle_fasteoi_irq);
    irq_set_chip_data(irq, aic);
    irq_set_probe(irq);
    return 0;
    }
    static const struct irq_domain_ops wpcm450_aic_ops = {
    .map = wpcm450_aic_map,
    .xlate = irq_domain_xlate_twocell,
    };
    static int __init wpcm450_aic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    if (parent)
    return -EINVAL;
    aic = kzalloc_obj(*aic);
    if (!aic)
    return -ENOMEM;
    aic.regs = of_iomap(node, 0);
    if (!aic.regs) {
    pr_err("Failed to map WPCM450 AIC registers\n");
    kfree(aic);
    return -ENOMEM;
    }
    wpcm450_aic_init_hw();
    set_handle_irq(wpcm450_aic_handle_irq);
    aic.domain = irq_domain_create_linear(of_fwnode_handle(node), AIC_NUM_IRQS, &wpcm450_aic_ops, aic);
    return 0;
    }
    IRQCHIP_DECLARE(wpcm450_aic, "nuvoton,wpcm450-aic", wpcm450_aic_of_init);
