//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/pci.c
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
// Copyright (C) 2001 Dave Engebretsen, IBM Corporation
// Copyright (C) 2003 Anton Blanchard <anton@au.ibm.com>, IBM
//
// pSeries specific routines for PCI.
//

pub const MAX_VFS_FOR_MAP_PE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pe_map_bar_entry {
    pub /: *mut *mut __be64 bar; / Input: Virtual Function BAR,
    pub /: *mut *mut __be16 rid; / Input: Virtual Function Router ID,
    pub /: *mut *mut __be16 pe_num; / Output: Virtual Function PE Number,
    pub /: *mut *mut __be32 reserved; / Reserved Space,
}

    static int pseries_send_map_pe(struct pci_dev *pdev, u16 num_vfs,
    struct pe_map_bar_entry *vf_pe_array)
    {
    struct pci_dn *pdn;
    int rc;
    unsigned long buid, addr;
    let mut ibm_map_pes: c_int = rtas_function_token(RTAS_FN_IBM_OPEN_SRIOV_MAP_PE_NUMBER);
    if (ibm_map_pes == RTAS_UNKNOWN_SERVICE)
    return -EINVAL;
    pdn = pci_get_pdn(pdev);
    addr = rtas_config_addr(pdn.busno, pdn.devfn, 0);
    buid = pdn.phb.buid;
    spin_lock(&rtas_data_buf_lock);
    memcpy(rtas_data_buf, vf_pe_array,
    RTAS_DATA_BUF_SIZE);
    rc = rtas_call(ibm_map_pes, 5, 1, core::ptr::null_mut(), addr,
    BUID_HI(buid), BUID_LO(buid),
    rtas_data_buf,
    num_vfs * sizeof(struct pe_map_bar_entry));
    memcpy(vf_pe_array, rtas_data_buf, RTAS_DATA_BUF_SIZE);
    spin_unlock(&rtas_data_buf_lock);
    if (rc)
    dev_err(&pdev.dev,
    "%s: Failed to associate pes PE#%lx, rc=%x\n",
    __func__,  addr, rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pseries_set_pe_num(pdev: *mut pci_dev, vf_index: u16, pe_num: __be16) {
    static void pseries_set_pe_num(struct pci_dev *pdev, u16 vf_index, __be16 pe_num)
    {
    struct pci_dn *pdn;
    pdn = pci_get_pdn(pdev);
    pdn.pe_num_map[vf_index] = be16_to_cpu(pe_num);
    dev_dbg(&pdev.dev, "VF %04x:%02x:%02x.%x associated with PE#%x\n",
    pci_domain_nr(pdev.bus),
    pdev.bus.number,
    PCI_SLOT(pci_iov_virtfn_devfn(pdev, vf_index)),
    PCI_FUNC(pci_iov_virtfn_devfn(pdev, vf_index)),
    pdn.pe_num_map[vf_index]);
    }
#[no_mangle]
unsafe extern "C" fn pseries_associate_pes(pdev: *mut pci_dev, num_vfs: u16) -> c_int {
    static int pseries_associate_pes(struct pci_dev *pdev, u16 num_vfs)
    {
    struct pci_dn *pdn;
    int i, rc, vf_index;
    struct pe_map_bar_entry *vf_pe_array;
    struct resource *res;
    u64 size;
    vf_pe_array = kzalloc(RTAS_DATA_BUF_SIZE, GFP_KERNEL);
    if (!vf_pe_array)
    return -ENOMEM;
    pdn = pci_get_pdn(pdev);
// create firmware structure to associate pes
    for (vf_index = 0; vf_index < num_vfs; vf_index++) {
    pdn.pe_num_map[vf_index] = IODA_INVALID_PE;
    for (i = 0; i < PCI_SRIOV_NUM_BARS; i++) {
    res = &pdev.resource[i + PCI_IOV_RESOURCES];
    if (!res.parent)
    continue;
    size = pcibios_iov_resource_alignment(pdev, i +
    PCI_IOV_RESOURCES);
    vf_pe_array[vf_index].bar =
    cpu_to_be64(res.start + size * vf_index);
    vf_pe_array[vf_index].rid =
    cpu_to_be16((pci_iov_virtfn_bus(pdev, vf_index)
    << 8) | pci_iov_virtfn_devfn(pdev,
    vf_index));
    vf_pe_array[vf_index].pe_num =
    cpu_to_be16(IODA_INVALID_PE);
    }
    }
    rc = pseries_send_map_pe(pdev, num_vfs, vf_pe_array);
// Only zero is success
    if (!rc)
    for (vf_index = 0; vf_index < num_vfs; vf_index++)
    pseries_set_pe_num(pdev, vf_index,
    vf_pe_array[vf_index].pe_num);
    kfree(vf_pe_array);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pseries_pci_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> c_int {
    static int pseries_pci_sriov_enable(struct pci_dev *pdev, u16 num_vfs)
    {
    struct pci_dn         *pdn;
    int                    rc;
    const int *max_vfs;
    int max_config_vfs;
    struct device_node *dn = pci_device_to_OF_node(pdev);
    max_vfs = of_get_property(dn, "ibm,number-of-configurable-vfs", core::ptr::null_mut());
    if (!max_vfs)
    return -EINVAL;
// First integer stores max config
    max_config_vfs = of_read_number(&max_vfs[0], 1);
    if (max_config_vfs < num_vfs || num_vfs > MAX_VFS_FOR_MAP_PE) {
    dev_err(&pdev.dev,
    "Num VFs %x > %x Configurable VFs\n",
    num_vfs, (num_vfs > MAX_VFS_FOR_MAP_PE) ?
    MAX_VFS_FOR_MAP_PE : max_config_vfs);
    return -EINVAL;
    }
    pdn = pci_get_pdn(pdev);
    pdn.pe_num_map = kmalloc_objs(*pdn.pe_num_map, num_vfs);
    if (!pdn.pe_num_map)
    return -ENOMEM;
    rc = pseries_associate_pes(pdev, num_vfs);
// Anything other than zero is failure
    if (rc) {
    dev_err(&pdev.dev, "Failure to enable sriov: %x\n", rc);
    kfree(pdn.pe_num_map);
    } else {
    pci_vf_drivers_autoprobe(pdev, false);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn pseries_pcibios_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> c_int {
    static int pseries_pcibios_sriov_enable(struct pci_dev *pdev, u16 num_vfs)
    {
// Allocate PCI data
    add_sriov_vf_pdns(pdev);
    return pseries_pci_sriov_enable(pdev, num_vfs);
    }
#[no_mangle]
unsafe extern "C" fn pseries_pcibios_sriov_disable(pdev: *mut pci_dev) -> c_int {
    static int pseries_pcibios_sriov_disable(struct pci_dev *pdev)
    {
    struct pci_dn         *pdn;
    pdn = pci_get_pdn(pdev);
// Releasing pe_num_map
    kfree(pdn.pe_num_map);
// Release PCI data
    remove_sriov_vf_pdns(pdev);
    pci_vf_drivers_autoprobe(pdev, true);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pSeries_request_regions() -> void __init {
    static void __init pSeries_request_regions(void)
    {
    if (!isa_io_base)
    return;
    request_region(0x20,0x20,"pic1");
    request_region(0xa0,0x20,"pic2");
    request_region(0x00,0x20,"dma1");
    request_region(0x40,0x20,"timer");
    request_region(0x80,0x10,"dma page reg");
    request_region(0xc0,0x20,"dma2");
    }
#[no_mangle]
pub unsafe extern "C" fn pSeries_final_fixup() -> void __init {
    void __init pSeries_final_fixup(void)
    {
    pSeries_request_regions();
    eeh_show_enabled();

    ppc_md.pcibios_sriov_enable = pseries_pcibios_sriov_enable;
    ppc_md.pcibios_sriov_disable = pseries_pcibios_sriov_disable;

    }
//
// Assume the winbond 82c105 is the IDE controller on a
// p610/p615/p630. We should probably be more careful in case
// someone tries to plug in a similar adapter.
//
#[no_mangle]
unsafe extern "C" fn fixup_winbond_82c105(dev: *mut *mut pci_dev) {
    static void fixup_winbond_82c105(struct pci_dev* dev)
    {
    struct resource *r;
    unsigned int reg;
    if (!machine_is(pseries))
    return;
    printk("Using INTC for W82c105 IDE controller.\n");
    pci_read_config_dword(dev, 0x40, &reg);
// Enable LEGIRQ to use INTC instead of ISA interrupts
    pci_write_config_dword(dev, 0x40, reg | (1<<11));
    pci_dev_for_each_resource(dev, r) {
// zap the 2nd function of the winbond chip
    if (dev.bus.number == 0 && dev.devfn == 0x81 &&
    r.flags & IORESOURCE_IO)
    r.flags &= ~IORESOURCE_IO;
    if (r.start == 0 && r.end) {
    r.flags = 0;
    r.end = 0;
    }
    }
    }
    DECLARE_PCI_FIXUP_HEADER(PCI_VENDOR_ID_WINBOND, PCI_DEVICE_ID_WINBOND_82C105,
    fixup_winbond_82c105);
#[no_mangle]
unsafe extern "C" fn prop_to_pci_speed(prop: u32) -> enum pci_bus_speed {
    static enum pci_bus_speed prop_to_pci_speed(u32 prop)
    {
    switch (prop) {
    case 0x01:
    return PCIE_SPEED_2_5GT;
    case 0x02:
    return PCIE_SPEED_5_0GT;
    case 0x04:
    return PCIE_SPEED_8_0GT;
    case 0x08:
    return PCIE_SPEED_16_0GT;
    case 0x10:
    return PCIE_SPEED_32_0GT;
    default:
    pr_debug("Unexpected PCI link speed property value\n");
    return PCI_SPEED_UNKNOWN;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pseries_root_bridge_prepare(bridge: *mut pci_host_bridge) -> c_int {
    int pseries_root_bridge_prepare(struct pci_host_bridge *bridge)
    {
    struct device_node *dn, *pdn;
    struct pci_bus *bus;
    u32 pcie_link_speed_stats[2];
    int rc;
    bus = bridge.bus;
// Rely on the pcibios_free_controller_deferred() callback.
    pci_set_host_bridge_release(bridge, pcibios_free_controller_deferred,
    (void *) pci_bus_to_host(bus));
    dn = pcibios_get_phb_of_node(bus);
    if (!dn)
    return 0;
    for (pdn = dn; pdn != core::ptr::null_mut(); pdn = of_get_next_parent(pdn)) {
    rc = of_property_read_u32_array(pdn,
    "ibm,pcie-link-speed-stats",
    &pcie_link_speed_stats[0], 2);
    if (!rc)
    break;
    }
    of_node_put(pdn);
    if (rc) {
    pr_debug("no ibm,pcie-link-speed-stats property\n");
    return 0;
    }
    bus.max_bus_speed = prop_to_pci_speed(pcie_link_speed_stats[0]);
    bus.cur_bus_speed = prop_to_pci_speed(pcie_link_speed_stats[1]);
    return 0;
    }
