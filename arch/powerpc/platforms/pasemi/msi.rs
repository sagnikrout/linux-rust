//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pasemi/msi.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2007, Olof Johansson, PA Semi
//
// Based on arch/powerpc/sysdev/mpic_u3msi.c:
//
// Copyright 2006, Segher Boessenkool, IBM Corporation.
// Copyright 2006-2007, Michael Ellerman, IBM Corporation.
//

// Allocate 16 interrupts per device, to give an alignment of 16,
// since that's the size of the grouping w.r.t. affinity. If someone
// needs more than 32 MSI's down the road we'll have to rethink this,
// but it should be OK for now.
//
pub const ALLOC_CHUNK: c_int = 16;
pub const PASEMI_MSI_ADDR: c_uint = 0xfc080000;
// A bit ugly, can we get this from the pci_dev somehow?
    static struct mpic *msi_mpic;
#[no_mangle]
unsafe extern "C" fn mpic_pasemi_msi_mask_irq(data: *mut irq_data) {
    static void mpic_pasemi_msi_mask_irq(struct irq_data *data)
    {
    pr_debug("mpic_pasemi_msi_mask_irq %d\n", data.irq);
    pci_msi_mask_irq(data);
    mpic_mask_irq(data);
    }
#[no_mangle]
unsafe extern "C" fn mpic_pasemi_msi_unmask_irq(data: *mut irq_data) {
    static void mpic_pasemi_msi_unmask_irq(struct irq_data *data)
    {
    pr_debug("mpic_pasemi_msi_unmask_irq %d\n", data.irq);
    mpic_unmask_irq(data);
    pci_msi_unmask_irq(data);
    }
    static struct irq_chip mpic_pasemi_msi_chip = {
    .irq_shutdown		= mpic_pasemi_msi_mask_irq,
    .irq_mask		= mpic_pasemi_msi_mask_irq,
    .irq_unmask		= mpic_pasemi_msi_unmask_irq,
    .irq_eoi		= mpic_end_irq,
    .irq_set_type		= mpic_set_irq_type,
    .irq_set_affinity	= mpic_set_affinity,
    .name			= "PASEMI-MSI",
    };
#[no_mangle]
unsafe extern "C" fn pasemi_msi_teardown_msi_irqs(pdev: *mut pci_dev) {
    static void pasemi_msi_teardown_msi_irqs(struct pci_dev *pdev)
    {
    struct msi_desc *entry;
    irq_hw_number_t hwirq;
    pr_debug("pasemi_msi_teardown_msi_irqs, pdev %p\n", pdev);
    msi_for_each_desc(entry, &pdev.dev, MSI_DESC_ASSOCIATED) {
    hwirq = virq_to_hw(entry.irq);
    irq_set_msi_desc(entry.irq, core::ptr::null_mut());
    irq_dispose_mapping(entry.irq);
    entry.irq = 0;
    msi_bitmap_free_hwirqs(&msi_mpic.msi_bitmap, hwirq, ALLOC_CHUNK);
    }
    }
#[no_mangle]
unsafe extern "C" fn pasemi_msi_setup_msi_irqs(pdev: *mut pci_dev, nvec: c_int, type: c_int) -> c_int {
    static int pasemi_msi_setup_msi_irqs(struct pci_dev *pdev, int nvec, int type)
    {
    unsigned int virq;
    struct msi_desc *entry;
    struct msi_msg msg;
    int hwirq;
    if (type == PCI_CAP_ID_MSIX)
    pr_debug("pasemi_msi: MSI-X untested, trying anyway\n");
    pr_debug("pasemi_msi_setup_msi_irqs, pdev %p nvec %d type %d\n",
    pdev, nvec, type);
    msg.address_hi = 0;
    msg.address_lo = PASEMI_MSI_ADDR;
    msi_for_each_desc(entry, &pdev.dev, MSI_DESC_NOTASSOCIATED) {
// Allocate 16 interrupts for now, since that's the grouping for
// affinity. This can be changed later if it turns out 32 is too
// few MSIs for someone, but restrictions will apply to how the
// sources can be changed independently.
//
    hwirq = msi_bitmap_alloc_hwirqs(&msi_mpic.msi_bitmap,
    ALLOC_CHUNK);
    if (hwirq < 0) {
    pr_debug("pasemi_msi: failed allocating hwirq\n");
    return hwirq;
    }
    virq = irq_create_mapping(msi_mpic.irqhost, hwirq);
    if (!virq) {
    pr_debug("pasemi_msi: failed mapping hwirq 0x%x\n",
    hwirq);
    msi_bitmap_free_hwirqs(&msi_mpic.msi_bitmap, hwirq,
    ALLOC_CHUNK);
    return -ENOSPC;
    }
// Vector on MSI is really an offset, the hardware adds
// it to the value written at the magic address. So set
// it to 0 to remain sane.
//
    mpic_set_vector(virq, 0);
    irq_set_msi_desc(virq, entry);
    irq_set_chip(virq, &mpic_pasemi_msi_chip);
    irq_set_irq_type(virq, IRQ_TYPE_EDGE_RISING);
    pr_debug("pasemi_msi: allocated virq 0x%x (hw 0x%x) " \
    "addr 0x%x\n", virq, hwirq, msg.address_lo);
// Likewise, the device writes [0...511] into the target
// register to generate MSI [512...1023]
//
    msg.data = hwirq-0x200;
    pci_write_msi_msg(virq, &msg);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mpic_pasemi_msi_init(mpic: *mut mpic) -> int __init {
    int __init mpic_pasemi_msi_init(struct mpic *mpic)
    {
    int rc;
    struct pci_controller *phb;
    struct device_node *of_node;
    of_node = irq_domain_get_of_node(mpic.irqhost);
    if (!of_node ||
    !of_device_is_compatible(of_node,
    "pasemi,pwrficient-openpic"))
    return -ENODEV;
    rc = mpic_msi_init_allocator(mpic);
    if (rc) {
    pr_debug("pasemi_msi: Error allocating bitmap!\n");
    return rc;
    }
    pr_debug("pasemi_msi: Registering PA Semi MPIC MSI callbacks\n");
    msi_mpic = mpic;
    list_for_each_entry(phb, &hose_list, list_node) {
    WARN_ON(phb.controller_ops.setup_msi_irqs);
    phb.controller_ops.setup_msi_irqs = pasemi_msi_setup_msi_irqs;
    phb.controller_ops.teardown_msi_irqs = pasemi_msi_teardown_msi_irqs;
    }
    return 0;
    }
