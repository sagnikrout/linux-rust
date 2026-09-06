//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-xtensa-pic.c
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
// Xtensa built-in interrupt controller
//
// Copyright (C) 2002 - 2013 Tensilica, Inc.
// Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Chris Zankel <chris@zankel.net>
// Kevin Chea
//

//
// Device Tree IRQ specifier translation function which works with one or
// two cell bindings. First cell value maps directly to the hwirq number.
// Second cell if present specifies whether hwirq number is external (1) or
// internal (0).
//
    static int xtensa_pic_irq_domain_xlate(struct irq_domain *d,
    struct device_node *ctrlr,
    const u32 *intspec, unsigned int intsize,
    unsigned long *out_hwirq, unsigned int *out_type)
    {
    return xtensa_irq_domain_xlate(intspec, intsize,
    intspec[0], intspec[0],
    out_hwirq, out_type);
    }
    static const struct irq_domain_ops xtensa_irq_domain_ops = {
    .xlate = xtensa_pic_irq_domain_xlate,
    .map = xtensa_irq_map,
    };
#[no_mangle]
unsafe extern "C" fn xtensa_irq_mask(d: *mut irq_data) {
    static void xtensa_irq_mask(struct irq_data *d)
    {
    u32 irq_mask;
    irq_mask = xtensa_get_sr(intenable);
    irq_mask &= ~BIT(d.hwirq);
    xtensa_set_sr(irq_mask, intenable);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_irq_unmask(d: *mut irq_data) {
    static void xtensa_irq_unmask(struct irq_data *d)
    {
    u32 irq_mask;
    irq_mask = xtensa_get_sr(intenable);
    irq_mask |= BIT(d.hwirq);
    xtensa_set_sr(irq_mask, intenable);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_irq_ack(d: *mut irq_data) {
    static void xtensa_irq_ack(struct irq_data *d)
    {
    xtensa_set_sr(BIT(d.hwirq), intclear);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_irq_retrigger(d: *mut irq_data) -> c_int {
    static int xtensa_irq_retrigger(struct irq_data *d)
    {
    let mut mask: c_uint = BIT(d.hwirq);
    if (WARN_ON(mask & ~XCHAL_INTTYPE_MASK_SOFTWARE))
    return 0;
    xtensa_set_sr(mask, intset);
    return 1;
    }
    static struct irq_chip xtensa_irq_chip = {
    .name		= "xtensa",
    .irq_mask	= xtensa_irq_mask,
    .irq_unmask	= xtensa_irq_unmask,
    .irq_ack	= xtensa_irq_ack,
    .irq_retrigger	= xtensa_irq_retrigger,
    };
#[no_mangle]
pub unsafe extern "C" fn xtensa_pic_init_legacy(interrupt_parent: *mut device_node) -> int __init {
    int __init xtensa_pic_init_legacy(struct device_node *interrupt_parent)
    {
    struct irq_domain *root_domain =
    irq_domain_create_legacy(core::ptr::null_mut(), NR_IRQS - 1, 1, 0,
    &xtensa_irq_domain_ops, &xtensa_irq_chip);
    irq_set_default_domain(root_domain);
    return 0;
    }
    static int __init xtensa_pic_init(struct device_node *np,
    struct device_node *interrupt_parent)
    {
    struct irq_domain *root_domain =
    irq_domain_create_linear(of_fwnode_handle(np), NR_IRQS, &xtensa_irq_domain_ops,
    &xtensa_irq_chip);
    irq_set_default_domain(root_domain);
    return 0;
    }
    IRQCHIP_DECLARE(xtensa_irq_chip, "cdns,xtensa-pic", xtensa_pic_init);
