//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-nvic.c
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
// drivers/irq/irq-nvic.c
//
// Copyright (C) 2008 ARM Limited, All Rights Reserved.
// Copyright (C) 2013 Pengutronix
//
// Support for the Nested Vectored Interrupt Controller found on the
// ARMv7-M CPUs (Cortex-M3/M4)
//

pub const NVIC_ISER: c_uint = 0x000;
pub const NVIC_ICER: c_uint = 0x080;
pub const NVIC_IPR: c_uint = 0x400;
pub const NVIC_MAX_BANKS: c_int = 16;
//
// Each bank handles 32 irqs. Only the 16th (= last) bank handles only
// 16 irqs.
//

    static struct irq_domain *nvic_irq_domain;
#[no_mangle]
unsafe extern "C" fn nvic_handle_irq(regs: *mut pt_regs) -> void __irq_entry {
    static void __irq_entry nvic_handle_irq(struct pt_regs *regs)
    {
    let mut icsr: c_ulong = readl_relaxed(BASEADDR_V7M_SCB + V7M_SCB_ICSR);
    let mut hwirq: irq_hw_number_t = (icsr & V7M_SCB_ICSR_VECTACTIVE) - 16;
    generic_handle_domain_irq(nvic_irq_domain, hwirq);
    }
    static int nvic_irq_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    int i, ret;
    irq_hw_number_t hwirq;
    let mut type: c_uint = IRQ_TYPE_NONE;
    struct irq_fwspec *fwspec = arg;
    ret = irq_domain_translate_onecell(domain, fwspec, &hwirq, &type);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++)
    irq_map_generic_chip(domain, virq + i, hwirq + i);
    return 0;
    }
    static const struct irq_domain_ops nvic_irq_domain_ops = {
    .translate = irq_domain_translate_onecell,
    .alloc = nvic_irq_domain_alloc,
    .free = irq_domain_free_irqs_top,
    };
    static int __init nvic_of_init(struct device_node *node,
    struct device_node *parent)
    {
    let mut clr: c_uint = IRQ_NOREQUEST | IRQ_NOPROBE | IRQ_NOAUTOEN;
    unsigned int irqs, i, numbanks;
    void __iomem *nvic_base;
    int ret;
    numbanks = (readl_relaxed(V7M_SCS_ICTR) &
    V7M_SCS_ICTR_INTLINESNUM_MASK) + 1;
    nvic_base = of_iomap(node, 0);
    if (!nvic_base) {
    pr_warn("unable to map nvic registers\n");
    return -ENOMEM;
    }
    irqs = numbanks * 32;
    if (irqs > NVIC_MAX_IRQ)
    irqs = NVIC_MAX_IRQ;
    nvic_irq_domain =
    irq_domain_create_linear(of_fwnode_handle(node), irqs, &nvic_irq_domain_ops, core::ptr::null_mut());
    if (!nvic_irq_domain) {
    pr_warn("Failed to allocate irq domain\n");
    iounmap(nvic_base);
    return -ENOMEM;
    }
    ret = irq_alloc_domain_generic_chips(nvic_irq_domain, 32, 1,
    "nvic_irq", handle_fasteoi_irq,
    clr, 0, IRQ_GC_INIT_MASK_CACHE);
    if (ret) {
    pr_warn("Failed to allocate irq chips\n");
    irq_domain_remove(nvic_irq_domain);
    iounmap(nvic_base);
    return ret;
    }
    for (i = 0; i < numbanks; ++i) {
    struct irq_chip_generic *gc;
    gc = irq_get_domain_generic_chip(nvic_irq_domain, 32 * i);
    gc.reg_base = nvic_base + 4 * i;
    gc.chip_types[0].regs.enable = NVIC_ISER;
    gc.chip_types[0].regs.disable = NVIC_ICER;
    gc.chip_types[0].chip.irq_mask = irq_gc_mask_disable_reg;
    gc.chip_types[0].chip.irq_unmask = irq_gc_unmask_enable_reg;
// This is a no-op as end of interrupt is signaled by the
// exception return sequence.
//
    gc.chip_types[0].chip.irq_eoi = irq_gc_noop;
// disable interrupts
    writel_relaxed(~0, gc.reg_base + NVIC_ICER);
    }
// Set priority on all interrupts
    for (i = 0; i < irqs; i += 4)
    writel_relaxed(0, nvic_base + NVIC_IPR + i);
    set_handle_irq(nvic_handle_irq);
    return 0;
    }
    IRQCHIP_DECLARE(armv7m_nvic, "arm,armv7m-nvic", nvic_of_init);
