//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/pci_dlpar.c
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
// PCI Dynamic LPAR, PCI Hot Plug and PCI EEH recovery code
// for RPA-compliant PPC64 platform.
// Copyright (C) 2003 Linda Xie <lxie@us.ibm.com>
// Copyright (C) 2005 International Business Machines
//
// Updates, 2005, John Rose <johnrose@austin.ibm.com>
// Updates, 2005, Linas Vepstas <linas@austin.ibm.com>
//

    struct pci_controller *init_phb_dynamic(struct device_node *dn)
    {
    struct pci_controller *phb;
    int nid;
    pr_debug("PCI: Initializing new hotplug PHB %pOF\n", dn);
    nid = of_node_to_nid(dn);
    if (likely((nid) >= 0)) {
    if (!node_online(nid)) {
    if (register_node(nid)) {
    pr_err("PCI: Failed to register node %d\n", nid);
    } else {
    update_numa_distance(dn);
    node_set_online(nid);
    }
    }
    }
    phb = pcibios_alloc_controller(dn);
    if (!phb)
    return core::ptr::null_mut();
    rtas_setup_phb(phb);
    pci_process_bridge_OF_ranges(phb, dn, 0);
    phb.controller_ops = pseries_pci_controller_ops;
    pci_devs_phb_init_dynamic(phb);
    pseries_msi_allocate_domains(phb);
    ppc_iommu_register_device(phb);
// Create EEH devices for the PHB
    eeh_phb_pe_create(phb);
    if (dn.child)
    pseries_eeh_init_edev_recursive(PCI_DN(dn));
    pcibios_scan_phb(phb);
    pcibios_finish_adding_to_bus(phb.bus);
    return phb;
    }
    EXPORT_SYMBOL_GPL(init_phb_dynamic);
// RPA-specific bits for removing PHBs
#[no_mangle]
pub unsafe extern "C" fn remove_phb_dynamic(phb: *mut pci_controller) -> c_int {
    int remove_phb_dynamic(struct pci_controller *phb)
    {
    struct pci_bus *b = phb.bus;
    struct pci_host_bridge *host_bridge = to_pci_host_bridge(b.bridge);
    struct resource *res;
    int rc, i;
    pr_debug("PCI: Removing PHB %04x:%02x...\n",
    pci_domain_nr(b), b.number);
// We cannot to remove a root bus that has children
    if (!(list_empty(&b.children) && list_empty(&b.devices)))
    return -EBUSY;
// We -know- there aren't any child devices anymore at this stage
// and thus, we can safely unmap the IO space as it's not in use
//
    res = &phb.io_resource;
    if (res.flags & IORESOURCE_IO) {
    rc = pcibios_unmap_io_space(b);
    if (rc) {
    printk(KERN_ERR "%s: failed to unmap IO on bus %s\n",
    __func__, b.name);
    return 1;
    }
    }
    ppc_iommu_unregister_device(phb);
    pseries_msi_free_domains(phb);
// Keep a reference so phb isn't freed yet
    get_device(&host_bridge.dev);
// Remove the PCI bus and unregister the bridge device from sysfs
    phb.bus = core::ptr::null_mut();
    pci_remove_bus(b);
    host_bridge.bus = core::ptr::null_mut();
    device_unregister(&host_bridge.dev);
// Now release the IO resource
    if (res.flags & IORESOURCE_IO)
    release_resource(res);
// Release memory resources
    for (i = 0; i < 3; ++i) {
    res = &phb.mem_resources[i];
    if (!(res.flags & IORESOURCE_MEM))
    continue;
    release_resource(res);
    }
//
// The pci_controller data structure is freed by
// the pcibios_free_controller_deferred() callback;
// see pseries_root_bridge_prepare().
//
    put_device(&host_bridge.dev);
    return 0;
    }
    EXPORT_SYMBOL_GPL(remove_phb_dynamic);
