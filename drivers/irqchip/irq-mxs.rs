//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-mxs.c
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
// Copyright (C) 2009-2010 Freescale Semiconductor, Inc. All Rights Reserved.
// Copyright (C) 2014 Oleksij Rempel <linux@rempel-privat.de>
// Add Alphascale ASM9260 support.
//

//
// this device provide 4 offsets for each register:
// 0x0 - plain read write mode
// 0x4 - set mode, OR logic.
// 0x8 - clr mode, XOR logic.
// 0xc - togle mode.
//
pub const SET_REG: c_int = 4;
pub const CLR_REG: c_int = 8;
pub const HW_ICOLL_VECTOR: c_uint = 0x0000;
pub const HW_ICOLL_LEVELACK: c_uint = 0x0010;
pub const HW_ICOLL_CTRL: c_uint = 0x0020;
pub const HW_ICOLL_STAT_OFFSET: c_uint = 0x0070;
pub const HW_ICOLL_INTERRUPT0: c_uint = 0x0120;

pub const BV_ICOLL_LEVELACK_IRQLEVELACK__LEVEL0: c_uint = 0x1;
pub const ICOLL_NUM_IRQS: c_int = 128;
    enum icoll_type {
    ICOLL,
    ASM9260_ICOLL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icoll_priv {
    pub vector: *mut void __iomem,
    pub levelack: *mut void __iomem,
    pub ctrl: *mut void __iomem,
    pub stat: *mut void __iomem,
    pub intr: *mut void __iomem,
    pub clear: *mut void __iomem,
    pub type: enum icoll_type,
}

    static struct icoll_priv icoll_priv;
    static struct irq_domain *icoll_domain;
// calculate bit offset depending on number of interrupt per register
#[no_mangle]
unsafe extern "C" fn icoll_intr_bitshift(d: *mut irq_data, bit: u32) -> u32 {
    static u32 icoll_intr_bitshift(struct irq_data *d, u32 bit)
    {
//
// mask lower part of hwirq to convert it
// in 0, 1, 2 or 3 and then multiply it by 8 (or shift by 3)
//
    return bit << ((d.hwirq & 3) << 3);
    }
// calculate mem offset depending on number of interrupt per register
    static void __iomem *icoll_intr_reg(struct irq_data *d)
    {
// offset = hwirq / intr_per_reg * 0x10
    return icoll_priv.intr + ((d.hwirq >> 2) * 0x10);
    }
#[no_mangle]
unsafe extern "C" fn icoll_ack_irq(d: *mut irq_data) {
    static void icoll_ack_irq(struct irq_data *d)
    {
//
// The Interrupt Collector is able to prioritize irqs.
// Currently only level 0 is used. So acking can use
// BV_ICOLL_LEVELACK_IRQLEVELACK__LEVEL0 unconditionally.
//
    __raw_writel(BV_ICOLL_LEVELACK_IRQLEVELACK__LEVEL0,
    icoll_priv.levelack);
    }
#[no_mangle]
unsafe extern "C" fn icoll_mask_irq(d: *mut irq_data) {
    static void icoll_mask_irq(struct irq_data *d)
    {
    __raw_writel(BM_ICOLL_INTR_ENABLE,
    icoll_priv.intr + CLR_REG + HW_ICOLL_INTERRUPTn(d.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn icoll_unmask_irq(d: *mut irq_data) {
    static void icoll_unmask_irq(struct irq_data *d)
    {
    __raw_writel(BM_ICOLL_INTR_ENABLE,
    icoll_priv.intr + SET_REG + HW_ICOLL_INTERRUPTn(d.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn asm9260_mask_irq(d: *mut irq_data) {
    static void asm9260_mask_irq(struct irq_data *d)
    {
    __raw_writel(icoll_intr_bitshift(d, BM_ICOLL_INTR_ENABLE),
    icoll_intr_reg(d) + CLR_REG);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_unmask_irq(d: *mut irq_data) {
    static void asm9260_unmask_irq(struct irq_data *d)
    {
    __raw_writel(ASM9260_BM_CLEAR_BIT(d.hwirq),
    icoll_priv.clear +
    ASM9260_HW_ICOLL_CLEARn(d.hwirq));
    __raw_writel(icoll_intr_bitshift(d, BM_ICOLL_INTR_ENABLE),
    icoll_intr_reg(d) + SET_REG);
    }
    static struct irq_chip mxs_icoll_chip = {
    .irq_ack = icoll_ack_irq,
    .irq_mask = icoll_mask_irq,
    .irq_unmask = icoll_unmask_irq,
    .flags = IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SKIP_SET_WAKE,
    };
    static struct irq_chip asm9260_icoll_chip = {
    .irq_ack = icoll_ack_irq,
    .irq_mask = asm9260_mask_irq,
    .irq_unmask = asm9260_unmask_irq,
    .flags = IRQCHIP_MASK_ON_SUSPEND |
    IRQCHIP_SKIP_SET_WAKE,
    };
#[no_mangle]
unsafe extern "C" fn icoll_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry icoll_handle_irq(struct pt_regs *regs)
    {
    u32 irqnr;
    irqnr = __raw_readl(icoll_priv.stat);
    __raw_writel(irqnr, icoll_priv.vector);
    generic_handle_domain_irq(icoll_domain, irqnr);
    }
    static int icoll_irq_domain_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hw)
    {
    struct irq_chip *chip;
    if (icoll_priv.type == ICOLL)
    chip = &mxs_icoll_chip;
    else
    chip = &asm9260_icoll_chip;
    irq_set_chip_and_handler(virq, chip, handle_level_irq);
    return 0;
    }
    static const struct irq_domain_ops icoll_irq_domain_ops = {
    .map = icoll_irq_domain_map,
    .xlate = irq_domain_xlate_onecell,
    };
    static void __init icoll_add_domain(struct device_node *np,
    int num)
    {
    icoll_domain = irq_domain_create_linear(of_fwnode_handle(np), num,
    &icoll_irq_domain_ops, core::ptr::null_mut());
    if (!icoll_domain)
    panic("%pOF: unable to create irq domain", np);
    }
#[no_mangle]
unsafe extern "C" fn icoll_init_iobase(np: *mut device_node) -> *mut void __iomem  __init {
    static void __iomem * __init icoll_init_iobase(struct device_node *np)
    {
    void __iomem *icoll_base;
    icoll_base = of_io_request_and_map(np, 0, np.name);
    if (IS_ERR(icoll_base))
    panic("%pOF: unable to map resource", np);
    return icoll_base;
    }
    static int __init icoll_of_init(struct device_node *np,
    struct device_node *interrupt_parent)
    {
    void __iomem *icoll_base;
    icoll_priv.type = ICOLL;
    icoll_base		= icoll_init_iobase(np);
    icoll_priv.vector	= icoll_base + HW_ICOLL_VECTOR;
    icoll_priv.levelack	= icoll_base + HW_ICOLL_LEVELACK;
    icoll_priv.ctrl		= icoll_base + HW_ICOLL_CTRL;
    icoll_priv.stat		= icoll_base + HW_ICOLL_STAT_OFFSET;
    icoll_priv.intr		= icoll_base + HW_ICOLL_INTERRUPT0;
    icoll_priv.clear	= core::ptr::null_mut();
//
// Interrupt Collector reset, which initializes the priority
// for each irq to level 0.
//
    stmp_reset_block(icoll_priv.ctrl);
    icoll_add_domain(np, ICOLL_NUM_IRQS);
    set_handle_irq(icoll_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(mxs, "fsl,icoll", icoll_of_init);
    static int __init asm9260_of_init(struct device_node *np,
    struct device_node *interrupt_parent)
    {
    void __iomem *icoll_base;
    int i;
    icoll_priv.type = ASM9260_ICOLL;
    icoll_base = icoll_init_iobase(np);
    icoll_priv.vector	= icoll_base + ASM9260_HW_ICOLL_VECTOR;
    icoll_priv.levelack	= icoll_base + ASM9260_HW_ICOLL_LEVELACK;
    icoll_priv.ctrl		= icoll_base + ASM9260_HW_ICOLL_CTRL;
    icoll_priv.stat		= icoll_base + ASM9260_HW_ICOLL_STAT_OFFSET;
    icoll_priv.intr		= icoll_base + ASM9260_HW_ICOLL_INTERRUPT0;
    icoll_priv.clear	= icoll_base + ASM9260_HW_ICOLL_CLEAR0;
    writel_relaxed(ASM9260_BM_CTRL_IRQ_ENABLE,
    icoll_priv.ctrl);
//
// ASM9260 don't provide reset bit. So, we need to set level 0
// manually.
//
    for (i = 0; i < 16 * 0x10; i += 0x10)
    writel(0, icoll_priv.intr + i);
    icoll_add_domain(np, ASM9260_NUM_IRQS);
    set_handle_irq(icoll_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(asm9260, "alphascale,asm9260-icoll", asm9260_of_init);
