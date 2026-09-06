//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-or1k-pic.c
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
// Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
// Copyright (C) 2014 Stefan Kristansson <stefan.kristiansson@saunalahti.fi>
//

// OR1K PIC implementation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct or1k_pic_dev {
    pub chip: irq_chip,
    pub handle: irq_flow_handler_t,
    pub flags: c_ulong,
}

//
// We're a couple of cycles faster than the generic implementations with
// these 'fast' versions.
//
#[no_mangle]
unsafe extern "C" fn or1k_pic_mask(data: *mut irq_data) {
    static void or1k_pic_mask(struct irq_data *data)
    {
    mtspr(SPR_PICMR, mfspr(SPR_PICMR) & ~(1UL << data.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn or1k_pic_unmask(data: *mut irq_data) {
    static void or1k_pic_unmask(struct irq_data *data)
    {
    mtspr(SPR_PICMR, mfspr(SPR_PICMR) | (1UL << data.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn or1k_pic_ack(data: *mut irq_data) {
    static void or1k_pic_ack(struct irq_data *data)
    {
    mtspr(SPR_PICSR, (1UL << data.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn or1k_pic_mask_ack(data: *mut irq_data) {
    static void or1k_pic_mask_ack(struct irq_data *data)
    {
    mtspr(SPR_PICMR, mfspr(SPR_PICMR) & ~(1UL << data.hwirq));
    mtspr(SPR_PICSR, (1UL << data.hwirq));
    }
//
// There are two oddities with the OR1200 PIC implementation:
// i)  LEVEL-triggered interrupts are latched and need to be cleared
// ii) the interrupt latch is cleared by writing a 0 to the bit,
// as opposed to a 1 as mandated by the spec
//
#[no_mangle]
unsafe extern "C" fn or1k_pic_or1200_ack(data: *mut irq_data) {
    static void or1k_pic_or1200_ack(struct irq_data *data)
    {
    mtspr(SPR_PICSR, mfspr(SPR_PICSR) & ~(1UL << data.hwirq));
    }
#[no_mangle]
unsafe extern "C" fn or1k_pic_or1200_mask_ack(data: *mut irq_data) {
    static void or1k_pic_or1200_mask_ack(struct irq_data *data)
    {
    mtspr(SPR_PICMR, mfspr(SPR_PICMR) & ~(1UL << data.hwirq));
    mtspr(SPR_PICSR, mfspr(SPR_PICSR) & ~(1UL << data.hwirq));
    }
    static struct or1k_pic_dev or1k_pic_level = {
    .chip = {
    .name = "or1k-PIC-level",
    .irq_unmask = or1k_pic_unmask,
    .irq_mask = or1k_pic_mask,
    },
    .handle = handle_level_irq,
    .flags = IRQ_LEVEL | IRQ_NOPROBE,
    };
    static struct or1k_pic_dev or1k_pic_edge = {
    .chip = {
    .name = "or1k-PIC-edge",
    .irq_unmask = or1k_pic_unmask,
    .irq_mask = or1k_pic_mask,
    .irq_ack = or1k_pic_ack,
    .irq_mask_ack = or1k_pic_mask_ack,
    },
    .handle = handle_edge_irq,
    .flags = IRQ_LEVEL | IRQ_NOPROBE,
    };
    static struct or1k_pic_dev or1k_pic_or1200 = {
    .chip = {
    .name = "or1200-PIC",
    .irq_unmask = or1k_pic_unmask,
    .irq_mask = or1k_pic_mask,
    .irq_ack = or1k_pic_or1200_ack,
    .irq_mask_ack = or1k_pic_or1200_mask_ack,
    },
    .handle = handle_level_irq,
    .flags = IRQ_LEVEL | IRQ_NOPROBE,
    };
    static struct irq_domain *root_domain;
#[no_mangle]
pub unsafe extern "C" fn pic_get_irq(first: c_int) -> c_int {
    static inline int pic_get_irq(int first)
    {
    int hwirq;
    hwirq = ffs(mfspr(SPR_PICSR) >> first);
    if (!hwirq)
    return NO_IRQ;
    else
    hwirq = hwirq + first - 1;
    return hwirq;
    }
#[no_mangle]
unsafe extern "C" fn or1k_pic_handle_irq(regs: *mut pt_regs) {
    static void or1k_pic_handle_irq(struct pt_regs *regs)
    {
    let mut irq: c_int = -1;
    while ((irq = pic_get_irq(irq + 1)) != NO_IRQ)
    generic_handle_domain_irq(root_domain, irq);
    }
//
// The OR1K PIC is a cpu-local interrupt controller and does not distinguish or
// use distinct irq number ranges for per-cpu event interrupts (IPI). Since
// information to determine whether a particular irq number should be treated as
// per-cpu is not available at mapping time, we use a wrapper handler function
// which chooses the right handler at runtime based on whether IRQF_PERCPU was
// used when requesting the irq.  Borrowed from J-Core AIC.
//
#[no_mangle]
unsafe extern "C" fn or1k_irq_flow_handler(desc: *mut irq_desc) {
    static void or1k_irq_flow_handler(struct irq_desc *desc)
    {

    struct irq_data *data = irq_desc_get_irq_data(desc);
    struct or1k_pic_dev *pic = data.domain.host_data;
    if (irqd_is_per_cpu(data))
    handle_percpu_devid_irq(desc);
    else
    pic.handle(desc);

    }
#[no_mangle]
unsafe extern "C" fn or1k_map(d: *mut irq_domain, irq: c_uint, hw: irq_hw_number_t) -> c_int {
    static int or1k_map(struct irq_domain *d, unsigned int irq, irq_hw_number_t hw)
    {
    struct or1k_pic_dev *pic = d.host_data;
    if (IS_ENABLED(CONFIG_SMP))
    irq_set_chip_and_handler(irq, &pic.chip, or1k_irq_flow_handler);
    else
    irq_set_chip_and_handler(irq, &pic.chip, pic.handle);
    irq_set_status_flags(irq, pic.flags);
    return 0;
    }
    static const struct irq_domain_ops or1k_irq_domain_ops = {
    .xlate = irq_domain_xlate_onecell,
    .map = or1k_map,
    };
//
// This sets up the IRQ domain for the PIC built in to the OpenRISC
// 1000 CPU.  This is the "root" domain as these are the interrupts
// that directly trigger an exception in the CPU.
//
    static int __init or1k_pic_init(struct device_node *node,
    struct or1k_pic_dev *pic)
    {
// Disable all interrupts until explicitly requested
    mtspr(SPR_PICMR, (0UL));
    root_domain = irq_domain_create_linear(of_fwnode_handle(node), 32, &or1k_irq_domain_ops,
    pic);
    set_handle_irq(or1k_pic_handle_irq);
    return 0;
    }
    static int __init or1k_pic_or1200_init(struct device_node *node,
    struct device_node *parent)
    {
    return or1k_pic_init(node, &or1k_pic_or1200);
    }
    IRQCHIP_DECLARE(or1k_pic_or1200, "opencores,or1200-pic", or1k_pic_or1200_init);
    IRQCHIP_DECLARE(or1k_pic, "opencores,or1k-pic", or1k_pic_or1200_init);
    static int __init or1k_pic_level_init(struct device_node *node,
    struct device_node *parent)
    {
    return or1k_pic_init(node, &or1k_pic_level);
    }
    IRQCHIP_DECLARE(or1k_pic_level, "opencores,or1k-pic-level",
    or1k_pic_level_init);
    static int __init or1k_pic_edge_init(struct device_node *node,
    struct device_node *parent)
    {
    return or1k_pic_init(node, &or1k_pic_edge);
    }
    IRQCHIP_DECLARE(or1k_pic_edge, "opencores,or1k-pic-edge", or1k_pic_edge_init);
