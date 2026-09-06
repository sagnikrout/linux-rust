//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-epc.h
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


// SPDX-License-Identifier: GPL-2.0
//
// PCI Endpoint *Controller* (EPC) header file
//
// Copyright (C) 2017 Texas Instruments
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_interface_type {
    UNKNOWN_INTERFACE = -1,
    PRIMARY_INTERFACE,
    SECONDARY_INTERFACE,
}

//
// struct pci_epc_map - information about EPC memory for mapping a RC PCI
// address range
// @pci_addr: start address of the RC PCI address range to map
// @pci_size: size of the RC PCI address range mapped from @pci_addr
// @map_pci_addr: RC PCI address used as the first address mapped (may be lower
// than @pci_addr)
// @map_size: size of the controller memory needed for mapping the RC PCI address
// range @map_pci_addr..@pci_addr+@pci_size
// @phys_base: base physical address of the allocated EPC memory for mapping the
// RC PCI address range
// @phys_addr: physical address at which @pci_addr is mapped
// @virt_base: base virtual address of the allocated EPC memory for mapping the
// RC PCI address range
// @virt_addr: virtual address at which @pci_addr is mapped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_map {
    pub pci_addr: u64,
    pub pci_size: usize,
    pub map_pci_addr: u64,
    pub map_size: usize,
    pub phys_base: phys_addr_t,
    pub phys_addr: phys_addr_t,
    pub virt_base: *mut void __iomem,
    pub virt_addr: *mut void __iomem,
}

//
// enum pci_epc_aux_resource_type - auxiliary resource type identifiers
// @PCI_EPC_AUX_DOORBELL_MMIO: Doorbell MMIO, that might be outside the DMA
// controller register window
//
// EPC backends may expose auxiliary blocks (e.g. DMA engines) by mapping their
// register windows and descriptor memories into BAR space. This enum
// identifies the type of each exposable resource.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_aux_resource_type {
    PCI_EPC_AUX_DOORBELL_MMIO,
}

//
// struct pci_epc_aux_resource - a physical auxiliary resource that may be
// exposed for peer use
// @type:       resource type, see enum pci_epc_aux_resource_type
// @phys_addr:  physical base address of the resource
// @size:       size of the resource in bytes
// @bar:        BAR number where this resource is already exposed to the RC
// (NO_BAR if not)
// @bar_offset: offset within @bar where the resource starts (valid iff
// @bar != NO_BAR)
// @u:          type-specific metadata
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_aux_resource {
    pub type: pci_epc_aux_resource_type,
    pub phys_addr: phys_addr_t,
    pub size: resource_size_t,
    pub bar: pci_barno,
    pub bar_offset: resource_size_t,
// PCI_EPC_AUX_DOORBELL_MMIO
    pub /: *mut *mut int irq; / IRQ number for the doorbell handler,
    pub /: *mut *mut u32 data; / write value to ring the doorbell,
    pub db_mmio: },
    pub u: },
}

//
// struct pci_epc_ops - set of function pointers for performing EPC operations
// @write_header: ops to populate configuration space header
// @set_bar: ops to configure the BAR
// @clear_bar: ops to reset the BAR
// @align_addr: operation to get the mapping address, mapping size and offset
// into a controller memory window needed to map an RC PCI address
// region
// @map_addr: ops to map CPU address to PCI address
// @unmap_addr: ops to unmap CPU address and PCI address
// @set_msi: ops to set the requested number of MSI interrupts in the MSI
// capability register
// @get_msi: ops to get the number of MSI interrupts allocated by the RC from
// the MSI capability register
// @set_msix: ops to set the requested number of MSI-X interrupts in the
// MSI-X capability register
// @get_msix: ops to get the number of MSI-X interrupts allocated by the RC
// from the MSI-X capability register
// @raise_irq: ops to raise a legacy, MSI or MSI-X interrupt
// @map_msi_irq: ops to map physical address to MSI address and return MSI data
// @start: ops to start the PCI link
// @stop: ops to stop the PCI link
// @get_features: ops to get the features supported by the EPC
// @get_aux_resources_count: ops to get the number of controller-owned
// auxiliary resources
// @get_aux_resources: ops to retrieve controller-owned auxiliary resources
// @owner: the module owner containing the ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_ops {
    pub hdr): *mut pci_epf_header,
    pub epf_bar): *mut pci_epf_bar,
    pub epf_bar): *mut pci_epf_bar,
    pub offset): *mut usize,
    pub size): phys_addr_t addr, u64 pci_addr, size_t,
    pub addr): phys_addr_t,
    pub nr_irqs): u8,
    pub vfunc_no): *mut *mut *mut int (get_msi)(struct pci_epc epc, u8 func_no, u8,
    pub offset): u16 nr_irqs, enum pci_barno, u32,
    pub vfunc_no): *mut *mut *mut int (get_msix)(struct pci_epc epc, u8 func_no, u8,
    pub interrupt_num): unsigned int type, u16,
    pub msi_addr_offset): *mut u32,
    pub epc): *mut *mut int (start)(struct pci_epc,
    pub epc): *mut *mut void (stop)(struct pci_epc,
    pub vfunc_no): u8 func_no, u8,
    pub vfunc_no): u8,
    pub num_resources): c_int,
    pub owner: *mut module,
}

//
// struct pci_epc_mem_window - address window of the endpoint controller
// @phys_base: physical base address of the PCI address window
// @size: the size of the PCI address window
// @page_size: size of each page
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_mem_window {
    pub phys_base: phys_addr_t,
    pub size: usize,
    pub page_size: usize,
}

//
// struct pci_epc_mem - address space of the endpoint controller
// @window: address window of the endpoint controller
// @bitmap: bitmap to manage the PCI address space
// @pages: number of bits representing the address region
// @lock: mutex to protect bitmap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_mem {
    pub window: pci_epc_mem_window,
    pub bitmap: *mut c_ulong,
    pub pages: c_int,
// mutex to protect against concurrent access for memory allocation
    pub lock: mutex,
}

//
// struct pci_epc - represents the PCI EPC device
// @dev: PCI EPC device
// @pci_epf: list of endpoint functions present in this EPC device
// @list_lock: Mutex for protecting pci_epf list
// @ops: function pointers for performing endpoint operations
// @windows: array of address space of the endpoint controller
// @mem: first window of the endpoint controller, which corresponds to
// default address space of the endpoint controller supporting
// single window.
// @num_windows: number of windows supported by device
// @max_functions: max number of functions that can be configured in this EPC
// @max_vfs: Array indicating the maximum number of virtual functions that can
// be associated with each physical function
// @group: configfs group representing the PCI EPC device
// @lock: mutex to protect pci_epc ops
// @function_num_map: bitmap to manage physical function number
// @domain_nr: PCI domain number of the endpoint controller
// @init_complete: flag to indicate whether the EPC initialization is complete
// or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc {
    pub dev: device,
    pub pci_epf: list_head,
    pub list_lock: mutex,
    pub ops: *const pci_epc_ops,
    pub windows: *mut pci_epc_mem,
    pub mem: *mut pci_epc_mem,
    pub num_windows: c_uint,
    pub max_functions: u8,
    pub max_vfs: *mut u8,
    pub group: *mut config_group,
// mutex to protect against concurrent access of EP controller
    pub lock: mutex,
    pub function_num_map: c_ulong,
    pub domain_nr: c_int,
    pub init_complete: bool,
}

//
// enum pci_epc_bar_type - configurability of endpoint BAR
// @BAR_PROGRAMMABLE: The BAR mask can be configured by the EPC.
// @BAR_FIXED: The BAR mask is fixed by the hardware.
// @BAR_RESIZABLE: The BAR implements the PCI-SIG Resizable BAR Capability.
// NOTE: An EPC driver can currently only set a single supported
// size.
// @BAR_RESERVED: Used for HW-backed BARs (e.g. MSI-X table, DMA regs). The BAR
// should not be disabled by an EPC driver. The BAR should not be
// reprogrammed by an EPF driver. An EPF driver is allowed to
// disable the BAR if absolutely necessary. (However, right now
// there is no EPC operation to disable a BAR that has not been
// programmed using pci_epc_set_bar().)
// @BAR_DISABLED: The BAR should be disabled by an EPC driver. The BAR will be
// unavailable to an EPF driver.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_bar_type {
    BAR_PROGRAMMABLE = 0,
    BAR_FIXED,
    BAR_RESIZABLE,
    BAR_RESERVED,
    BAR_DISABLED,
}

//
// enum pci_epc_bar_rsvd_region_type - type of a fixed subregion behind a BAR
// @PCI_EPC_BAR_RSVD_DMA_CTRL_MMIO: Integrated DMA controller MMIO window
// @PCI_EPC_BAR_RSVD_MSIX_TBL_RAM: MSI-X table structure
// @PCI_EPC_BAR_RSVD_MSIX_PBA_RAM: MSI-X PBA structure
//
// BARs marked BAR_RESERVED are owned by the SoC/EPC hardware and must not be
// reprogrammed by EPF drivers. Some of them still expose fixed subregions that
// EPFs may want to reference (e.g. embedded doorbell fallback).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epc_bar_rsvd_region_type {
    PCI_EPC_BAR_RSVD_DMA_CTRL_MMIO = 0,
    PCI_EPC_BAR_RSVD_MSIX_TBL_RAM,
    PCI_EPC_BAR_RSVD_MSIX_PBA_RAM,
}

//
// struct pci_epc_bar_rsvd_region - fixed subregion behind a BAR
// @type: reserved region type
// @offset: offset within the BAR aperture
// @size: size of the reserved region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_bar_rsvd_region {
    pub type: pci_epc_bar_rsvd_region_type,
    pub offset: resource_size_t,
    pub size: resource_size_t,
}

//
// struct pci_epc_bar_desc - hardware description for a BAR
// @type: the type of the BAR
// @fixed_size: the fixed size, only applicable if type is BAR_FIXED_MASK.
// @only_64bit: if true, an EPF driver is not allowed to choose if this BAR
// should be configured as 32-bit or 64-bit, the EPF driver must
// configure this BAR as 64-bit.
// @nr_rsvd_regions: number of fixed subregions described for BAR_RESERVED
// @rsvd_regions: fixed subregions behind BAR_RESERVED
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_bar_desc {
    pub type: pci_epc_bar_type,
    pub fixed_size: u64,
    pub only_64bit: bool,
    pub nr_rsvd_regions: u8,
    pub rsvd_regions: *const pci_epc_bar_rsvd_region,
}

//
// struct pci_epc_features - features supported by a EPC device per function
// @linkup_notifier: indicate if the EPC device can notify EPF driver on link up
// @dynamic_inbound_mapping: indicate if the EPC device supports updating
// inbound mappings for an already configured BAR
// (i.e. allow calling pci_epc_set_bar() again
// without first calling pci_epc_clear_bar())
// @subrange_mapping: indicate if the EPC device can map inbound subranges for a
// BAR. This feature depends on @dynamic_inbound_mapping
// feature.
// @msi_capable: indicate if the endpoint function has MSI capability
// @msix_capable: indicate if the endpoint function has MSI-X capability
// @intx_capable: indicate if the endpoint can raise INTx interrupts
// @bar: array specifying the hardware description for each BAR
// @align: alignment size required for BAR buffer allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_features {
    pub 1: unsigned int linkup_notifier :,
    pub 1: unsigned int dynamic_inbound_mapping :,
    pub 1: unsigned int subrange_mapping :,
    pub 1: unsigned int msi_capable :,
    pub 1: unsigned int msix_capable :,
    pub 1: unsigned int intx_capable :,
    pub bar: [pci_epc_bar_desc; PCI_STD_NUM_BARS],
    pub align: usize,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &epc->dev) -> return;
}
extern "C" {
    pub fn pci_epc_destroy(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_linkup(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_linkdown(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_init_notify(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_notify_pending_init(epc: *mut pci_epc, epf: *mut pci_epf);
}
extern "C" {
    pub fn pci_epc_deinit_notify(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_bus_master_enable_notify(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_bar_size_to_rebar_cap(size: usize, cap: *mut u32) -> c_int;
}
extern "C" {
    pub fn pci_epc_set_msi(epc: *mut pci_epc, func_no: u8, vfunc_no: u8, nr_irqs: u8) -> c_int;
}
extern "C" {
    pub fn pci_epc_get_msi(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> c_int;
}
extern "C" {
    pub fn pci_epc_get_msix(epc: *mut pci_epc, func_no: u8, vfunc_no: u8) -> c_int;
}
extern "C" {
    pub fn pci_epc_start(epc: *mut pci_epc) -> c_int;
}
extern "C" {
    pub fn pci_epc_stop(epc: *mut pci_epc);
}
// epc_features, enum pci_barno bar);
extern "C" {
    pub fn pci_epc_put(epc: *mut pci_epc);
}
extern "C" {
    pub fn pci_epc_mem_exit(epc: *mut pci_epc);
}

