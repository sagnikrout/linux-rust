//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-mips-cpu.c
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
// Copyright 2001 MontaVista Software Inc.
// Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
//
// Copyright (C) 2001 Ralf Baechle
// Copyright (C) 2005  MIPS Technologies, Inc.	All rights reserved.
// Author: Maciej W. Rozycki <macro@mips.com>
//
// This file define the irq handler for MIPS CPU interrupts.
//
// Almost all MIPS CPUs define 8 interrupt sources.  They are typically
// level triggered (i.e., cannot be cleared from CPU; must be cleared from
// device).
//
// The first two are software interrupts (i.e. not exposed as pins) which
// may be used for IPIs in multi-threaded single-core systems.
//
// The last one is usually the CPU timer interrupt if the counter register
// is present, or for old CPUs with an external FPU by convention it's the
// FPU exception interrupt.
//

    static struct irq_domain *irq_domain;
    static struct irq_domain *ipi_domain;
#[no_mangle]
pub unsafe extern "C" fn unmask_mips_irq(d: *mut irq_data) {
    static inline void unmask_mips_irq(struct irq_data *d)
    {
    set_c0_status(IE_SW0 << d.hwirq);
    irq_enable_hazard();
    }
#[no_mangle]
pub unsafe extern "C" fn mask_mips_irq(d: *mut irq_data) {
    static inline void mask_mips_irq(struct irq_data *d)
    {
    clear_c0_status(IE_SW0 << d.hwirq);
    irq_disable_hazard();
    }
    static struct irq_chip mips_cpu_irq_controller = {
    .name		= "MIPS",
    .irq_ack	= mask_mips_irq,
    .irq_mask	= mask_mips_irq,
    .irq_mask_ack	= mask_mips_irq,
    .irq_unmask	= unmask_mips_irq,
    .irq_eoi	= unmask_mips_irq,
    .irq_disable	= mask_mips_irq,
    .irq_enable	= unmask_mips_irq,
    };
//
// Basically the same as above but taking care of all the MT stuff
//
#[no_mangle]
unsafe extern "C" fn mips_mt_cpu_irq_startup(d: *mut irq_data) -> c_uint {
    static unsigned int mips_mt_cpu_irq_startup(struct irq_data *d)
    {
    let mut vpflags: c_uint = dvpe();
    clear_c0_cause(C_SW0 << d.hwirq);
    evpe(vpflags);
    unmask_mips_irq(d);
    return 0;
    }
//
// While we ack the interrupt interrupts are disabled and thus we don't need
// to deal with concurrency issues.  Same for mips_cpu_irq_end.
//
#[no_mangle]
unsafe extern "C" fn mips_mt_cpu_irq_ack(d: *mut irq_data) {
    static void mips_mt_cpu_irq_ack(struct irq_data *d)
    {
    let mut vpflags: c_uint = dvpe();
    clear_c0_cause(C_SW0 << d.hwirq);
    evpe(vpflags);
    mask_mips_irq(d);
    }

#[no_mangle]
unsafe extern "C" fn mips_mt_send_ipi(d: *mut irq_data, cpu: c_uint) {
    static void mips_mt_send_ipi(struct irq_data *d, unsigned int cpu)
    {
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    unsigned long flags;
    int vpflags;
    local_irq_save(flags);
// We can only send IPIs to VPEs within the local core
    WARN_ON(!cpus_are_siblings(smp_processor_id(), cpu));
    vpflags = dvpe();
    settc(cpu_vpe_id(&cpu_data[cpu]));
    write_vpe_c0_cause(read_vpe_c0_cause() | (C_SW0 << hwirq));
    evpe(vpflags);
    local_irq_restore(flags);
    }

    static struct irq_chip mips_mt_cpu_irq_controller = {
    .name		= "MIPS",
    .irq_startup	= mips_mt_cpu_irq_startup,
    .irq_ack	= mips_mt_cpu_irq_ack,
    .irq_mask	= mask_mips_irq,
    .irq_mask_ack	= mips_mt_cpu_irq_ack,
    .irq_unmask	= unmask_mips_irq,
    .irq_eoi	= unmask_mips_irq,
    .irq_disable	= mask_mips_irq,
    .irq_enable	= unmask_mips_irq,

    .ipi_send_single = mips_mt_send_ipi,

    };
#[no_mangle]
pub unsafe extern "C" fn plat_irq_dispatch() -> asmlinkage void __weak {
    asmlinkage void __weak plat_irq_dispatch(void)
    {
    let mut pending: c_ulong = read_c0_cause() & read_c0_status() & ST0_IM;
    int irq;
    if (!pending) {
    spurious_interrupt();
    return;
    }
    pending >>= CAUSEB_IP;
    while (pending) {
    struct irq_domain *d;
    irq = fls(pending) - 1;
    if (IS_ENABLED(CONFIG_GENERIC_IRQ_IPI) && irq < 2)
    d = ipi_domain;
    else
    d = irq_domain;
    do_domain_IRQ(d, irq);
    pending &= ~BIT(irq);
    }
    }
    static int mips_cpu_intc_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    struct irq_chip *chip;
    if (hw < 2 && cpu_has_mipsmt) {
// Software interrupts are used for MT/CMT IPI
    chip = &mips_mt_cpu_irq_controller;
    } else {
    chip = &mips_cpu_irq_controller;
    }
    if (cpu_has_vint)
    set_vi_handler(hw, plat_irq_dispatch);
    irq_set_chip_and_handler(irq, chip, handle_percpu_irq);
    return 0;
    }
    static const struct irq_domain_ops mips_cpu_intc_irq_domain_ops = {
    .map = mips_cpu_intc_map,
    .xlate = irq_domain_xlate_onecell,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_ipi_domain_state {
    pub 2): DECLARE_BITMAP(allocated,,
}

    static int mips_cpu_ipi_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct cpu_ipi_domain_state *state = domain.host_data;
    unsigned int i, hwirq;
    int ret;
    for (i = 0; i < nr_irqs; i++) {
    hwirq = find_first_zero_bit(state.allocated, 2);
    if (hwirq == 2)
    return -EBUSY;
    bitmap_set(state.allocated, hwirq, 1);
    ret = irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq,
    &mips_mt_cpu_irq_controller,
    core::ptr::null_mut());
    if (ret)
    return ret;
    ret = irq_domain_set_hwirq_and_chip(domain.parent, virq + i, hwirq,
    &mips_mt_cpu_irq_controller,
    core::ptr::null_mut());
    if (ret)
    return ret;
    ret = irq_set_irq_type(virq + i, IRQ_TYPE_LEVEL_HIGH);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int mips_cpu_ipi_match(struct irq_domain *d, struct device_node *node,
    enum irq_domain_bus_token bus_token)
    {
    bool is_ipi;
    switch (bus_token) {
    case DOMAIN_BUS_IPI:
    is_ipi = d.bus_token == bus_token;
    return (!node || (to_of_node(d.fwnode) == node)) && is_ipi;
    default:
    return 0;
    }
    }
    static const struct irq_domain_ops mips_cpu_ipi_chip_ops = {
    .alloc	= mips_cpu_ipi_alloc,
    .match	= mips_cpu_ipi_match,
    };
#[no_mangle]
unsafe extern "C" fn mips_cpu_register_ipi_domain(of_node: *mut device_node) {
    static void mips_cpu_register_ipi_domain(struct device_node *of_node)
    {
    struct cpu_ipi_domain_state *ipi_domain_state;
    ipi_domain_state = kzalloc_obj(*ipi_domain_state);
    ipi_domain = irq_domain_create_hierarchy(irq_domain, IRQ_DOMAIN_FLAG_IPI_SINGLE, 2,
    of_fwnode_handle(of_node),
    &mips_cpu_ipi_chip_ops, ipi_domain_state);
    if (!ipi_domain)
    panic("Failed to add MIPS CPU IPI domain");
    irq_domain_update_bus_token(ipi_domain, DOMAIN_BUS_IPI);
    }

    static inline void mips_cpu_register_ipi_domain(struct device_node *of_node) {}

#[no_mangle]
unsafe extern "C" fn __mips_cpu_irq_init(of_node: *mut device_node) -> void __init {
    static void __init __mips_cpu_irq_init(struct device_node *of_node)
    {
// Mask interrupts.
    clear_c0_status(ST0_IM);
    clear_c0_cause(CAUSEF_IP);
    irq_domain = irq_domain_create_legacy(of_fwnode_handle(of_node), 8, MIPS_CPU_IRQ_BASE, 0,
    &mips_cpu_intc_irq_domain_ops, core::ptr::null_mut());
    if (!irq_domain)
    panic("Failed to add irqdomain for MIPS CPU");
//
// Only proceed to register the software interrupt IPI implementation
// for CPUs which implement the MIPS MT (multi-threading) ASE.
//
    if (cpu_has_mipsmt)
    mips_cpu_register_ipi_domain(of_node);
    }
#[no_mangle]
pub unsafe extern "C" fn mips_cpu_irq_init() -> void __init {
    void __init mips_cpu_irq_init(void)
    {
    __mips_cpu_irq_init(core::ptr::null_mut());
    }
    int __init mips_cpu_irq_of_init(struct device_node *of_node,
    struct device_node *parent)
    {
    __mips_cpu_irq_init(of_node);
    return 0;
    }
    IRQCHIP_DECLARE(cpu_intc, "mti,cpu-interrupt-controller", mips_cpu_irq_of_init);
