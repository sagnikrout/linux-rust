//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pci-bridge.h
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

//
// PCI controller operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_controller_ops {
    pub pdev): *mut *mut void (dma_dev_setup)(struct pci_dev,
    pub bus): *mut *mut void (dma_bus_setup)(struct pci_bus,
    pub mask): u64,
    pub bus): *mut *mut int (probe_mode)(struct pci_bus,
// Called when pci_enable_device() is called. Returns true to
// allow assignment/enabling of the device.
    pub pdev): *mut *mut bool (enable_device_hook)(struct pci_dev,
    pub pdev): *mut *mut void (disable_device)(struct pci_dev,
    pub pdev): *mut *mut void (release_device)(struct pci_dev,
// Called during PCI resource reassignment
    pub type): c_ulong,
    pub type): c_ulong,
    pub pdev): *mut *mut void (reset_secondary_bus)(struct pci_dev,

    pub type): int nvec, int,
    pub pdev): *mut *mut void (teardown_msi_irqs)(struct pci_dev,

    pub hose): *mut *mut void (shutdown)(struct pci_controller,
    pub pdev): *mut pci_dev,
}

//
// Structure of a PCI controller (host bridge)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_controller {
    pub bus: *mut pci_bus,
    pub is_dynamic: c_char,

    pub node: c_int,

    pub dn: *mut device_node,
    pub list_node: list_head,
    pub parent: *mut device,
    pub first_busno: c_int,
    pub last_busno: c_int,
    pub self_busno: c_int,
    pub busn: resource,
    pub io_base_virt: *mut void __iomem,

    pub io_base_alloc: *mut void __iomem,

    pub io_base_phys: resource_size_t,
    pub pci_io_size: resource_size_t,
// Some machines have a special region to forward the ISA
// "memory" cycles such as VGA memory regions. Left to 0
// if unsupported
//
    pub isa_mem_phys: resource_size_t,
    pub isa_mem_size: resource_size_t,
    pub controller_ops: pci_controller_ops,
    pub ops: *mut pci_ops,
    pub cfg_addr: *mut unsigned int __iomem,
    pub cfg_data: *mut void __iomem,
//
// Used for variants of PCI indirect handling and possible quirks:
// SET_CFG_TYPE - used on 4xx or any PHB that does explicit type0/1
// EXT_REG - provides access to PCI-e extended registers
// SURPRESS_PRIMARY_BUS - we suppress the setting of PCI_PRIMARY_BUS
// on Freescale PCI-e controllers since they used the PCI_PRIMARY_BUS
// to determine which bus number to match on when generating type0
// config cycles
// NO_PCIE_LINK - the Freescale PCI-e controllers have issues with
// hanging if we don't have link and try to do config cycles to
// anything but the PHB.  Only allow talking to the PHB if this is
// set.
// BIG_ENDIAN - cfg_addr is a big endian register
// BROKEN_MRM - the 440EPx/GRx chips have an errata that causes hangs on
// the PLB4.  Effectively disable MRM commands by setting this.
// FSL_CFG_REG_LINK - Freescale controller version in which the PCIe
// link status is in a RC PCIe cfg register (vs being a SoC register)
//
pub const PPC_INDIRECT_TYPE_SET_CFG_TYPE: c_uint = 0x00000001;
pub const PPC_INDIRECT_TYPE_EXT_REG: c_uint = 0x00000002;
pub const PPC_INDIRECT_TYPE_SURPRESS_PRIMARY_BUS: c_uint = 0x00000004;
pub const PPC_INDIRECT_TYPE_NO_PCIE_LINK: c_uint = 0x00000008;
pub const PPC_INDIRECT_TYPE_BIG_ENDIAN: c_uint = 0x00000010;
pub const PPC_INDIRECT_TYPE_BROKEN_MRM: c_uint = 0x00000020;
pub const PPC_INDIRECT_TYPE_FSL_CFG_REG_LINK: c_uint = 0x00000040;
    pub indirect_type: u32,
// Currently, we limit ourselves to 1 IO range and 3 mem
// ranges since the common pci_bus structure can't handle more
//
    pub io_resource: resource,
    pub mem_resources: [resource; 3],
    pub mem_offset: [resource_size_t; 3],
    pub /: *mut *mut int global_number; / PCI domain number,
    pub dma_window_base_cur: resource_size_t,
    pub dma_window_size: resource_size_t,

    pub buid: c_ulong,
    pub pci_data: *mut pci_dn,

    pub private_data: *mut c_void,
// IRQ domain hierarchy
    pub dev_domain: *mut irq_domain,
// iommu_ops support
    pub iommu: iommu_device,
}

// These are used for config access before all the PCI probing

extern "C" {
    pub fn pci_create_OF_bus_map();
}

//
// PCI stuff, for nodes representing PCI devices, pointed to
// by device_node->data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dn {
    pub flags: c_int,
pub const PCI_DN_FLAG_IOV_VF: c_uint = 0x01;
pub const PCI_DN_FLAG_DEAD: c_uint = 0x02    /* Device has been hot-removed */;
    pub /: *mut *mut int busno; / pci bus number,
    pub /: *mut *mut int devfn; / pci device and function number,
    pub /: *mut *mut int vendor_id; / Vendor ID,
    pub /: *mut *mut int device_id; / Device ID,
    pub /: *mut *mut int class_code; / Device class code,
    pub parent: *mut pci_dn,
    pub /: *mut *mut *mut pci_controller phb; / for pci devices,
    pub /: *mut *mut *mut iommu_table_group table_group; / for phb's or bridges,
    pub /: *mut *mut int pci_ext_config_space; / for pci devices,

    pub /: *mut *mut *mut eeh_dev edev; / eeh device,

pub const IODA_INVALID_PE: c_uint = 0xFFFFFFFF;
    pub pe_number: c_uint,

    pub /: *mut *mut u16 vfs_expanded; / number of VFs IOV BAR expanded,
    pub enabled*/: *mut *mut u16 num_vfs; / number of VFs,
    pub /: *mut *mut *mut unsigned int pe_num_map; / PE# for the first VF PE or array,
    pub /: *mut *mut bool m64_single_mode; / Use M64 BAR in Single Mode,

    pub /: *mut *mut *mut int (m64_map)[PCI_SRIOV_NUM_BARS]; / Only used on powernv,
    pub /: *mut *mut int last_allow_rc; / Only used on pseries,

    pub /: *mut *mut int mps; / Maximum Payload Size,
    pub child_list: list_head,
    pub list: list_head,
    pub holes: [resource; PCI_SRIOV_NUM_BARS],
}

// Get the pointer to a device_node's pci_dn

extern "C" {
    pub fn pci_remove_device_node_info(dn: *mut device_node);
}

extern "C" {
    pub fn remove_sriov_vf_pdns(pdev: *mut pci_dev);
}

// Find the bus corresponding to the indicated device node
// Remove all of the PCI devices under this bus
extern "C" {
    pub fn pci_hp_remove_devices(bus: *mut pci_bus);
}
// Discover new pci devices under this bus, and add them
extern "C" {
    pub fn pci_hp_add_devices(bus: *mut pci_bus);
}
extern "C" {
    pub fn pcibios_unmap_io_space(bus: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn pcibios_map_io_space(bus: *mut pci_bus) -> c_int;
}

// Get the PCI host controller for an OF device
// Fill up host controller resources from the OF node
// Allocate & free a PCI host bridge structure
extern "C" {
    pub fn pcibios_free_controller(phb: *mut pci_controller);
}
extern "C" {
    pub fn pcibios_free_controller_deferred(bridge: *mut pci_host_bridge);
}

extern "C" {
    pub fn pcibios_vaddr_is_ioport(address: *mut void __iomem) -> c_int;
}

