//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/powernv/pci.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnv_phb_type {
    PNV_PHB_IODA2,
    PNV_PHB_NPU_OCAPI,
}

// Precise PHB model for error management
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pnv_phb_model {
    PNV_PHB_MODEL_UNKNOWN,
    PNV_PHB_MODEL_P7IOC,
    PNV_PHB_MODEL_PHB3,
}

pub const PNV_PCI_DIAG_BUF_SIZE: c_int = 8192;

//
// A brief note on PNV_IODA_PE_BUS_ALL
//
// This is needed because of the behaviour of PCIe-to-PCI bridges. The PHB uses
// the Requester ID field of the PCIe request header to determine the device
// (and PE) that initiated a DMA. In legacy PCI individual memory read/write
// requests aren't tagged with the RID. To work around this the PCIe-to-PCI
// bridge will use (secondary_bus_no << 8) | 0x00 as the RID on the PCIe side.
//
// PCIe-to-X bridges have a similar issue even though PCI-X requests also have
// a RID in the transaction header. The PCIe-to-X bridge is permitted to "take
// ownership" of a transaction by a PCI-X device when forwarding it to the PCIe
// side of the bridge.
//
// To work around these problems we use the BUS_ALL flag since every subordinate
// bus of the bridge should go into the same PE.
//
// Indicates operations are frozen for a PE: MMIO in PESTA & DMA in PESTB.
pub const PNV_IODA_STOPPED_STATE: c_uint = 0x8000000000000000;
// Data associated with a PE, including IOMMU tracking etc..
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_ioda_pe {
    pub flags: c_ulong,
    pub phb: *mut pnv_phb,
    pub device_count: c_int,
// A PE can be associated with a single device or an
// entire bus (& children). In the former case, pdev
// is populated, in the later case, pbus is.
//

    pub parent_dev: *mut pci_dev,

    pub pdev: *mut pci_dev,
    pub pbus: *mut pci_bus,
// Effective RID (device RID for a device PE and base bus
// RID with devfn 0 for a bus PE)
//
    pub rid: c_uint,
// PE number
    pub pe_number: c_uint,
// "Base" iommu table, ie, 4K TCEs, 32-bit DMA
    pub table_group: iommu_table_group,
// 64-bit TCE bypass region
    pub tce_bypass_enabled: bool,
    pub tce_bypass_base: u64,
//
// Used to track whether we've done DMA setup for this PE or not. We
// want to defer allocating TCE tables, etc until we've added a
// non-bridge device to the PE.
//
    pub dma_setup_done: bool,
// MSIs. MVE index is identical for 32 and 64 bit MSI
// and -1 if not supported. (It's actually identical to the
// PE number)
//
    pub mve_number: c_int,
// PEs in compound case
    pub master: *mut pnv_ioda_pe,
    pub slaves: list_head,
// Link in list of PE#s
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_phb {
    pub hose: *mut pci_controller,
    pub type: pnv_phb_type,
    pub model: pnv_phb_model,
    pub hub_id: u64,
    pub opal_id: u64,
    pub flags: c_int,
    pub regs: *mut void __iomem,
    pub regs_phys: u64,
    pub lock: spinlock_t,

    pub has_dbgfs: c_int,
    pub dbgfs: *mut dentry,

    pub msi_base: c_uint,
    pub msi_bmp: msi_bitmap,
    pub phb): *mut *mut int (init_m64)(struct pnv_phb,
    pub pe_no): *mut *mut *mut int (get_pe_state)(struct pnv_phb phb, int,
    pub pe_no): *mut *mut *mut void (freeze_pe)(struct pnv_phb phb, int,
    pub opt): *mut *mut *mut int (unfreeze_pe)(struct pnv_phb phb, int pe_no, int,
// Global bridge info
    pub total_pe_num: c_uint,
    pub reserved_pe_idx: c_uint,
    pub root_pe_idx: c_uint,
// 32-bit MMIO window
    pub m32_size: c_uint,
    pub m32_segsize: c_uint,
    pub m32_pci_base: c_uint,
// 64-bit MMIO window
    pub m64_bar_idx: c_uint,
    pub m64_size: c_ulong,
    pub m64_segsize: c_ulong,
    pub m64_base: c_ulong,
pub const MAX_M64_BARS: c_int = 64;
    pub m64_bar_alloc: c_ulong,
// IO ports
    pub io_size: c_uint,
    pub io_segsize: c_uint,
    pub io_pci_base: c_uint,
// PE allocation
    pub pe_alloc_mutex: mutex,
    pub pe_alloc: *mut c_ulong,
    pub pe_array: *mut pnv_ioda_pe,
// M32 & IO segment maps
    pub m64_segmap: *mut c_uint,
    pub m32_segmap: *mut c_uint,
    pub io_segmap: *mut c_uint,
// IRQ chip
    pub irq_chip: irq_chip,
// Sorted list of used PE's based
// on the sequence of creation
//
    pub pe_list: list_head,
    pub pe_list_mutex: mutex,
// Reverse map of PEs, indexed by {bus, devfn}
    pub pe_rmap: [c_uint; 0x10000],
    pub ioda: },
// PHB and hub diagnostics
    pub diag_data_size: c_uint,
    pub diag_data: *mut u8,
}

// IODA PE management
//
// WARNING: We cannot rely on the resource flags. The Linux PCI
// allocation code sometimes decides to put a 64-bit prefetchable
// BAR in the 32-bit window, so we have to compare the addresses.
//
// For simplicity we only test resource start.
//
extern "C" {
    pub fn pnv_ioda_configure_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe) -> c_int;
}
extern "C" {
    pub fn pnv_ioda_deconfigure_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe) -> c_int;
}
extern "C" {
    pub fn pnv_pci_ioda2_setup_dma_pe(phb: *mut pnv_phb, pe: *mut pnv_ioda_pe);
}
extern "C" {
    pub fn pnv_pci_ioda2_release_pe_dma(pe: *mut pnv_ioda_pe);
}
extern "C" {
    pub fn pnv_ioda_free_pe(pe: *mut pnv_ioda_pe);
}

//
// For SR-IOV we want to put each VF's MMIO resource in to a separate PE.
// This requires a bit of acrobatics with the MMIO -> PE configuration
// and this structure is used to keep track of it all.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_iov_data {
// number of VFs enabled
    pub num_vfs: u16,
// pointer to the array of VF PEs. num_vfs long
    pub vf_pe_arr: *mut pnv_ioda_pe,
// Did we map the VF BAR with single-PE IODA BARs?
    pub m64_single_mode: [bool; PCI_SRIOV_NUM_BARS],
//
// True if we're using any segmented windows. In that case we need
// shift the start of the IOV resource the segment corresponding to
// the allocated PE.
//
    pub need_shift: bool,
//
// Bit mask used to track which m64 windows are used to map the
// SR-IOV BARs for this device.
//
    pub MAX_M64_BARS): DECLARE_BITMAP(used_m64_bar_mask,,
//
// If we map the SR-IOV BARs with a segmented window then
// parts of that window will be "claimed" by other PEs.
//
// "holes" here is used to reserve the leading portion
// of the window that is used by other (non VF) PEs.
//
    pub holes: [resource; PCI_SRIOV_NUM_BARS],
}

extern "C" {
    pub fn pnv_pci_ioda_fixup_iov(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pnv_pcibios_sriov_enable(pdev: *mut pci_dev, num_vfs: u16) -> c_int;
}
extern "C" {
    pub fn pnv_pcibios_sriov_disable(pdev: *mut pci_dev) -> c_int;
}

extern "C" {
    pub fn pnv_pci_init_ioda2_phb(np: *mut device_node);
}
extern "C" {
    pub fn pnv_pci_init_npu2_opencapi_phb(np: *mut device_node);
}
extern "C" {
    pub fn pnv_pci_reset_secondary_bus(dev: *mut pci_dev);
}
extern "C" {
    pub fn pnv_eeh_phb_reset(hose: *mut pci_controller, option: c_int) -> c_int;
}
extern "C" {
    pub fn pnv_eeh_post_init() -> c_int;
}

// pci-ioda-tce.c
pub const POWERNV_IOMMU_DEFAULT_LEVELS: c_int = 2;
pub const POWERNV_IOMMU_MAX_LEVELS: c_int = 5;
extern "C" {
    pub fn pnv_tce_free(tbl: *mut iommu_table, index: c_long, npages: c_long);
}
extern "C" {
    pub fn pnv_tce_get(tbl: *mut iommu_table, index: c_long) -> c_ulong;
}
extern "C" {
    pub fn pnv_pci_ioda2_table_free_pages(tbl: *mut iommu_table);
}
extern "C" {
    pub fn pnv_ioda_parse_tce_sizes(phb: *mut pnv_phb) -> c_ulong;
}
