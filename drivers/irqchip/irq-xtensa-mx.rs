//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-xtensa-mx.c
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
// Xtensa MX interrupt distributor
//
// Copyright (C) 2002 - 2013 Tensilica, Inc.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//

pub const HW_IRQ_IPI_COUNT: c_int = 2;
pub const HW_IRQ_MX_BASE: c_int = 2;
pub const HW_IRQ_EXTERN_BASE: c_int = 3;
    static DEFINE_PER_CPU(unsigned int, cached_irq_mask);
    static int xtensa_mx_irq_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    if (hw < HW_IRQ_IPI_COUNT) {
    struct irq_chip *irq_chip = d.host_data;
    irq_set_chip_and_handler_name(irq, irq_chip,
    handle_percpu_irq, "ipi");
    irq_set_status_flags(irq, IRQ_LEVEL);
    return 0;
    }
    irqd_set_single_target(irq_desc_get_irq_data(irq_to_desc(irq)));
    return xtensa_irq_map(d, irq, hw);
    }
//
// Device Tree IRQ specifier translation function which works with one or
// two cell bindings. First cell value maps directly to the hwirq number.
// Second cell if present specifies whether hwirq number is external (1) or
// internal (0).
//
    static int xtensa_mx_irq_domain_xlate(struct irq_domain *d,
    struct device_node *ctrlr,
    const u32 *intspec, unsigned int intsize,
    unsigned long *out_hwirq, unsigned int *out_type)
    {
    return xtensa_irq_domain_xlate(intspec, intsize,
    intspec[0], intspec[0] + HW_IRQ_EXTERN_BASE,
    out_hwirq, out_type);
    }
    static const struct irq_domain_ops xtensa_mx_irq_domain_ops = {
    .xlate = xtensa_mx_irq_domain_xlate,
    .map = xtensa_mx_irq_map,
    };
#[no_mangle]
pub unsafe extern "C" fn secondary_init_irq() {
    void secondary_init_irq(void)
    {
    __this_cpu_write(cached_irq_mask,
    XCHAL_INTTYPE_MASK_EXTERN_EDGE |
    XCHAL_INTTYPE_MASK_EXTERN_LEVEL);
    xtensa_set_sr(XCHAL_INTTYPE_MASK_EXTERN_EDGE |
    XCHAL_INTTYPE_MASK_EXTERN_LEVEL, intenable);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_mask(d: *mut irq_data) {
    static void xtensa_mx_irq_mask(struct irq_data *d)
    {
    let mut mask: c_uint = 1u << d.hwirq;
    if (mask & (XCHAL_INTTYPE_MASK_EXTERN_EDGE |
    XCHAL_INTTYPE_MASK_EXTERN_LEVEL)) {
    let mut ext_irq: c_uint = xtensa_get_ext_irq_no(d.hwirq);
    if (ext_irq >= HW_IRQ_MX_BASE) {
    set_er(1u << (ext_irq - HW_IRQ_MX_BASE), MIENG);
    return;
    }
    }
    mask = __this_cpu_read(cached_irq_mask) & ~mask;
    __this_cpu_write(cached_irq_mask, mask);
    xtensa_set_sr(mask, intenable);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_unmask(d: *mut irq_data) {
    static void xtensa_mx_irq_unmask(struct irq_data *d)
    {
    let mut mask: c_uint = 1u << d.hwirq;
    if (mask & (XCHAL_INTTYPE_MASK_EXTERN_EDGE |
    XCHAL_INTTYPE_MASK_EXTERN_LEVEL)) {
    let mut ext_irq: c_uint = xtensa_get_ext_irq_no(d.hwirq);
    if (ext_irq >= HW_IRQ_MX_BASE) {
    set_er(1u << (ext_irq - HW_IRQ_MX_BASE), MIENGSET);
    return;
    }
    }
    mask |= __this_cpu_read(cached_irq_mask);
    __this_cpu_write(cached_irq_mask, mask);
    xtensa_set_sr(mask, intenable);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_enable(d: *mut irq_data) {
    static void xtensa_mx_irq_enable(struct irq_data *d)
    {
    xtensa_mx_irq_unmask(d);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_disable(d: *mut irq_data) {
    static void xtensa_mx_irq_disable(struct irq_data *d)
    {
    xtensa_mx_irq_mask(d);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_ack(d: *mut irq_data) {
    static void xtensa_mx_irq_ack(struct irq_data *d)
    {
    xtensa_set_sr(1 << d.hwirq, intclear);
    }
#[no_mangle]
unsafe extern "C" fn xtensa_mx_irq_retrigger(d: *mut irq_data) -> c_int {
    static int xtensa_mx_irq_retrigger(struct irq_data *d)
    {
    let mut mask: c_uint = 1u << d.hwirq;
    if (WARN_ON(mask & ~XCHAL_INTTYPE_MASK_SOFTWARE))
    return 0;
    xtensa_set_sr(mask, intset);
    return 1;
    }
    static int xtensa_mx_irq_set_affinity(struct irq_data *d,
    const struct cpumask *dest, bool force)
    {
    let mut cpu: c_int = cpumask_any_and(dest, cpu_online_mask);
    let mut mask: unsigned = 1u << cpu;
    set_er(mask, MIROUT(d.hwirq - HW_IRQ_MX_BASE));
    irq_data_update_effective_affinity(d, cpumask_of(cpu));
    return 0;
    }
    static struct irq_chip xtensa_mx_irq_chip = {
    .name		= "xtensa-mx",
    .irq_enable	= xtensa_mx_irq_enable,
    .irq_disable	= xtensa_mx_irq_disable,
    .irq_mask	= xtensa_mx_irq_mask,
    .irq_unmask	= xtensa_mx_irq_unmask,
    .irq_ack	= xtensa_mx_irq_ack,
    .irq_retrigger	= xtensa_mx_irq_retrigger,
    .irq_set_affinity = xtensa_mx_irq_set_affinity,
    };
#[no_mangle]
unsafe extern "C" fn xtensa_mx_init_common(root_domain: *mut irq_domain) -> void __init {
    static void __init xtensa_mx_init_common(struct irq_domain *root_domain)
    {
    unsigned int i;
    irq_set_default_domain(root_domain);
    secondary_init_irq();
// Initialize default IRQ routing to CPU 0
    for (i = 0; i < XCHAL_NUM_EXTINTERRUPTS; ++i)
    set_er(1, MIROUT(i));
    }
#[no_mangle]
pub unsafe extern "C" fn xtensa_mx_init_legacy(interrupt_parent: *mut device_node) -> int __init {
    int __init xtensa_mx_init_legacy(struct device_node *interrupt_parent)
    {
    struct irq_domain *root_domain =
    irq_domain_create_legacy(core::ptr::null_mut(), NR_IRQS - 1, 1, 0, &xtensa_mx_irq_domain_ops,
    &xtensa_mx_irq_chip);
    xtensa_mx_init_common(root_domain);
    return 0;
    }
    static int __init xtensa_mx_init(struct device_node *np,
    struct device_node *interrupt_parent)
    {
    struct irq_domain *root_domain =
    irq_domain_create_linear(of_fwnode_handle(np), NR_IRQS, &xtensa_mx_irq_domain_ops,
    &xtensa_mx_irq_chip);
    xtensa_mx_init_common(root_domain);
    return 0;
    }
    IRQCHIP_DECLARE(xtensa_mx_irq_chip, "cdns,xtensa-mx", xtensa_mx_init);
