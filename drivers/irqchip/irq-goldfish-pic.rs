//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-goldfish-pic.c
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
// Driver for MIPS Goldfish Programmable Interrupt Controller.
//
// Author: Miodrag Dinic <miodrag.dinic@mips.com>
//

pub const GFPIC_NR_IRQS: c_int = 32;
// 8..39 Cascaded Goldfish PIC interrupts
pub const GFPIC_IRQ_BASE: c_int = 8;
pub const GFPIC_REG_IRQ_PENDING: c_uint = 0x04;
pub const GFPIC_REG_IRQ_DISABLE_ALL: c_uint = 0x08;
pub const GFPIC_REG_IRQ_DISABLE: c_uint = 0x0c;
pub const GFPIC_REG_IRQ_ENABLE: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct goldfish_pic_data {
    pub base: *mut void __iomem,
    pub irq_domain: *mut irq_domain,
}

#[no_mangle]
unsafe extern "C" fn goldfish_pic_cascade(desc: *mut irq_desc) {
    static void goldfish_pic_cascade(struct irq_desc *desc)
    {
    struct goldfish_pic_data *gfpic = irq_desc_get_handler_data(desc);
    struct irq_chip *host_chip = irq_desc_get_chip(desc);
    u32 pending, hwirq;
    chained_irq_enter(host_chip, desc);
    pending = readl(gfpic.base + GFPIC_REG_IRQ_PENDING);
    while (pending) {
    hwirq = __fls(pending);
    generic_handle_domain_irq(gfpic.irq_domain, hwirq);
    pending &= ~(1 << hwirq);
    }
    chained_irq_exit(host_chip, desc);
    }
    static const struct irq_domain_ops goldfish_irq_domain_ops = {
    .xlate = irq_domain_xlate_onecell,
    };
    static int __init goldfish_pic_of_init(struct device_node *of_node,
    struct device_node *parent)
    {
    struct goldfish_pic_data *gfpic;
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    unsigned int parent_irq;
    let mut ret: c_int = 0;
    gfpic = kzalloc_obj(*gfpic);
    if (!gfpic) {
    ret = -ENOMEM;
    goto out_err;
    }
    parent_irq = irq_of_parse_and_map(of_node, 0);
    if (!parent_irq) {
    pr_err("Failed to map parent IRQ!\n");
    ret = -EINVAL;
    goto out_free;
    }
    gfpic.base = of_iomap(of_node, 0);
    if (!gfpic.base) {
    pr_err("Failed to map base address!\n");
    ret = -ENOMEM;
    goto out_unmap_irq;
    }
// Mask interrupts.
    writel(1, gfpic.base + GFPIC_REG_IRQ_DISABLE_ALL);
    gc = irq_alloc_generic_chip("GFPIC", 1, GFPIC_IRQ_BASE, gfpic.base,
    handle_level_irq);
    if (!gc) {
    pr_err("Failed to allocate chip structures!\n");
    ret = -ENOMEM;
    goto out_iounmap;
    }
    ct = gc.chip_types;
    ct.regs.enable = GFPIC_REG_IRQ_ENABLE;
    ct.regs.disable = GFPIC_REG_IRQ_DISABLE;
    ct.chip.irq_unmask = irq_gc_unmask_enable_reg;
    ct.chip.irq_mask = irq_gc_mask_disable_reg;
    irq_setup_generic_chip(gc, IRQ_MSK(GFPIC_NR_IRQS), 0,
    IRQ_NOPROBE | IRQ_LEVEL, 0);
    gfpic.irq_domain = irq_domain_create_legacy(of_fwnode_handle(of_node), GFPIC_NR_IRQS,
    GFPIC_IRQ_BASE, 0, &goldfish_irq_domain_ops,
    core::ptr::null_mut());
    if (!gfpic.irq_domain) {
    pr_err("Failed to add irqdomain!\n");
    ret = -ENOMEM;
    goto out_destroy_generic_chip;
    }
    irq_set_chained_handler_and_data(parent_irq,
    goldfish_pic_cascade, gfpic);
    pr_info("Successfully registered.\n");
    return 0;
    out_destroy_generic_chip:
    irq_destroy_generic_chip(gc, IRQ_MSK(GFPIC_NR_IRQS),
    IRQ_NOPROBE | IRQ_LEVEL, 0);
    out_iounmap:
    iounmap(gfpic.base);
    out_unmap_irq:
    irq_dispose_mapping(parent_irq);
    out_free:
    kfree(gfpic);
    out_err:
    pr_err("Failed to initialize! (errno = %d)\n", ret);
    return ret;
    }
    IRQCHIP_DECLARE(google_gf_pic, "google,goldfish-pic", goldfish_pic_of_init);
