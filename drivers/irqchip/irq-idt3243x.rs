//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-idt3243x.c
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
// Driver for IDT/Renesas 79RC3243x Interrupt Controller.
//

pub const IDT_PIC_NR_IRQS: c_int = 32;
pub const IDT_PIC_IRQ_PEND: c_uint = 0x00;
pub const IDT_PIC_IRQ_MASK: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_pic_data {
    pub base: *mut void __iomem,
    pub irq_domain: *mut irq_domain,
    pub gc: *mut irq_chip_generic,
}

#[no_mangle]
unsafe extern "C" fn idt_irq_dispatch(desc: *mut irq_desc) {
    static void idt_irq_dispatch(struct irq_desc *desc)
    {
    struct idt_pic_data *idtpic = irq_desc_get_handler_data(desc);
    struct irq_chip *host_chip = irq_desc_get_chip(desc);
    u32 pending, hwirq;
    chained_irq_enter(host_chip, desc);
    pending = irq_reg_readl(idtpic.gc, IDT_PIC_IRQ_PEND);
    pending &= ~idtpic.gc.mask_cache;
    while (pending) {
    hwirq = __fls(pending);
    generic_handle_domain_irq(idtpic.irq_domain, hwirq);
    pending &= ~(1 << hwirq);
    }
    chained_irq_exit(host_chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn idt_pic_init(of_node: *mut device_node, parent: *mut device_node) -> c_int {
    static int idt_pic_init(struct device_node *of_node, struct device_node *parent)
    {
    struct irq_domain *domain;
    struct idt_pic_data *idtpic;
    struct irq_chip_generic *gc;
    struct irq_chip_type *ct;
    unsigned int parent_irq;
    let mut ret: c_int = 0;
    idtpic = kzalloc_obj(*idtpic);
    if (!idtpic) {
    ret = -ENOMEM;
    goto out_err;
    }
    parent_irq = irq_of_parse_and_map(of_node, 0);
    if (!parent_irq) {
    pr_err("Failed to map parent IRQ!\n");
    ret = -EINVAL;
    goto out_free;
    }
    idtpic.base = of_iomap(of_node, 0);
    if (!idtpic.base) {
    pr_err("Failed to map base address!\n");
    ret = -ENOMEM;
    goto out_unmap_irq;
    }
    domain = irq_domain_create_linear(of_fwnode_handle(of_node), IDT_PIC_NR_IRQS,
    &irq_generic_chip_ops, core::ptr::null_mut());
    if (!domain) {
    pr_err("Failed to add irqdomain!\n");
    ret = -ENOMEM;
    goto out_iounmap;
    }
    idtpic.irq_domain = domain;
    ret = irq_alloc_domain_generic_chips(domain, 32, 1, "IDTPIC",
    handle_level_irq, 0,
    IRQ_NOPROBE | IRQ_LEVEL, 0);
    if (ret)
    goto out_domain_remove;
    gc = irq_get_domain_generic_chip(domain, 0);
    gc.reg_base = idtpic.base;
    gc.private = idtpic;
    ct = gc.chip_types;
    ct.regs.mask = IDT_PIC_IRQ_MASK;
    ct.chip.irq_mask = irq_gc_mask_set_bit;
    ct.chip.irq_unmask = irq_gc_mask_clr_bit;
    idtpic.gc = gc;
// Mask interrupts.
    writel(0xffffffff, idtpic.base + IDT_PIC_IRQ_MASK);
    gc.mask_cache = 0xffffffff;
    irq_set_chained_handler_and_data(parent_irq,
    idt_irq_dispatch, idtpic);
    return 0;
    out_domain_remove:
    irq_domain_remove(domain);
    out_iounmap:
    iounmap(idtpic.base);
    out_unmap_irq:
    irq_dispose_mapping(parent_irq);
    out_free:
    kfree(idtpic);
    out_err:
    pr_err("Failed to initialize! (errno = %d)\n", ret);
    return ret;
    }
    IRQCHIP_DECLARE(idt_pic, "idt,32434-pic", idt_pic_init);
