//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pci.h
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

pub const ARCH_GENERIC_PCI_MMAP_RESOURCE: c_int = 1;
pub const arch_can_pci_mmap_wc(): c_int = 1;
pub const PCIBIOS_MIN_IO: c_uint = 0x1000;
pub const PCIBIOS_MIN_MEM: c_uint = 0x10000000;

extern "C" {
    pub fn pci_iounmap(: *mut pci_dev, : *mut void __iomem);
}
extern "C" {
    pub fn pci_domain_nr(: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn pci_proc_domain(: *mut pci_bus) -> c_int;
}

pub const ZPCI_NR_DMA_SPACES: c_int = 1;

// PCI Function Controls
pub const ZPCI_FC_FN_ENABLED: c_uint = 0x80;
pub const ZPCI_FC_ERROR: c_uint = 0x40;
pub const ZPCI_FC_BLOCKED: c_uint = 0x20;
pub const ZPCI_FC_DMA_ENABLED: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fmb_fmt0 {
    pub dma_rbytes: u64,
    pub dma_wbytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fmb_fmt1 {
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fmb_fmt2 {
    pub consumed_work_units: u64,
    pub max_work_units: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fmb_fmt3 {
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_fmb {
    pub 8: u32 format :,
    pub 24: u32 fmt_ind :,
    pub samples: u32,
    pub last_update: u64,
// common counters
    pub ld_ops: u64,
    pub st_ops: u64,
    pub stb_ops: u64,
    pub rpcit_ops: u64,
// format specific counters
    pub fmt0: zpci_fmb_fmt0,
    pub fmt1: zpci_fmb_fmt1,
    pub fmt2: zpci_fmb_fmt2,
    pub fmt3: zpci_fmb_fmt3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zpci_state {
    ZPCI_FN_STATE_STANDBY = 0,
    ZPCI_FN_STATE_CONFIGURED = 1,
    ZPCI_FN_STATE_RESERVED = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_bar_struct {
    pub /: *mut *mut *mut resource res; / bus resource,
    pub mio_wb: *mut void __iomem,
    pub mio_wt: *mut void __iomem,
    pub /: *mut *mut u32 val; / bar start & 3 flag bits,
    pub /: *mut *mut u16 map_idx; / index into bar mapping array,
    pub /: *mut *mut u8 size; / order 2 exponent,
}

pub const ZPCI_FUNCTIONS_PER_BUS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_bus {
    pub kref: kref,
    pub bus: *mut pci_bus,
    pub function: [*mut zpci_dev; ZPCI_FUNCTIONS_PER_BUS],
    pub resources: list_head,
    pub bus_next: list_head,
    pub bus_resource: resource,
    pub msi_parent_domain: *mut irq_domain,
    pub /: *mut *mut int topo; / TID if topo_is_tid, PCHID otherwise,
    pub domain_nr: c_int,
    pub 1: u8 multifunction :,
    pub 1: u8 topo_is_tid :,
    pub max_bus_speed: pci_bus_speed,
}

// Content Code Description for PCI Function Error
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_ccdf_err {
    pub reserved1: u32,
    pub /: *mut *mut u32 fh; / function handle,
    pub /: *mut *mut u32 fid; / function id,
    pub /: *mut *mut u32 ett : 4; / expected table type,
    pub /: *mut *mut u32 mvn : 12; / MSI vector number,
    pub /: *mut *mut u32 dmaas : 8; / DMA address space,
    pub 6: u32 reserved2 :,
    pub /: *mut *mut u32 q : 1; / event qualifier,
    pub /: *mut *mut u32 rw : 1; / read/write,
    pub /: *mut *mut u64 faddr; / failing address,
    pub reserved3: u32,
    pub reserved4: u16,
    pub /: *mut *mut u16 pec; / PCI event code,
    pub __packed: },
pub const ZPCI_ERR_PENDING_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_ccdf_pending {
    pub mediated_recovery: bool,
    pub count: u8,
    pub head: u8,
    pub tail: u8,
    pub err: [zpci_ccdf_err; ZPCI_ERR_PENDING_MAX],
}

// Private data per function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpci_dev {
    pub zbus: *mut zpci_bus,
    pub /: *mut *mut list_head entry; / list of all zpci_devices, needed for hotplug, etc.,
    pub iommu_list: list_head,
    pub kref: kref,
    pub rcu: rcu_head,
    pub hotplug_slot: hotplug_slot,
    pub /: *mut *mut mutex state_lock; / protect state changes,
    pub state: zpci_state,
    pub /: *mut *mut u32 fid; / function ID, used by sclp,
    pub /: *mut *mut u32 fh; / function handle, used by insn's,
    pub /: *mut *mut u32 gisa; / GISA designation for passthrough,
    pub /: *mut *mut u16 vfn; / virtual function number,
    pub /: *mut *mut u16 pchid; / physical channel ID,
    pub /: *mut *mut u16 maxstbl; / Maximum store block size,
    pub /: *mut *mut u16 rid; / RID as supplied by firmware,
    pub /: *mut *mut u16 tid; / Topology for which RID is valid,
    pub /: *mut *mut u8 pfgid; / function group ID,
    pub /: *mut *mut u8 pft; / pci function type,
    pub port: u8,
    pub fidparm: u8,
    pub /: *mut *mut u8 dtsm; / Supported DT mask,
    pub 1: u8 rid_available :,
    pub 1: u8 has_hp_slot :,
    pub 1: u8 has_resources :,
    pub 1: u8 is_physfn :,
    pub 1: u8 util_str_avail :,
    pub 1: u8 tid_avail :,
    pub /: *mut *mut u8 rtr_avail : 1; / Relaxed translation allowed,
    pub RID*/: *mut *mut unsigned int devfn; / DEVFN part of the,
    pub /: *mut *mut u8 pfip[CLP_PFIP_NR_SEGMENTS]; / pci function internal path,
    pub /: *mut *mut u32 uid; / user defined id,
    pub /: *mut *mut u8 util_str[CLP_UTIL_STR_LEN]; / utility string,
// IRQ stuff
    pub /: *mut *mut u64 msi_addr; / MSI address,
    pub /: *mut *mut unsigned int max_msi; / maximum number of MSI's,
    pub msi_first_bit: c_uint,
    pub msi_nr_irqs: c_uint,
    pub /: *mut *mut *mut airq_iv aibv; / adapter interrupt bit vector,
    pub /: *mut *mut unsigned long aisb; / number of the summary bit,
// DMA stuff
    pub dma_table: *mut c_ulong,
    pub tlb_refresh: c_int,
    pub /: *mut *mut iommu_device iommu_dev; / IOMMU core handle,
    pub res_name: [c_char; 16],
    pub mio_capable: bool,
    pub bars: [zpci_bar_struct; PCI_STD_NUM_BARS],
    pub /: *mut *mut u64 start_dma; / Start of available DMA addresses,
    pub /: *mut *mut u64 end_dma; / End of available DMA addresses,
    pub /: *mut *mut u64 dma_mask; / DMA address space mask,
// Function measurement block
    pub fmb_lock: mutex,
    pub fmb: *mut zpci_fmb,
    pub /: *mut *mut u16 fmb_update; / update interval,
    pub fmb_length: u16,
    pub version: u8,
    pub max_bus_speed: pci_bus_speed,
    pub debugfs_dev: *mut dentry,
// IOMMU and passthrough
    pub /: *mut *mut *mut iommu_domain s390_domain; / attached IOMMU domain,
    pub kzdev: *mut kvm_zdev,
    pub kzdev_lock: mutex,
    pub pending_errs: zpci_ccdf_pending,
    pub pending_errs_lock: mutex,
    pub /: *mut *mut spinlock_t dom_lock; / protect s390_domain change,
}

// -----------------------------------------------------------------------------
// Base stuff
extern "C" {
    pub fn zpci_add_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_enable_device(: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_reenable_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_disable_device(: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_scan_configured_device(zdev: *mut zpci_dev, fh: u32) -> c_int;
}
extern "C" {
    pub fn zpci_deconfigure_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_device_reserved(zdev: *mut zpci_dev);
}
extern "C" {
    pub fn zpci_is_device_configured(zdev: *mut zpci_dev) -> bool;
}
extern "C" {
    pub fn zpci_scan_devices() -> c_int;
}
extern "C" {
    pub fn zpci_hot_reset_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_register_ioat(: *mut zpci_dev, _arg: u8, _arg: u64, _arg: u64, _arg: u64, : *mut u8) -> c_int;
}
extern "C" {
    pub fn zpci_unregister_ioat(: *mut zpci_dev, _arg: u8) -> c_int;
}
extern "C" {
    pub fn zpci_remove_reserved_devices();
}
extern "C" {
    pub fn zpci_update_fh(zdev: *mut zpci_dev, fh: u32);
}
// CLP
extern "C" {
    pub fn clp_setup_writeback_mio() -> c_int;
}
extern "C" {
    pub fn clp_scan_pci_devices(scan_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn clp_query_pci_fn(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn clp_enable_fh(zdev: *mut zpci_dev, fh: *mut u32, nr_dma_as: u8) -> c_int;
}
extern "C" {
    pub fn clp_disable_fh(zdev: *mut zpci_dev, fh: *mut u32) -> c_int;
}
extern "C" {
    pub fn clp_get_state(fid: u32, state: *mut zpci_state) -> c_int;
}
extern "C" {
    pub fn clp_refresh_fh(fid: u32, fh: *mut u32) -> c_int;
}
// UID
extern "C" {
    pub fn update_uid_checking(new: bool);
}
// Firmware Sysfs
extern "C" {
    pub fn __zpci_fw_sysfs_init() -> int __init;
}
extern "C" {
    pub fn __zpci_fw_sysfs_init() -> return;
}
// IOMMU Interface
extern "C" {
    pub fn zpci_init_iommu(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_destroy_iommu(zdev: *mut zpci_dev);
}
extern "C" {
    pub fn zpci_iommu_register_ioat(zdev: *mut zpci_dev, status: *mut u8) -> c_int;
}

// Error handling and recovery
extern "C" {
    pub fn zpci_event_error(: *mut c_void);
}
extern "C" {
    pub fn zpci_event_availability(: *mut c_void);
}
extern "C" {
    pub fn zpci_is_enabled() -> bool;
}

extern "C" {
    pub fn zpci_init_slot(: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_exit_slot(: *mut zpci_dev);
}

// Helpers
extern "C" {
    pub fn to_zpci(_arg: to_pci_dev(dev)) -> return;
}
// DMA
extern "C" {
    pub fn zpci_dma_init() -> c_int;
}
extern "C" {
    pub fn zpci_dma_exit();
}
extern "C" {
    pub fn zpci_dma_init_device(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_dma_exit_device(zdev: *mut zpci_dev) -> c_int;
}
// IRQ
extern "C" {
    pub fn zpci_irq_init() -> int __init;
}
extern "C" {
    pub fn zpci_irq_exit() -> void __init;
}
extern "C" {
    pub fn zpci_set_irq(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_create_parent_msi_domain(zbus: *mut zpci_bus) -> c_int;
}
extern "C" {
    pub fn zpci_remove_parent_msi_domain(zbus: *mut zpci_bus);
}
// FMB
extern "C" {
    pub fn zpci_fmb_enable_device(: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_fmb_disable_device(: *mut zpci_dev) -> c_int;
}
// Debug
extern "C" {
    pub fn zpci_debug_init() -> c_int;
}
extern "C" {
    pub fn zpci_debug_exit();
}
extern "C" {
    pub fn zpci_debug_init_device(: *mut zpci_dev, : *const c_char);
}
extern "C" {
    pub fn zpci_debug_exit_device(: *mut zpci_dev);
}
// Error handling
extern "C" {
    pub fn zpci_report_error(: *mut pci_dev, : *mut zpci_report_error_header) -> c_int;
}
extern "C" {
    pub fn zpci_clear_error_state(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_reset_load_store_blocked(zdev: *mut zpci_dev) -> c_int;
}
extern "C" {
    pub fn zpci_start_mediated_recovery(zdev: *mut zpci_dev);
}
extern "C" {
    pub fn zpci_stop_mediated_recovery(zdev: *mut zpci_dev);
}

// Returns the node based on PCI bus

