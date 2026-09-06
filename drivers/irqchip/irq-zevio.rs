//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-zevio.c
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
// linux/drivers/irqchip/irq-zevio.c
//
// Copyright (C) 2013 Daniel Tang <tangrs@tangrs.id.au>
//

pub const IO_STATUS: c_uint = 0x000;
pub const IO_RAW_STATUS: c_uint = 0x004;
pub const IO_ENABLE: c_uint = 0x008;
pub const IO_DISABLE: c_uint = 0x00C;
pub const IO_CURRENT: c_uint = 0x020;
pub const IO_RESET: c_uint = 0x028;
pub const IO_MAX_PRIOTY: c_uint = 0x02C;
pub const IO_IRQ_BASE: c_uint = 0x000;
pub const IO_FIQ_BASE: c_uint = 0x100;
pub const IO_INVERT_SEL: c_uint = 0x200;
pub const IO_STICKY_SEL: c_uint = 0x204;
pub const IO_PRIORITY_SEL: c_uint = 0x300;
pub const MAX_INTRS: c_int = 32;

    static struct irq_domain *zevio_irq_domain;
    static void __iomem *zevio_irq_io;
#[no_mangle]
unsafe extern "C" fn zevio_irq_ack(irqd: *mut irq_data) {
    static void zevio_irq_ack(struct irq_data *irqd)
    {
    struct irq_chip_generic *gc = irq_data_get_irq_chip_data(irqd);
    struct irq_chip_regs *regs = &irq_data_get_chip_type(irqd).regs;
    readl(gc.reg_base + regs.ack);
    }
#[no_mangle]
unsafe extern "C" fn zevio_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry zevio_handle_irq(struct pt_regs *regs)
    {
    int irqnr;
    while (readl(zevio_irq_io + IO_STATUS)) {
    irqnr = readl(zevio_irq_io + IO_CURRENT);
    generic_handle_domain_irq(zevio_irq_domain, irqnr);
    }
    }
#[no_mangle]
unsafe extern "C" fn zevio_init_irq_base(base: *mut void __iomem) -> void __init {
    static void __init zevio_init_irq_base(void __iomem *base)
    {
// Disable all interrupts
    writel(~0, base + IO_DISABLE);
// Accept interrupts of all priorities
    writel(0xF, base + IO_MAX_PRIOTY);
// Reset existing interrupts
    readl(base + IO_RESET);
    }
    static int __init zevio_of_init(struct device_node *node,
    struct device_node *parent)
    {
    let mut clr: c_uint = IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN;
    struct irq_chip_generic *gc;
    int ret;
    if (WARN_ON(zevio_irq_io || zevio_irq_domain))
    return -EBUSY;
    zevio_irq_io = of_iomap(node, 0);
    BUG_ON(!zevio_irq_io);
// Do not invert interrupt status bits
    writel(~0, zevio_irq_io + IO_INVERT_SEL);
// Disable sticky interrupts
    writel(0, zevio_irq_io + IO_STICKY_SEL);
// We don't use IRQ priorities. Set each IRQ to highest priority.
    memset_io(zevio_irq_io + IO_PRIORITY_SEL, 0, MAX_INTRS * sizeof(u32));
// Init IRQ and FIQ
    zevio_init_irq_base(zevio_irq_io + IO_IRQ_BASE);
    zevio_init_irq_base(zevio_irq_io + IO_FIQ_BASE);
    zevio_irq_domain = irq_domain_create_linear(of_fwnode_handle(node), MAX_INTRS,
    &irq_generic_chip_ops, core::ptr::null_mut());
    BUG_ON(!zevio_irq_domain);
    ret = irq_alloc_domain_generic_chips(zevio_irq_domain, MAX_INTRS, 1,
    "zevio_intc", handle_level_irq,
    clr, 0, IRQ_GC_INIT_MASK_CACHE);
    BUG_ON(ret);
    gc = irq_get_domain_generic_chip(zevio_irq_domain, 0);
    gc.reg_base				= zevio_irq_io;
    gc.chip_types[0].chip.irq_ack		= zevio_irq_ack;
    gc.chip_types[0].chip.irq_mask		= irq_gc_mask_disable_reg;
    gc.chip_types[0].chip.irq_unmask	= irq_gc_unmask_enable_reg;
    gc.chip_types[0].regs.mask		= IO_IRQ_BASE + IO_ENABLE;
    gc.chip_types[0].regs.enable		= IO_IRQ_BASE + IO_ENABLE;
    gc.chip_types[0].regs.disable		= IO_IRQ_BASE + IO_DISABLE;
    gc.chip_types[0].regs.ack		= IO_IRQ_BASE + IO_RESET;
    set_handle_irq(zevio_handle_irq);
    pr_info("TI-NSPIRE classic IRQ controller\n");
    return 0;
    }
    IRQCHIP_DECLARE(zevio_irq, "lsi,zevio-intc", zevio_of_init);
