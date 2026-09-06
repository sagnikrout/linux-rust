//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-epf.h
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
// PCI Endpoint *Function* (EPF) header file
//
// Copyright (C) 2017 Texas Instruments
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_barno {
    NO_BAR = -1,
    BAR_0,
    BAR_1,
    BAR_2,
    BAR_3,
    BAR_4,
    BAR_5,
}

//
// struct pci_epf_header - represents standard configuration header
// @vendorid: identifies device manufacturer
// @deviceid: identifies a particular device
// @revid: specifies a device-specific revision identifier
// @progif_code: identifies a specific register-level programming interface
// @subclass_code: identifies more specifically the function of the device
// @baseclass_code: broadly classifies the type of function the device performs
// @cache_line_size: specifies the system cacheline size in units of DWORDs
// @subsys_vendor_id: vendor of the add-in card or subsystem
// @subsys_id: ID specific to vendor
// @interrupt_pin: interrupt pin the device (or device function) uses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_header {
    pub vendorid: u16,
    pub deviceid: u16,
    pub revid: u8,
    pub progif_code: u8,
    pub subclass_code: u8,
    pub baseclass_code: u8,
    pub cache_line_size: u8,
    pub subsys_vendor_id: u16,
    pub subsys_id: u16,
    pub interrupt_pin: pci_interrupt_pin,
}

//
// struct pci_epf_ops - set of function pointers for performing EPF operations
// @bind: ops to perform when a EPC device has been bound to EPF device
// @unbind: ops to perform when a binding has been lost between a EPC device
// and EPF device
// @add_cfs: ops to initialize function-specific configfs attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_ops {
    pub epf): *mut *mut int (bind)(struct pci_epf,
    pub epf): *mut *mut void (unbind)(struct pci_epf,
    pub group): *mut config_group,
}

//
// struct pci_epc_event_ops - Callbacks for capturing the EPC events
// @epc_init: Callback for the EPC initialization complete event
// @epc_deinit: Callback for the EPC deinitialization event
// @link_up: Callback for the EPC link up event
// @link_down: Callback for the EPC link down event
// @bus_master_enable: Callback for the EPC Bus Master Enable event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epc_event_ops {
    pub epf): *mut *mut int (epc_init)(struct pci_epf,
    pub epf): *mut *mut void (epc_deinit)(struct pci_epf,
    pub epf): *mut *mut int (link_up)(struct pci_epf,
    pub epf): *mut *mut int (link_down)(struct pci_epf,
    pub epf): *mut *mut int (bus_master_enable)(struct pci_epf,
}

//
// struct pci_epf_driver - represents the PCI EPF driver
// @probe: ops to perform when a new EPF device has been bound to the EPF driver
// @remove: ops to perform when the binding between the EPF device and EPF
// driver is broken
// @driver: PCI EPF driver
// @ops: set of function pointers for performing EPF operations
// @owner: the owner of the module that registers the PCI EPF driver
// @epf_group: list of configfs group corresponding to the PCI EPF driver
// @id_table: identifies EPF devices for probing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_driver {
    pub id): *const pci_epf_device_id,
    pub epf): *mut *mut void (remove)(struct pci_epf,
    pub driver: device_driver,
    pub ops: *const pci_epf_ops,
    pub owner: *mut module,
    pub epf_group: list_head,
    pub id_table: *const pci_epf_device_id,
}

//
// struct pci_epf_bar_submap - BAR subrange for inbound mapping
// @phys_addr: target physical/DMA address for this subrange
// @size: the size of the subrange to be mapped
//
// When pci_epf_bar.num_submap is >0, pci_epf_bar.submap describes the
// complete BAR layout. This allows an EPC driver to program multiple
// inbound translation windows for a single BAR when supported by the
// controller. The array order defines the BAR layout (submap[0] at offset
// 0, and each immediately follows the previous one).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_bar_submap {
    pub phys_addr: dma_addr_t,
    pub size: usize,
}

//
// struct pci_epf_bar - represents the BAR of EPF device
// @phys_addr: physical address that should be mapped to the BAR
// @addr: virtual address corresponding to the @phys_addr
// @size: the size of the address space present in BAR
// @mem_size: the size actually allocated to accommodate the iATU alignment
// requirement
// @barno: BAR number
// @flags: flags that are set for the BAR
// @num_submap: number of entries in @submap
// @submap: array of subrange descriptors allocated by the caller. See
// struct pci_epf_bar_submap for the semantics in detail.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_bar {
    pub phys_addr: dma_addr_t,
    pub addr: *mut c_void,
    pub size: usize,
    pub mem_size: usize,
    pub barno: pci_barno,
    pub flags: c_int,
// Optional sub-range mapping
    pub num_submap: c_uint,
    pub submap: *mut pci_epf_bar_submap,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_epf_doorbell_type {
    PCI_EPF_DOORBELL_MSI = 0,
    PCI_EPF_DOORBELL_EMBEDDED,
}

//
// struct pci_epf_doorbell_msg - represents doorbell message
// @msg: Doorbell address/data pair to be mapped into BAR space.
// For MSI-backed doorbells this is the MSI message, while for
// "embedded" doorbells this represents an MMIO write that asserts
// an interrupt on the EP side.
// @virq: IRQ number of this doorbell message
// @irq_flags: Required flags for request_irq()/request_threaded_irq().
// Callers may OR-in additional flags (e.g. IRQF_ONESHOT).
// @type: Doorbell type.
// @bar: BAR number where the doorbell target is already exposed to the RC
// (NO_BAR if not)
// @offset: offset within @bar for the doorbell target (valid iff
// @bar != NO_BAR)
// @iova_base: Internal: base DMA address returned by dma_map_resource() for the
// embedded doorbell MMIO window (used only for unmapping). Valid
// when @type is PCI_EPF_DOORBELL_EMBEDDED and @iova_size is
// non-zero.
// @iova_size: Internal: size of the dma_map_resource() mapping at @iova_base.
// Zero when no mapping was created (e.g. pre-exposed fixed BAR).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_doorbell_msg {
    pub msg: msi_msg,
    pub virq: c_int,
    pub irq_flags: c_ulong,
    pub type: pci_epf_doorbell_type,
    pub bar: pci_barno,
    pub offset: resource_size_t,
    pub iova_base: dma_addr_t,
    pub iova_size: usize,
}

//
// struct pci_epf - represents the PCI EPF device
// @dev: the PCI EPF device
// @name: the name of the PCI EPF device
// @header: represents standard configuration header
// @bar: represents the BAR of EPF device
// @msi_interrupts: number of MSI interrupts required by this function
// @msix_interrupts: number of MSI-X interrupts required by this function
// @func_no: unique (physical) function number within this endpoint device
// @vfunc_no: unique virtual function number within a physical function
// @epc: the EPC device to which this EPF device is bound
// @epf_pf: the physical EPF device to which this virtual EPF device is bound
// @driver: the EPF driver to which this EPF device is bound
// @id: pointer to the EPF device ID
// @list: to add pci_epf as a list of PCI endpoint functions to pci_epc
// @lock: mutex to protect pci_epf_ops
// @sec_epc: the secondary EPC device to which this EPF device is bound
// @sec_epc_list: to add pci_epf as list of PCI endpoint functions to secondary
// EPC device
// @sec_epc_bar: represents the BAR of EPF device associated with secondary EPC
// @sec_epc_func_no: unique (physical) function number within the secondary EPC
// @group: configfs group associated with the EPF device
// @is_bound: indicates if bind notification to function driver has been invoked
// @is_vf: true - virtual function, false - physical function
// @vfunction_num_map: bitmap to manage virtual function number
// @pci_vepf: list of virtual endpoint functions associated with this function
// @event_ops: callbacks for capturing the EPC events
// @db_msg: data for MSI from RC side
// @num_db: number of doorbells
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf {
    pub dev: device,
    pub name: *const c_char,
    pub header: *mut pci_epf_header,
    pub bar: [pci_epf_bar; PCI_STD_NUM_BARS],
    pub msi_interrupts: u8,
    pub msix_interrupts: u16,
    pub func_no: u8,
    pub vfunc_no: u8,
    pub epc: *mut pci_epc,
    pub epf_pf: *mut pci_epf,
    pub driver: *mut pci_epf_driver,
    pub id: *const pci_epf_device_id,
    pub list: list_head,
// mutex to protect against concurrent access of pci_epf_ops
    pub lock: mutex,
// Below members are to attach secondary EPC to an endpoint function
    pub sec_epc: *mut pci_epc,
    pub sec_epc_list: list_head,
    pub sec_epc_bar: [pci_epf_bar; PCI_STD_NUM_BARS],
    pub sec_epc_func_no: u8,
    pub group: *mut config_group,
    pub is_bound: c_uint,
    pub is_vf: c_uint,
    pub vfunction_num_map: c_ulong,
    pub pci_vepf: list_head,
    pub event_ops: *const pci_epc_event_ops,
    pub db_msg: *mut pci_epf_doorbell_msg,
    pub num_db: u16,
}

//
// struct pci_epf_msix_tbl - represents the MSI-X table entry structure
// @msg_addr: Writes to this address will trigger MSI-X interrupt in host
// @msg_data: Data that should be written to @msg_addr to trigger MSI-X
// interrupt
// @vector_ctrl: Identifies if the function is prohibited from sending a message
// using this MSI-X table entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_epf_msix_tbl {
    pub msg_addr: u64,
    pub msg_data: u32,
    pub vector_ctrl: u32,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &epf->dev) -> return;
}
extern "C" {
    pub fn pci_epf_destroy(epf: *mut pci_epf);
}
extern "C" {
    pub fn pci_epf_unregister_driver(driver: *mut pci_epf_driver);
}
extern "C" {
    pub fn pci_epf_bind(epf: *mut pci_epf) -> c_int;
}
extern "C" {
    pub fn pci_epf_unbind(epf: *mut pci_epf);
}
extern "C" {
    pub fn pci_epf_add_vepf(epf_pf: *mut pci_epf, epf_vf: *mut pci_epf) -> c_int;
}
extern "C" {
    pub fn pci_epf_remove_vepf(epf_pf: *mut pci_epf, epf_vf: *mut pci_epf);
}
