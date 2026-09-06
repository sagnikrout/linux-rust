//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/icp-opal.c
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
// Copyright 2016 IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn icp_opal_teardown_cpu() {
    static void icp_opal_teardown_cpu(void)
    {
    let mut hw_cpu: c_int = hard_smp_processor_id();
// Clear any pending IPI
    opal_int_set_mfrr(hw_cpu, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_flush_ipi() {
    static void icp_opal_flush_ipi(void)
    {
//
// We take the ipi irq but and never return so we need to EOI the IPI,
// but want to leave our priority 0.
//
// Should we check all the other interrupts too?
// Should we be flagging idle loop instead?
// Or creating some task to be scheduled?
//
    if (opal_int_eoi((0x00 << 24) | XICS_IPI) > 0)
    force_external_irq_replay();
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_get_xirr() -> c_uint {
    static unsigned int icp_opal_get_xirr(void)
    {
    unsigned int kvm_xirr;
    __be32 hw_xirr;
    int64_t rc;
// Handle an interrupt latched by KVM first
    kvm_xirr = kvmppc_get_xics_latch();
    if (kvm_xirr)
    return kvm_xirr;
// Then ask OPAL
    rc = opal_int_get_xirr(&hw_xirr, false);
    if (rc < 0)
    return 0;
    return be32_to_cpu(hw_xirr);
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_get_irq() -> c_uint {
    static unsigned int icp_opal_get_irq(void)
    {
    unsigned int xirr;
    unsigned int vec;
    unsigned int irq;
    xirr = icp_opal_get_xirr();
    vec = xirr & 0x00ffffff;
    if (vec == XICS_IRQ_SPURIOUS)
    return 0;
    irq = irq_find_mapping(xics_host, vec);
    if (likely(irq)) {
    xics_push_cppr(vec);
    return irq;
    }
// We don't have a linux mapping, so have rtas mask it.
    xics_mask_unknown_vec(vec);
// We might learn about it later, so EOI it
    if (opal_int_eoi(xirr) > 0)
    force_external_irq_replay();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_set_cpu_priority(cppr: c_uchar) {
    static void icp_opal_set_cpu_priority(unsigned char cppr)
    {
//
// Here be dragons. The caller has asked to allow only IPI's and not
// external interrupts. But OPAL XIVE doesn't support that. So instead
// of allowing no interrupts allow all. That's still not right, but
// currently the only caller who does this is xics_migrate_irqs_away()
// and it works in that case.
//
    if (cppr >= DEFAULT_PRIORITY)
    cppr = LOWEST_PRIORITY;
    xics_set_base_cppr(cppr);
    opal_int_set_cppr(cppr);
    iosync();
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_eoi(d: *mut irq_data) {
    static void icp_opal_eoi(struct irq_data *d)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    int64_t rc;
    iosync();
    rc = opal_int_eoi((xics_pop_cppr() << 24) | hw_irq);
//
// EOI tells us whether there are more interrupts to fetch.
//
// Some HW implementations might not be able to send us another
// external interrupt in that case, so we force a replay.
//
    if (rc > 0)
    force_external_irq_replay();
    }

#[no_mangle]
unsafe extern "C" fn icp_opal_cause_ipi(cpu: c_int) {
    static void icp_opal_cause_ipi(int cpu)
    {
    let mut hw_cpu: c_int = get_hard_smp_processor_id(cpu);
    kvmppc_set_host_ipi(cpu);
    opal_int_set_mfrr(hw_cpu, IPI_PRIORITY);
    }
#[no_mangle]
unsafe extern "C" fn icp_opal_ipi_action(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t icp_opal_ipi_action(int irq, void *dev_id)
    {
    let mut cpu: c_int = smp_processor_id();
    kvmppc_clear_host_ipi(cpu);
    opal_int_set_mfrr(get_hard_smp_processor_id(cpu), 0xff);
    return smp_ipi_demux();
    }
//
// Called when an interrupt is received on an off-line CPU to
// clear the interrupt, so that the CPU can go back to nap mode.
//
#[no_mangle]
pub unsafe extern "C" fn icp_opal_flush_interrupt() {
    void icp_opal_flush_interrupt(void)
    {
    unsigned int xirr;
    unsigned int vec;
    do {
    xirr = icp_opal_get_xirr();
    vec = xirr & 0x00ffffff;
    if (vec == XICS_IRQ_SPURIOUS)
    break;
    if (vec == XICS_IPI) {
// Clear pending IPI
    let mut cpu: c_int = smp_processor_id();
    kvmppc_clear_host_ipi(cpu);
    opal_int_set_mfrr(get_hard_smp_processor_id(cpu), 0xff);
    } else {
    pr_err("XICS: hw interrupt 0x%x to offline cpu, "
    "disabling\n", vec);
    xics_mask_unknown_vec(vec);
    }
// EOI the interrupt
    } while (opal_int_eoi(xirr) > 0);
    }

    static const struct icp_ops icp_opal_ops = {
    .get_irq	= icp_opal_get_irq,
    .eoi		= icp_opal_eoi,
    .set_priority	= icp_opal_set_cpu_priority,
    .teardown_cpu	= icp_opal_teardown_cpu,
    .flush_ipi	= icp_opal_flush_ipi,

    .ipi_action	= icp_opal_ipi_action,
    .cause_ipi	= icp_opal_cause_ipi,

    };
#[no_mangle]
pub unsafe extern "C" fn icp_opal_init() -> int __init {
    int __init icp_opal_init(void)
    {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opal-intc");
    if (!np)
    return -ENODEV;
    icp_ops = &icp_opal_ops;
    printk("XICS: Using OPAL ICP fallbacks\n");
    of_node_put(np);
    return 0;
    }
