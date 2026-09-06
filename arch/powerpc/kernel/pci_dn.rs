//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/pci_dn.c
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
// pci_dn.c
//
// Copyright (C) 2001 Todd Inglett, IBM Corporation
//
// PCI manipulation via device_nodes.
//

//
// The function is used to find the firmware data of one
// specific PCI device, which is attached to the indicated
// PCI bus. For VFs, their firmware data is linked to that
// one of PF's bridge. For other devices, their firmware
// data is linked to that of their bridge.
//
    static struct pci_dn *pci_bus_to_pdn(struct pci_bus *bus)
    {
    struct pci_bus *pbus;
    struct device_node *dn;
    struct pci_dn *pdn;
//
// We probably have virtual bus which doesn't
// have associated bridge.
//
    pbus = bus;
    while (pbus) {
    if (pci_is_root_bus(pbus) || pbus.self)
    break;
    pbus = pbus.parent;
    }
//
// Except virtual bus, all PCI buses should
// have device nodes.
//
    dn = pci_bus_to_OF_node(pbus);
    pdn = dn ? PCI_DN(dn) : core::ptr::null_mut();
    return pdn;
    }
    struct pci_dn *pci_get_pdn_by_devfn(struct pci_bus *bus,
    int devfn)
    {
    struct device_node *dn = core::ptr::null_mut();
    struct pci_dn *parent, *pdn;
    struct pci_dev *pdev = core::ptr::null_mut();
// Fast path: fetch from PCI device
    list_for_each_entry(pdev, &bus.devices, bus_list) {
    if (pdev.devfn == devfn) {
    if (pdev.dev.archdata.pci_data)
    return pdev.dev.archdata.pci_data;
    dn = pci_device_to_OF_node(pdev);
    break;
    }
    }
// Fast path: fetch from device node
    pdn = dn ? PCI_DN(dn) : core::ptr::null_mut();
    if (pdn)
    return pdn;
// Slow path: fetch from firmware data hierarchy
    parent = pci_bus_to_pdn(bus);
    if (!parent)
    return core::ptr::null_mut();
    list_for_each_entry(pdn, &parent.child_list, list) {
    if (pdn.busno == bus.number &&
    pdn.devfn == devfn)
    return pdn;
    }
    return core::ptr::null_mut();
    }
    struct pci_dn *pci_get_pdn(struct pci_dev *pdev)
    {
    struct device_node *dn;
    struct pci_dn *parent, *pdn;
// Search device directly
    if (pdev.dev.archdata.pci_data)
    return pdev.dev.archdata.pci_data;
// Check device node
    dn = pci_device_to_OF_node(pdev);
    pdn = dn ? PCI_DN(dn) : core::ptr::null_mut();
    if (pdn)
    return pdn;
//
// VFs don't have device nodes. We hook their
// firmware data to PF's bridge.
//
    parent = pci_bus_to_pdn(pdev.bus);
    if (!parent)
    return core::ptr::null_mut();
    list_for_each_entry(pdn, &parent.child_list, list) {
    if (pdn.busno == pdev.bus.number &&
    pdn.devfn == pdev.devfn)
    return pdn;
    }
    return core::ptr::null_mut();
    }

    static struct eeh_dev *eeh_dev_init(struct pci_dn *pdn)
    {
    struct eeh_dev *edev;
// Allocate EEH device
    edev = kzalloc_obj(*edev);
    if (!edev)
    return core::ptr::null_mut();
// Associate EEH device with OF node
    pdn.edev = edev;
    edev.pdn = pdn;
    edev.bdfn = (pdn.busno << 8) | pdn.devfn;
    edev.controller = pdn.phb;
    return edev;
    }

    static struct pci_dn *add_one_sriov_vf_pdn(struct pci_dn *parent,
    int busno, int devfn)
    {
    struct pci_dn *pdn;
// Except PHB, we always have the parent
    if (!parent)
    return core::ptr::null_mut();
    pdn = kzalloc_obj(*pdn);
    if (!pdn)
    return core::ptr::null_mut();
    pdn.phb = parent.phb;
    pdn.parent = parent;
    pdn.busno = busno;
    pdn.devfn = devfn;
    pdn.pe_number = IODA_INVALID_PE;
    INIT_LIST_HEAD(&pdn.child_list);
    INIT_LIST_HEAD(&pdn.list);
    list_add_tail(&pdn.list, &parent.child_list);
    return pdn;
    }
    struct pci_dn *add_sriov_vf_pdns(struct pci_dev *pdev)
    {
    struct pci_dn *parent, *pdn;
    int i;
// Only support IOV for now
    if (WARN_ON(!pdev.is_physfn))
    return core::ptr::null_mut();
// Check if VFs have been populated
    pdn = pci_get_pdn(pdev);
    if (!pdn || (pdn.flags & PCI_DN_FLAG_IOV_VF))
    return core::ptr::null_mut();
    pdn.flags |= PCI_DN_FLAG_IOV_VF;
    parent = pci_bus_to_pdn(pdev.bus);
    if (!parent)
    return core::ptr::null_mut();
    for (i = 0; i < pci_sriov_get_totalvfs(pdev); i++) {
    struct eeh_dev *edev __maybe_unused;
    pdn = add_one_sriov_vf_pdn(parent,
    pci_iov_virtfn_bus(pdev, i),
    pci_iov_virtfn_devfn(pdev, i));
    if (!pdn) {
    dev_warn(&pdev.dev, "%s: Cannot create firmware data for VF#%d\n",
    __func__, i);
    return core::ptr::null_mut();
    }

// Create the EEH device for the VF
    edev = eeh_dev_init(pdn);
    BUG_ON(!edev);
// FIXME: these should probably be populated by the EEH probe
    edev.physfn = pdev;
    edev.vf_index = i;

    }
    return pci_get_pdn(pdev);
    }
#[no_mangle]
pub unsafe extern "C" fn remove_sriov_vf_pdns(pdev: *mut pci_dev) {
    void remove_sriov_vf_pdns(struct pci_dev *pdev)
    {
    struct pci_dn *parent;
    struct pci_dn *pdn, *tmp;
    int i;
// Only support IOV PF for now
    if (WARN_ON(!pdev.is_physfn))
    return;
// Check if VFs have been populated
    pdn = pci_get_pdn(pdev);
    if (!pdn || !(pdn.flags & PCI_DN_FLAG_IOV_VF))
    return;
    pdn.flags &= ~PCI_DN_FLAG_IOV_VF;
    parent = pci_bus_to_pdn(pdev.bus);
    if (!parent)
    return;
//
// We might introduce flag to pci_dn in future
// so that we can release VF's firmware data in
// a batch mode.
//
    for (i = 0; i < pci_sriov_get_totalvfs(pdev); i++) {
    struct eeh_dev *edev __maybe_unused;
    list_for_each_entry_safe(pdn, tmp,
    &parent.child_list, list) {
    if (pdn.busno != pci_iov_virtfn_bus(pdev, i) ||
    pdn.devfn != pci_iov_virtfn_devfn(pdev, i))
    continue;

//
// Release EEH state for this VF. The PCI core
// has already torn down the pci_dev for this VF, but
// we're responsible to removing the eeh_dev since it
// has the same lifetime as the pci_dn that spawned it.
//
    edev = pdn_to_eeh_dev(pdn);
    if (edev) {
//
// We allocate pci_dn's for the totalvfs count,
// but only the vfs that were activated
// have a configured PE.
//
    if (edev.pe)
    eeh_pe_tree_remove(edev);
    pdn.edev = core::ptr::null_mut();
    kfree(edev);
    }

    if (!list_empty(&pdn.list))
    list_del(&pdn.list);
    kfree(pdn);
    }
    }
    }

    struct pci_dn *pci_add_device_node_info(struct pci_controller *hose,
    struct device_node *dn)
    {
    const __be32 *type = of_get_property(dn, "ibm,pci-config-space-type", core::ptr::null_mut());
    const __be32 *regs;
    struct device_node *parent;
    struct pci_dn *pdn;

    struct eeh_dev *edev;

    pdn = kzalloc_obj(*pdn);
    if (pdn == core::ptr::null_mut())
    return core::ptr::null_mut();
    dn.data = pdn;
    pdn.phb = hose;
    pdn.pe_number = IODA_INVALID_PE;
    regs = of_get_property(dn, "reg", core::ptr::null_mut());
    if (regs) {
    let mut addr: u32 = of_read_number(regs, 1);
// First register entry is addr (00BBSS00)
    pdn.busno = (addr >> 16) & 0xff;
    pdn.devfn = (addr >> 8) & 0xff;
    }
// vendor/device IDs and class code
    regs = of_get_property(dn, "vendor-id", core::ptr::null_mut());
    pdn.vendor_id = regs ? of_read_number(regs, 1) : 0;
    regs = of_get_property(dn, "device-id", core::ptr::null_mut());
    pdn.device_id = regs ? of_read_number(regs, 1) : 0;
    regs = of_get_property(dn, "class-code", core::ptr::null_mut());
    pdn.class_code = regs ? of_read_number(regs, 1) : 0;
// Extended config space
    pdn.pci_ext_config_space = (type && of_read_number(type, 1) == 1);
// Create EEH device

    edev = eeh_dev_init(pdn);
    if (!edev) {
    kfree(pdn);
    return core::ptr::null_mut();
    }

// Attach to parent node
    INIT_LIST_HEAD(&pdn.child_list);
    INIT_LIST_HEAD(&pdn.list);
    parent = of_get_parent(dn);
    pdn.parent = parent ? PCI_DN(parent) : core::ptr::null_mut();
    of_node_put(parent);
    if (pdn.parent)
    list_add_tail(&pdn.list, &pdn.parent.child_list);
    return pdn;
    }
    EXPORT_SYMBOL_GPL(pci_add_device_node_info);
#[no_mangle]
pub unsafe extern "C" fn pci_remove_device_node_info(dn: *mut device_node) {
    void pci_remove_device_node_info(struct device_node *dn)
    {
    struct pci_dn *pdn = dn ? PCI_DN(dn) : core::ptr::null_mut();
    struct device_node *parent;
    struct pci_dev *pdev;

    struct eeh_dev *edev = pdn_to_eeh_dev(pdn);
    if (edev)
    edev.pdn = core::ptr::null_mut();

    if (!pdn)
    return;
    WARN_ON(!list_empty(&pdn.child_list));
    list_del(&pdn.list);
// Drop the parent pci_dn's ref to our backing dt node
    parent = of_get_parent(dn);
    if (parent)
    of_node_put(parent);
//
// At this point we *might* still have a pci_dev that was
// instantiated from this pci_dn. So defer free()ing it until
// the pci_dev's release function is called.
//
    pdev = pci_get_domain_bus_and_slot(pdn.phb.global_number,
    pdn.busno, pdn.devfn);
    if (pdev) {
// NB: pdev has a ref to dn
    pci_dbg(pdev, "marked pdn (from %pOF) as dead\n", dn);
    pdn.flags |= PCI_DN_FLAG_DEAD;
    } else {
    dn.data = core::ptr::null_mut();
    kfree(pdn);
    }
    pci_dev_put(pdev);
    }
    EXPORT_SYMBOL_GPL(pci_remove_device_node_info);
//
// Traverse a device tree stopping each PCI device in the tree.
// This is done depth first.  As each node is processed, a "pre"
// function is called and the children are processed recursively.
//
// The "pre" func returns a value.  If non-zero is returned from
// the "pre" func, the traversal stops and this value is returned.
// This return value is useful when using traverse as a method of
// finding a device.
//
// NOTE: we do not run the func for devices that do not appear to
// be PCI except for the start node which we assume (this is good
// because the start node is often a phb which may be missing PCI
// properties).
// We use the class-code as an indicator. If we run into
// one of these nodes we also assume its siblings are non-pci for
// performance.
//
    void *pci_traverse_device_nodes(struct device_node *start,
    void *(*fn)(struct device_node *, void *),
    void *data)
    {
    struct device_node *dn, *nextdn;
    void *ret;
// We started with a phb, iterate all childs
    for (dn = start.child; dn; dn = nextdn) {
    const __be32 *classp;
    let mut class: u32 = 0;
    nextdn = core::ptr::null_mut();
    classp = of_get_property(dn, "class-code", core::ptr::null_mut());
    if (classp)
    class = of_read_number(classp, 1);
    if (fn) {
    ret = fn(dn, data);
    if (ret)
    return ret;
    }
// If we are a PCI bridge, go down
    if (dn.child && ((class >> 8) == PCI_CLASS_BRIDGE_PCI ||
    (class >> 8) == PCI_CLASS_BRIDGE_CARDBUS))
// Depth first...do children
    nextdn = dn.child;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dn->sibling) -> else {
    else if (dn.sibling)
// ok, try next sibling instead.
    nextdn = dn.sibling;
    if (!nextdn) {
// Walk up to next valid sibling.
    do {
    dn = dn.parent;
    if (dn == start)
    return core::ptr::null_mut();
    } while (dn.sibling == core::ptr::null_mut());
    nextdn = dn.sibling;
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(pci_traverse_device_nodes);
    static void *add_pdn(struct device_node *dn, void *data)
    {
    struct pci_controller *hose = data;
    struct pci_dn *pdn;
    pdn = pci_add_device_node_info(hose, dn);
    if (!pdn)
    return ERR_PTR(-ENOMEM);
    return core::ptr::null_mut();
    }
//
// pci_devs_phb_init_dynamic - setup pci devices under this PHB
// phb: pci-to-host bridge (top-level bridge connecting to cpu)
//
// This routine is called both during boot, (before the memory
// subsystem is set up, before kmalloc is valid) and during the
// dynamic lpar operation of adding a PHB to a running system.
//
#[no_mangle]
pub unsafe extern "C" fn pci_devs_phb_init_dynamic(phb: *mut pci_controller) {
    void pci_devs_phb_init_dynamic(struct pci_controller *phb)
    {
    struct device_node *dn = phb.dn;
    struct pci_dn *pdn;
// PHB nodes themselves must not match
    pdn = pci_add_device_node_info(phb, dn);
    if (pdn) {
    pdn.devfn = pdn.busno = -1;
    pdn.vendor_id = pdn.device_id = pdn.class_code = 0;
    pdn.phb = phb;
    phb.pci_data = pdn;
    }
// Update dn->phb ptrs for new phb and children devices
    pci_traverse_device_nodes(dn, add_pdn, phb);
    }
#[no_mangle]
unsafe extern "C" fn pci_dev_pdn_setup(pdev: *mut pci_dev) {
    static void pci_dev_pdn_setup(struct pci_dev *pdev)
    {
    struct pci_dn *pdn;
    if (pdev.dev.archdata.pci_data)
    return;
// Setup the fast path
    pdn = pci_get_pdn(pdev);
    pdev.dev.archdata.pci_data = pdn;
    }
    DECLARE_PCI_FIXUP_EARLY(PCI_ANY_ID, PCI_ANY_ID, pci_dev_pdn_setup);
