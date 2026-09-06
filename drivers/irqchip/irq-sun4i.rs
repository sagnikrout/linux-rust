//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-sun4i.c
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


//
// Allwinner A1X SoCs IRQ chip driver.
//
// Copyright (C) 2012 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//
// Based on code from
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
// Benn Huang <benn@allwinnertech.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

pub const SUN4I_IRQ_VECTOR_REG: c_uint = 0x00;
pub const SUN4I_IRQ_PROTECTION_REG: c_uint = 0x08;
pub const SUN4I_IRQ_NMI_CTRL_REG: c_uint = 0x0c;

pub const SUN4I_IRQ_ENABLE_REG_OFFSET: c_uint = 0x40;
pub const SUN4I_IRQ_MASK_REG_OFFSET: c_uint = 0x50;
pub const SUNIV_IRQ_ENABLE_REG_OFFSET: c_uint = 0x20;
pub const SUNIV_IRQ_MASK_REG_OFFSET: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_irq_chip_data {
    pub irq_base: *mut void __iomem,
    pub irq_domain: *mut irq_domain,
    pub enable_reg_offset: u32,
    pub mask_reg_offset: u32,
}

    static struct sun4i_irq_chip_data *irq_ic_data;
    static void __exception_irq_entry sun4i_handle_irq(struct pt_regs *regs);
#[no_mangle]
unsafe extern "C" fn sun4i_irq_ack(irqd: *mut irq_data) {
    static void sun4i_irq_ack(struct irq_data *irqd)
    {
    let mut irq: c_uint = irqd_to_hwirq(irqd);
    if (irq != 0)
    return; /* Only IRQ 0 / the ENMI needs to be acked */
    writel(BIT(0), irq_ic_data.irq_base + SUN4I_IRQ_PENDING_REG(0));
    }
#[no_mangle]
unsafe extern "C" fn sun4i_irq_mask(irqd: *mut irq_data) {
    static void sun4i_irq_mask(struct irq_data *irqd)
    {
    let mut irq: c_uint = irqd_to_hwirq(irqd);
    let mut irq_off: c_uint = irq % 32;
    let mut reg: c_int = irq / 32;
    u32 val;
    val = readl(irq_ic_data.irq_base +
    SUN4I_IRQ_ENABLE_REG(irq_ic_data, reg));
    writel(val & ~(1 << irq_off),
    irq_ic_data.irq_base + SUN4I_IRQ_ENABLE_REG(irq_ic_data, reg));
    }
#[no_mangle]
unsafe extern "C" fn sun4i_irq_unmask(irqd: *mut irq_data) {
    static void sun4i_irq_unmask(struct irq_data *irqd)
    {
    let mut irq: c_uint = irqd_to_hwirq(irqd);
    let mut irq_off: c_uint = irq % 32;
    let mut reg: c_int = irq / 32;
    u32 val;
    val = readl(irq_ic_data.irq_base +
    SUN4I_IRQ_ENABLE_REG(irq_ic_data, reg));
    writel(val | (1 << irq_off),
    irq_ic_data.irq_base + SUN4I_IRQ_ENABLE_REG(irq_ic_data, reg));
    }
    static struct irq_chip sun4i_irq_chip = {
    .name		= "sun4i_irq",
    .irq_eoi	= sun4i_irq_ack,
    .irq_mask	= sun4i_irq_mask,
    .irq_unmask	= sun4i_irq_unmask,
    .flags		= IRQCHIP_EOI_THREADED | IRQCHIP_EOI_IF_HANDLED,
    };
    static int sun4i_irq_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hw)
    {
    irq_set_chip_and_handler(virq, &sun4i_irq_chip, handle_fasteoi_irq);
    irq_set_probe(virq);
    return 0;
    }
    static const struct irq_domain_ops sun4i_irq_ops = {
    .map = sun4i_irq_map,
    .xlate = irq_domain_xlate_onecell,
    };
    static int __init sun4i_of_init(struct device_node *node,
    struct device_node *parent)
    {
    irq_ic_data.irq_base = of_iomap(node, 0);
    if (!irq_ic_data.irq_base)
    panic("%pOF: unable to map IC registers\n",
    node);
// Disable all interrupts
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_ENABLE_REG(irq_ic_data, 0));
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_ENABLE_REG(irq_ic_data, 1));
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_ENABLE_REG(irq_ic_data, 2));
// Unmask all the interrupts, ENABLE_REG(x) is used for masking
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_MASK_REG(irq_ic_data, 0));
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_MASK_REG(irq_ic_data, 1));
    writel(0, irq_ic_data.irq_base + SUN4I_IRQ_MASK_REG(irq_ic_data, 2));
// Clear all the pending interrupts
    writel(0xffffffff, irq_ic_data.irq_base + SUN4I_IRQ_PENDING_REG(0));
    writel(0xffffffff, irq_ic_data.irq_base + SUN4I_IRQ_PENDING_REG(1));
    writel(0xffffffff, irq_ic_data.irq_base + SUN4I_IRQ_PENDING_REG(2));
// Enable protection mode
    writel(0x01, irq_ic_data.irq_base + SUN4I_IRQ_PROTECTION_REG);
// Configure the external interrupt source type
    writel(0x00, irq_ic_data.irq_base + SUN4I_IRQ_NMI_CTRL_REG);
    irq_ic_data.irq_domain = irq_domain_create_linear(of_fwnode_handle(node), 3 * 32,
    &sun4i_irq_ops, core::ptr::null_mut());
    if (!irq_ic_data.irq_domain)
    panic("%pOF: unable to create IRQ domain\n", node);
    set_handle_irq(sun4i_handle_irq);
    return 0;
    }
    static int __init sun4i_ic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    irq_ic_data = kzalloc_obj(struct sun4i_irq_chip_data);
    if (!irq_ic_data)
    return -ENOMEM;
    irq_ic_data.enable_reg_offset = SUN4I_IRQ_ENABLE_REG_OFFSET;
    irq_ic_data.mask_reg_offset = SUN4I_IRQ_MASK_REG_OFFSET;
    return sun4i_of_init(node, parent);
    }
    IRQCHIP_DECLARE(allwinner_sun4i_ic, "allwinner,sun4i-a10-ic", sun4i_ic_of_init);
    static int __init suniv_ic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    irq_ic_data = kzalloc_obj(struct sun4i_irq_chip_data);
    if (!irq_ic_data)
    return -ENOMEM;
    irq_ic_data.enable_reg_offset = SUNIV_IRQ_ENABLE_REG_OFFSET;
    irq_ic_data.mask_reg_offset = SUNIV_IRQ_MASK_REG_OFFSET;
    return sun4i_of_init(node, parent);
    }
    IRQCHIP_DECLARE(allwinner_sunvi_ic, "allwinner,suniv-f1c100s-ic",
    suniv_ic_of_init);
#[no_mangle]
unsafe extern "C" fn sun4i_handle_irq(regs: *mut pt_regs) -> void __exception_irq_entry {
    static void __exception_irq_entry sun4i_handle_irq(struct pt_regs *regs)
    {
    u32 hwirq;
//
// hwirq == 0 can mean one of 3 things:
// 1) no more irqs pending
// 2) irq 0 pending
// 3) spurious irq
// So if we immediately get a reading of 0, check the irq-pending reg
// to differentiate between 2 and 3. We only do this once to avoid
// the extra check in the common case of 1 happening after having
// read the vector-reg once.
//
    hwirq = readl(irq_ic_data.irq_base + SUN4I_IRQ_VECTOR_REG) >> 2;
    if (hwirq == 0 &&
    !(readl(irq_ic_data.irq_base + SUN4I_IRQ_PENDING_REG(0)) &
    BIT(0)))
    return;
    do {
    generic_handle_domain_irq(irq_ic_data.irq_domain, hwirq);
    hwirq = readl(irq_ic_data.irq_base +
    SUN4I_IRQ_VECTOR_REG) >> 2;
    } while (hwirq != 0);
    }
