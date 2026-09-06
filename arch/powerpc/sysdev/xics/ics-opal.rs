//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/ics-opal.c
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
// ICS backend for OPAL managed interrupts.
//
// Copyright 2011 IBM Corp.
//

#[no_mangle]
unsafe extern "C" fn ics_opal_mangle_server(server: c_int) -> c_int {
    static int ics_opal_mangle_server(int server)
    {
// No link for now
    return server << 2;
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_unmangle_server(server: c_int) -> c_int {
    static int ics_opal_unmangle_server(int server)
    {
// No link for now
    return server >> 2;
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_unmask_irq(d: *mut irq_data) {
    static void ics_opal_unmask_irq(struct irq_data *d)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    int64_t rc;
    int server;
    pr_devel("ics-hal: unmask virq %d [hw 0x%x]\n", d.irq, hw_irq);
    if (hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS)
    return;
    server = xics_get_irq_server(d.irq, irq_data_get_affinity_mask(d), 0);
    server = ics_opal_mangle_server(server);
    rc = opal_set_xive(hw_irq, server, DEFAULT_PRIORITY);
    if (rc != OPAL_SUCCESS)
    pr_err("%s: opal_set_xive(irq=%d [hw 0x%x] server=%x)"
    " error %lld\n",
    __func__, d.irq, hw_irq, server, rc);
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_startup(d: *mut irq_data) -> c_uint {
    static unsigned int ics_opal_startup(struct irq_data *d)
    {
    ics_opal_unmask_irq(d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_mask_real_irq(hw_irq: c_uint) {
    static void ics_opal_mask_real_irq(unsigned int hw_irq)
    {
    let mut server: c_int = ics_opal_mangle_server(xics_default_server);
    int64_t rc;
    if (hw_irq == XICS_IPI)
    return;
// Have to set XIVE to 0xff to be able to remove a slot
    rc = opal_set_xive(hw_irq, server, 0xff);
    if (rc != OPAL_SUCCESS)
    pr_err("%s: opal_set_xive(0xff) irq=%u returned %lld\n",
    __func__, hw_irq, rc);
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_mask_irq(d: *mut irq_data) {
    static void ics_opal_mask_irq(struct irq_data *d)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    pr_devel("ics-hal: mask virq %d [hw 0x%x]\n", d.irq, hw_irq);
    if (hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS)
    return;
    ics_opal_mask_real_irq(hw_irq);
    }
    static int ics_opal_set_affinity(struct irq_data *d,
    const struct cpumask *cpumask,
    bool force)
    {
    let mut hw_irq: c_uint = (unsigned int)irqd_to_hwirq(d);
    __be16 oserver;
    int16_t server;
    int8_t priority;
    int64_t rc;
    int wanted_server;
    if (hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS)
    return -1;
    rc = opal_get_xive(hw_irq, &oserver, &priority);
    if (rc != OPAL_SUCCESS) {
    pr_err("%s: opal_get_xive(irq=%d [hw 0x%x]) error %lld\n",
    __func__, d.irq, hw_irq, rc);
    return -1;
    }
    wanted_server = xics_get_irq_server(d.irq, cpumask, 1);
    if (wanted_server < 0) {
    pr_warn("%s: No online cpus in the mask %*pb for irq %d\n",
    __func__, cpumask_pr_args(cpumask), d.irq);
    return -1;
    }
    server = ics_opal_mangle_server(wanted_server);
    pr_debug("ics-hal: set-affinity irq %d [hw 0x%x] server: 0x%x/0x%x\n",
    d.irq, hw_irq, wanted_server, server);
    rc = opal_set_xive(hw_irq, server, priority);
    if (rc != OPAL_SUCCESS) {
    pr_err("%s: opal_set_xive(irq=%d [hw 0x%x] server=%x)"
    " error %lld\n",
    __func__, d.irq, hw_irq, server, rc);
    return -1;
    }
    return IRQ_SET_MASK_OK;
    }
    static struct irq_chip ics_opal_irq_chip = {
    .name = "OPAL ICS",
    .irq_startup = ics_opal_startup,
    .irq_mask = ics_opal_mask_irq,
    .irq_unmask = ics_opal_unmask_irq,
    .irq_eoi = core::ptr::null_mut(), /* Patched at init time */
    .irq_set_affinity = ics_opal_set_affinity,
    .irq_set_type = xics_set_irq_type,
    .irq_retrigger = xics_retrigger,
    };
#[no_mangle]
unsafe extern "C" fn ics_opal_host_match(ics: *mut ics, node: *mut device_node) -> c_int {
    static int ics_opal_host_match(struct ics *ics, struct device_node *node)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_check(ics: *mut ics, hw_irq: c_uint) -> c_int {
    static int ics_opal_check(struct ics *ics, unsigned int hw_irq)
    {
    int64_t rc;
    __be16 server;
    int8_t priority;
    if (WARN_ON(hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS))
    return -EINVAL;
// Check if HAL knows about this interrupt
    rc = opal_get_xive(hw_irq, &server, &priority);
    if (rc != OPAL_SUCCESS)
    return -ENXIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_mask_unknown(ics: *mut ics, vec: c_ulong) {
    static void ics_opal_mask_unknown(struct ics *ics, unsigned long vec)
    {
    int64_t rc;
    __be16 server;
    int8_t priority;
// Check if HAL knows about this interrupt
    rc = opal_get_xive(vec, &server, &priority);
    if (rc != OPAL_SUCCESS)
    return;
    ics_opal_mask_real_irq(vec);
    }
#[no_mangle]
unsafe extern "C" fn ics_opal_get_server(ics: *mut ics, vec: c_ulong) -> c_long {
    static long ics_opal_get_server(struct ics *ics, unsigned long vec)
    {
    int64_t rc;
    __be16 server;
    int8_t priority;
// Check if HAL knows about this interrupt
    rc = opal_get_xive(vec, &server, &priority);
    if (rc != OPAL_SUCCESS)
    return -1;
    return ics_opal_unmangle_server(be16_to_cpu(server));
    }
// Only one global & state struct ics
    static struct ics ics_hal = {
    .check		= ics_opal_check,
    .mask_unknown	= ics_opal_mask_unknown,
    .get_server	= ics_opal_get_server,
    .host_match	= ics_opal_host_match,
    .chip		= &ics_opal_irq_chip,
    };
#[no_mangle]
pub unsafe extern "C" fn ics_opal_init() -> int __init {
    int __init ics_opal_init(void)
    {
    if (!firmware_has_feature(FW_FEATURE_OPAL))
    return -ENODEV;
// We need to patch our irq chip's EOI to point to the
// right ICP
//
    ics_opal_irq_chip.irq_eoi = icp_ops.eoi;
// Register ourselves
    xics_register_ics(&ics_hal);
    pr_info("ICS OPAL backend registered\n");
    return 0;
    }
