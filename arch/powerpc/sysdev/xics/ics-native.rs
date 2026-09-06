//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/ics-native.c
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
// #define DEBUG

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ics_native {
    pub ics: ics,
    pub node: *mut device_node,
    pub base: *mut void __iomem,
    pub ibase: u32,
    pub icount: u32,
}

    static void __iomem *ics_native_xive(struct ics_native *in, unsigned int vec)
    {
    return in.base + 0x800 + ((vec - in.ibase) << 2);
    }
#[no_mangle]
unsafe extern "C" fn ics_native_unmask_irq(d: *mut irq_data) {
    static void ics_native_unmask_irq(struct irq_data *d)
    {
    let mut vec: c_uint = (unsigned int)irqd_to_hwirq(d);
    struct ics *ics = irq_data_get_irq_chip_data(d);
    struct ics_native *in = to_ics_native(ics);
    unsigned int server;
    pr_devel("ics-native: unmask virq %d [hw 0x%x]\n", d.irq, vec);
    if (vec < in.ibase || vec >= (in.ibase + in.icount))
    return;
    server = xics_get_irq_server(d.irq, irq_data_get_affinity_mask(d), 0);
    out_be32(ics_native_xive(in, vec), (server << 8) | DEFAULT_PRIORITY);
    }
#[no_mangle]
unsafe extern "C" fn ics_native_startup(d: *mut irq_data) -> c_uint {
    static unsigned int ics_native_startup(struct irq_data *d)
    {

//
// The generic MSI code returns with the interrupt disabled on the
// card, using the MSI mask bits. Firmware doesn't appear to unmask
// at that level, so we do it here by hand.
//
    if (irq_data_get_msi_desc(d))
    pci_msi_unmask_irq(d);

// unmask it
    ics_native_unmask_irq(d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ics_native_do_mask(in: *mut ics_native, vec: c_uint) {
    static void ics_native_do_mask(struct ics_native *in, unsigned int vec)
    {
    out_be32(ics_native_xive(in, vec), 0xff);
    }
#[no_mangle]
unsafe extern "C" fn ics_native_mask_irq(d: *mut irq_data) {
    static void ics_native_mask_irq(struct irq_data *d)
    {
    let mut vec: c_uint = (unsigned int)irqd_to_hwirq(d);
    struct ics *ics = irq_data_get_irq_chip_data(d);
    struct ics_native *in = to_ics_native(ics);
    pr_devel("ics-native: mask virq %d [hw 0x%x]\n", d.irq, vec);
    if (vec < in.ibase || vec >= (in.ibase + in.icount))
    return;
    ics_native_do_mask(in, vec);
    }
    static int ics_native_set_affinity(struct irq_data *d,
    const struct cpumask *cpumask,
    bool force)
    {
    let mut vec: c_uint = (unsigned int)irqd_to_hwirq(d);
    struct ics *ics = irq_data_get_irq_chip_data(d);
    struct ics_native *in = to_ics_native(ics);
    int server;
    u32 xive;
    if (vec < in.ibase || vec >= (in.ibase + in.icount))
    return -EINVAL;
    server = xics_get_irq_server(d.irq, cpumask, 1);
    if (server == -1) {
    pr_warn("%s: No online cpus in the mask %*pb for irq %d\n",
    __func__, cpumask_pr_args(cpumask), d.irq);
    return -1;
    }
    xive = in_be32(ics_native_xive(in, vec));
    xive = (xive & 0xff) | (server << 8);
    out_be32(ics_native_xive(in, vec), xive);
    return IRQ_SET_MASK_OK;
    }
    static struct irq_chip ics_native_irq_chip = {
    .name = "ICS",
    .irq_startup		= ics_native_startup,
    .irq_mask		= ics_native_mask_irq,
    .irq_unmask		= ics_native_unmask_irq,
    .irq_eoi		= core::ptr::null_mut(), /* Patched at init time */
    .irq_set_affinity 	= ics_native_set_affinity,
    .irq_set_type		= xics_set_irq_type,
    .irq_retrigger		= xics_retrigger,
    };
#[no_mangle]
unsafe extern "C" fn ics_native_check(ics: *mut ics, hw_irq: c_uint) -> c_int {
    static int ics_native_check(struct ics *ics, unsigned int hw_irq)
    {
    struct ics_native *in = to_ics_native(ics);
    pr_devel("%s: hw_irq=0x%x\n", __func__, hw_irq);
    if (hw_irq < in.ibase || hw_irq >= (in.ibase + in.icount))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ics_native_mask_unknown(ics: *mut ics, vec: c_ulong) {
    static void ics_native_mask_unknown(struct ics *ics, unsigned long vec)
    {
    struct ics_native *in = to_ics_native(ics);
    if (vec < in.ibase || vec >= (in.ibase + in.icount))
    return;
    ics_native_do_mask(in, vec);
    }
#[no_mangle]
unsafe extern "C" fn ics_native_get_server(ics: *mut ics, vec: c_ulong) -> c_long {
    static long ics_native_get_server(struct ics *ics, unsigned long vec)
    {
    struct ics_native *in = to_ics_native(ics);
    u32 xive;
    if (vec < in.ibase || vec >= (in.ibase + in.icount))
    return -EINVAL;
    xive = in_be32(ics_native_xive(in, vec));
    return (xive >> 8) & 0xfff;
    }
#[no_mangle]
unsafe extern "C" fn ics_native_host_match(ics: *mut ics, node: *mut device_node) -> c_int {
    static int ics_native_host_match(struct ics *ics, struct device_node *node)
    {
    struct ics_native *in = to_ics_native(ics);
    return in.node == node;
    }
    static struct ics ics_native_template = {
    .check		= ics_native_check,
    .mask_unknown	= ics_native_mask_unknown,
    .get_server	= ics_native_get_server,
    .host_match	= ics_native_host_match,
    .chip = &ics_native_irq_chip,
    };
#[no_mangle]
unsafe extern "C" fn ics_native_add_one(np: *mut device_node) -> int __init {
    static int __init ics_native_add_one(struct device_node *np)
    {
    struct ics_native *ics;
    u32 ranges[2];
    int rc, count;
    ics = kzalloc_obj(struct ics_native);
    if (!ics)
    return -ENOMEM;
    ics.node = of_node_get(np);
    memcpy(&ics.ics, &ics_native_template, sizeof(struct ics));
    ics.base = of_iomap(np, 0);
    if (!ics.base) {
    pr_err("Failed to map %pOFP\n", np);
    rc = -ENOMEM;
    goto fail;
    }
    count = of_property_count_u32_elems(np, "interrupt-ranges");
    if (count < 2 || count & 1) {
    pr_err("Failed to read interrupt-ranges of %pOFP\n", np);
    rc = -EINVAL;
    goto fail;
    }
    if (count > 2) {
    pr_warn("ICS %pOFP has %d ranges, only one supported\n",
    np, count >> 1);
    }
    rc = of_property_read_u32_array(np, "interrupt-ranges",
    ranges, 2);
    if (rc) {
    pr_err("Failed to read interrupt-ranges of %pOFP\n", np);
    goto fail;
    }
    ics.ibase = ranges[0];
    ics.icount = ranges[1];
    pr_info("ICS native initialized for sources %d..%d\n",
    ics.ibase, ics.ibase + ics.icount - 1);
// Register ourselves
    xics_register_ics(&ics.ics);
    return 0;
    fail:
    of_node_put(ics.node);
    kfree(ics);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn ics_native_init() -> int __init {
    int __init ics_native_init(void)
    {
    struct device_node *ics;
    let mut found_one: bool = false;
// We need to patch our irq chip's EOI to point to the
// right ICP
//
    ics_native_irq_chip.irq_eoi = icp_ops.eoi;
// Find native ICS in the device-tree
    for_each_compatible_node(ics, core::ptr::null_mut(), "openpower,xics-sources") {
    if (ics_native_add_one(ics) == 0)
    found_one = true;
    }
    if (found_one)
    pr_info("ICS native backend registered\n");
    return found_one ? 0 : -ENODEV;
    }
