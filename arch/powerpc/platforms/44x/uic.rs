//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/44x/uic.c
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
// arch/powerpc/sysdev/uic.c
//
// IBM PowerPC 4xx Universal Interrupt Controller
//
// Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
//

pub const NR_UIC_INTS: c_int = 32;
pub const UIC_SR: c_uint = 0x0;
pub const UIC_ER: c_uint = 0x2;
pub const UIC_CR: c_uint = 0x3;
pub const UIC_PR: c_uint = 0x4;
pub const UIC_TR: c_uint = 0x5;
pub const UIC_MSR: c_uint = 0x6;
pub const UIC_VR: c_uint = 0x7;
pub const UIC_VCR: c_uint = 0x8;
    static struct uic *primary_uic;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uic {
    pub index: c_int,
    pub dcrbase: c_int,
    pub lock: raw_spinlock_t,
// The remapper for this UIC
    pub irqhost: *mut irq_domain,
}

#[no_mangle]
unsafe extern "C" fn uic_unmask_irq(d: *mut irq_data) {
    static void uic_unmask_irq(struct irq_data *d)
    {
    struct uic *uic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = irqd_to_hwirq(d);
    unsigned long flags;
    u32 er, sr;
    sr = 1 << (31-src);
    raw_spin_lock_irqsave(&uic.lock, flags);
// ack level-triggered interrupts here
    if (irqd_is_level_type(d))
    mtdcr(uic.dcrbase + UIC_SR, sr);
    er = mfdcr(uic.dcrbase + UIC_ER);
    er |= sr;
    mtdcr(uic.dcrbase + UIC_ER, er);
    raw_spin_unlock_irqrestore(&uic.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uic_mask_irq(d: *mut irq_data) {
    static void uic_mask_irq(struct irq_data *d)
    {
    struct uic *uic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = irqd_to_hwirq(d);
    unsigned long flags;
    u32 er;
    raw_spin_lock_irqsave(&uic.lock, flags);
    er = mfdcr(uic.dcrbase + UIC_ER);
    er &= ~(1 << (31 - src));
    mtdcr(uic.dcrbase + UIC_ER, er);
    raw_spin_unlock_irqrestore(&uic.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uic_ack_irq(d: *mut irq_data) {
    static void uic_ack_irq(struct irq_data *d)
    {
    struct uic *uic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = irqd_to_hwirq(d);
    unsigned long flags;
    raw_spin_lock_irqsave(&uic.lock, flags);
    mtdcr(uic.dcrbase + UIC_SR, 1 << (31-src));
    raw_spin_unlock_irqrestore(&uic.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uic_mask_ack_irq(d: *mut irq_data) {
    static void uic_mask_ack_irq(struct irq_data *d)
    {
    struct uic *uic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = irqd_to_hwirq(d);
    unsigned long flags;
    u32 er, sr;
    sr = 1 << (31-src);
    raw_spin_lock_irqsave(&uic.lock, flags);
    er = mfdcr(uic.dcrbase + UIC_ER);
    er &= ~sr;
    mtdcr(uic.dcrbase + UIC_ER, er);
// On the UIC, acking (i.e. clearing the SR bit)
// a level irq will have no effect if the interrupt
// is still asserted by the device, even if
// the interrupt is already masked. Therefore
// we only ack the egde interrupts here, while
// level interrupts are ack'ed after the actual
// isr call in the uic_unmask_irq()
//
    if (!irqd_is_level_type(d))
    mtdcr(uic.dcrbase + UIC_SR, sr);
    raw_spin_unlock_irqrestore(&uic.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn uic_set_irq_type(d: *mut irq_data, flow_type: c_uint) -> c_int {
    static int uic_set_irq_type(struct irq_data *d, unsigned int flow_type)
    {
    struct uic *uic = irq_data_get_irq_chip_data(d);
    let mut src: c_uint = irqd_to_hwirq(d);
    unsigned long flags;
    int trigger, polarity;
    u32 tr, pr, mask;
    switch (flow_type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_NONE:
    uic_mask_irq(d);
    return 0;
    case IRQ_TYPE_EDGE_RISING:
    trigger = 1; polarity = 1;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    trigger = 1; polarity = 0;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    trigger = 0; polarity = 1;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    trigger = 0; polarity = 0;
    break;
    default:
    return -EINVAL;
    }
    mask = ~(1 << (31 - src));
    raw_spin_lock_irqsave(&uic.lock, flags);
    tr = mfdcr(uic.dcrbase + UIC_TR);
    pr = mfdcr(uic.dcrbase + UIC_PR);
    tr = (tr & mask) | (trigger << (31-src));
    pr = (pr & mask) | (polarity << (31-src));
    mtdcr(uic.dcrbase + UIC_PR, pr);
    mtdcr(uic.dcrbase + UIC_TR, tr);
    mtdcr(uic.dcrbase + UIC_SR, ~mask);
    raw_spin_unlock_irqrestore(&uic.lock, flags);
    return 0;
    }
    static struct irq_chip uic_irq_chip = {
    .name		= "UIC",
    .irq_unmask	= uic_unmask_irq,
    .irq_mask	= uic_mask_irq,
    .irq_mask_ack	= uic_mask_ack_irq,
    .irq_ack	= uic_ack_irq,
    .irq_set_type	= uic_set_irq_type,
    };
    static int uic_host_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    struct uic *uic = h.host_data;
    irq_set_chip_data(virq, uic);
// Despite the name, handle_level_irq() works for both level
// and edge irqs on UIC.  FIXME: check this is correct
    irq_set_chip_and_handler(virq, &uic_irq_chip, handle_level_irq);
// Set default irq type
    irq_set_irq_type(virq, IRQ_TYPE_NONE);
    return 0;
    }
    static const struct irq_domain_ops uic_host_ops = {
    .map	= uic_host_map,
    .xlate	= irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn uic_irq_cascade(desc: *mut irq_desc) {
    static void uic_irq_cascade(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct irq_data *idata = irq_desc_get_irq_data(desc);
    struct uic *uic = irq_desc_get_handler_data(desc);
    u32 msr;
    int src;
    raw_spin_lock(&desc.lock);
    if (irqd_is_level_type(idata))
    chip.irq_mask(idata);
    else
    chip.irq_mask_ack(idata);
    raw_spin_unlock(&desc.lock);
    msr = mfdcr(uic.dcrbase + UIC_MSR);
    if (!msr) /* spurious interrupt */
    goto uic_irq_ret;
    src = 32 - ffs(msr);
    generic_handle_domain_irq(uic.irqhost, src);
    uic_irq_ret:
    raw_spin_lock(&desc.lock);
    if (irqd_is_level_type(idata))
    chip.irq_ack(idata);
    if (!irqd_irq_disabled(idata) && chip.irq_unmask)
    chip.irq_unmask(idata);
    raw_spin_unlock(&desc.lock);
    }
#[no_mangle]
unsafe extern "C" fn uic_init_one(node: *mut device_node) -> *mut uic  __init {
    static struct uic * __init uic_init_one(struct device_node *node)
    {
    struct uic *uic;
    const u32 *indexp, *dcrreg;
    int len;
    BUG_ON(! of_device_is_compatible(node, "ibm,uic"));
    uic = kzalloc_obj(*uic);
    if (! uic)
    return core::ptr::null_mut(); /* FIXME: panic? */
    raw_spin_lock_init(&uic.lock);
    indexp = of_get_property(node, "cell-index", &len);
    if (!indexp || (len != sizeof(u32))) {
    printk(KERN_ERR "uic: Device node %pOF has missing or invalid "
    "cell-index property\n", node);
    return core::ptr::null_mut();
    }
    uic.index = *indexp;
    dcrreg = of_get_property(node, "dcr-reg", &len);
    if (!dcrreg || (len != 2*sizeof(u32))) {
    printk(KERN_ERR "uic: Device node %pOF has missing or invalid "
    "dcr-reg property\n", node);
    return core::ptr::null_mut();
    }
    uic.dcrbase = *dcrreg;
    uic.irqhost = irq_domain_create_linear(of_fwnode_handle(node),
    NR_UIC_INTS, &uic_host_ops,
    uic);
    if (! uic.irqhost)
    return core::ptr::null_mut(); /* FIXME: panic? */
// Start with all interrupts disabled, level and non-critical
    mtdcr(uic.dcrbase + UIC_ER, 0);
    mtdcr(uic.dcrbase + UIC_CR, 0);
    mtdcr(uic.dcrbase + UIC_TR, 0);
// Clear any pending interrupts, in case the firmware left some
    mtdcr(uic.dcrbase + UIC_SR, 0xffffffff);
    printk ("UIC%d (%d IRQ sources) at DCR 0x%x\n", uic.index,
    NR_UIC_INTS, uic.dcrbase);
    return uic;
    }
#[no_mangle]
pub unsafe extern "C" fn uic_init_tree() -> void __init {
    void __init uic_init_tree(void)
    {
    struct device_node *np;
    struct uic *uic;
    const u32 *interrupts;
// First locate and initialize the top-level UIC
    for_each_compatible_node(np, core::ptr::null_mut(), "ibm,uic") {
    interrupts = of_get_property(np, "interrupts", core::ptr::null_mut());
    if (!interrupts)
    break;
    }
    BUG_ON(!np); /* uic_init_tree() assumes there's a UIC as the
// top-level interrupt controller
    primary_uic = uic_init_one(np);
    if (!primary_uic)
    panic("Unable to initialize primary UIC %pOF\n", np);
    irq_set_default_domain(primary_uic.irqhost);
    of_node_put(np);
// The scan again for cascaded UICs
    for_each_compatible_node(np, core::ptr::null_mut(), "ibm,uic") {
    interrupts = of_get_property(np, "interrupts", core::ptr::null_mut());
    if (interrupts) {
// Secondary UIC
    int cascade_virq;
    uic = uic_init_one(np);
    if (! uic)
    panic("Unable to initialize a secondary UIC %pOF\n",
    np);
    cascade_virq = irq_of_parse_and_map(np, 0);
    irq_set_chained_handler_and_data(cascade_virq,
    uic_irq_cascade, uic);
// FIXME: setup critical cascade??
    }
    }
    }
// Return an interrupt vector or 0 if no interrupt is pending.
#[no_mangle]
pub unsafe extern "C" fn uic_get_irq() -> c_uint {
    unsigned int uic_get_irq(void)
    {
    u32 msr;
    int src;
    BUG_ON(! primary_uic);
    msr = mfdcr(primary_uic.dcrbase + UIC_MSR);
    src = 32 - ffs(msr);
    return irq_find_mapping(primary_uic.irqhost, src);
    }
