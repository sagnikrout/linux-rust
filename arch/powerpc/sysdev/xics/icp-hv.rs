//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/icp-hv.c
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
// Copyright 2011 IBM Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn icp_hv_get_xirr(cppr: c_uchar) -> c_uint {
    static inline unsigned int icp_hv_get_xirr(unsigned char cppr)
    {
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
    long rc;
    let mut ret: c_uint = XICS_IRQ_SPURIOUS;
    rc = plpar_hcall(H_XIRR, retbuf, cppr);
    if (rc == H_SUCCESS) {
    ret = (unsigned int)retbuf[0];
    } else {
    pr_err("%s: bad return code xirr cppr=0x%x returned %ld\n",
    __func__, cppr, rc);
    WARN_ON_ONCE(1);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn icp_hv_set_cppr(value: u8) {
    static inline void icp_hv_set_cppr(u8 value)
    {
    let mut rc: c_long = plpar_hcall_norets(H_CPPR, value);
    if (rc != H_SUCCESS) {
    pr_err("%s: bad return code cppr cppr=0x%x returned %ld\n",
    __func__, value, rc);
    WARN_ON_ONCE(1);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn icp_hv_set_xirr(value: c_uint) {
    static inline void icp_hv_set_xirr(unsigned int value)
    {
    let mut rc: c_long = plpar_hcall_norets(H_EOI, value);
    if (rc != H_SUCCESS) {
    pr_err("%s: bad return code eoi xirr=0x%x returned %ld\n",
    __func__, value, rc);
    WARN_ON_ONCE(1);
    icp_hv_set_cppr(value >> 24);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn icp_hv_set_qirr(n_cpu: c_int, value: u8) {
    static inline void icp_hv_set_qirr(int n_cpu , u8 value)
    {
    let mut hw_cpu: c_int = get_hard_smp_processor_id(n_cpu);
    long rc;
// Make sure all previous accesses are ordered before IPI sending
    mb();
    rc = plpar_hcall_norets(H_IPI, hw_cpu, value);
    if (rc != H_SUCCESS) {
    pr_err("%s: bad return code qirr cpu=%d hw_cpu=%d mfrr=0x%x "
    "returned %ld\n", __func__, n_cpu, hw_cpu, value, rc);
    WARN_ON_ONCE(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_eoi(d: *mut irq_data) {
    static void icp_hv_eoi(struct irq_data *d)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    iosync();
    icp_hv_set_xirr((xics_pop_cppr() << 24) | hw_irq);
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_teardown_cpu() {
    static void icp_hv_teardown_cpu(void)
    {
    let mut cpu: c_int = smp_processor_id();
// Clear any pending IPI
    icp_hv_set_qirr(cpu, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_flush_ipi() {
    static void icp_hv_flush_ipi(void)
    {
// We take the ipi irq but and never return so we
// need to EOI the IPI, but want to leave our priority 0
//
// should we check all the other interrupts too?
// should we be flagging idle loop instead?
// or creating some task to be scheduled?
//
    icp_hv_set_xirr((0x00 << 24) | XICS_IPI);
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_get_irq() -> c_uint {
    static unsigned int icp_hv_get_irq(void)
    {
    let mut xirr: c_uint = icp_hv_get_xirr(xics_cppr_top());
    let mut vec: c_uint = xirr & 0x00ffffff;
    unsigned int irq;
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
    icp_hv_set_xirr(xirr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_set_cpu_priority(cppr: c_uchar) {
    static void icp_hv_set_cpu_priority(unsigned char cppr)
    {
    xics_set_base_cppr(cppr);
    icp_hv_set_cppr(cppr);
    iosync();
    }

#[no_mangle]
unsafe extern "C" fn icp_hv_cause_ipi(cpu: c_int) {
    static void icp_hv_cause_ipi(int cpu)
    {
    icp_hv_set_qirr(cpu, IPI_PRIORITY);
    }
#[no_mangle]
unsafe extern "C" fn icp_hv_ipi_action(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t icp_hv_ipi_action(int irq, void *dev_id)
    {
    let mut cpu: c_int = smp_processor_id();
    icp_hv_set_qirr(cpu, 0xff);
    return smp_ipi_demux();
    }

    static const struct icp_ops icp_hv_ops = {
    .get_irq	= icp_hv_get_irq,
    .eoi		= icp_hv_eoi,
    .set_priority	= icp_hv_set_cpu_priority,
    .teardown_cpu	= icp_hv_teardown_cpu,
    .flush_ipi	= icp_hv_flush_ipi,

    .ipi_action	= icp_hv_ipi_action,
    .cause_ipi	= icp_hv_cause_ipi,

    };
#[no_mangle]
pub unsafe extern "C" fn icp_hv_init() -> int __init {
    int __init icp_hv_init(void)
    {
    struct device_node *np;
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,ppc-xicp");
    if (!np)
    np = of_find_node_by_type(core::ptr::null_mut(),
    "PowerPC-External-Interrupt-Presentation");
    if (!np)
    return -ENODEV;
    icp_ops = &icp_hv_ops;
    of_node_put(np);
    return 0;
    }
