//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/physdev.h
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


// SPDX-License-Identifier: MIT
//
// Prototype for this hypercall is:
// int physdev_op(int cmd, void *args)
// @cmd	 == PHYSDEVOP_??? (physdev operation).
// @args == Operation-specific extra arguments (NULL if none).
//
// Notify end-of-interrupt (EOI) for the specified IRQ.
// @arg == pointer to physdev_eoi structure.
//
pub const PHYSDEVOP_eoi: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_eoi {
// IN
    pub irq: u32,
}

//
// Register a shared page for the hypervisor to indicate whether the guest
// must issue PHYSDEVOP_eoi. The semantics of PHYSDEVOP_eoi change slightly
// once the guest used this function in that the associated event channel
// will automatically get unmasked. The page registered is used as a bit
// array indexed by Xen's PIRQ value.
//
pub const PHYSDEVOP_pirq_eoi_gmfn_v1: c_int = 17;
//
// Register a shared page for the hypervisor to indicate whether the
// guest must issue PHYSDEVOP_eoi. This hypercall is very similar to
// PHYSDEVOP_pirq_eoi_gmfn_v1 but it doesn't change the semantics of
// PHYSDEVOP_eoi. The page registered is used as a bit array indexed by
// Xen's PIRQ value.
//
pub const PHYSDEVOP_pirq_eoi_gmfn_v2: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pirq_eoi_gmfn {
// IN
    pub gmfn: xen_ulong_t,
}

//
// Query the status of an IRQ line.
// @arg == pointer to physdev_irq_status_query structure.
//
pub const PHYSDEVOP_irq_status_query: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_irq_status_query {
// IN
    pub irq: u32,
// OUT
    pub /: *mut *mut *mut uint32_t flags; / XENIRQSTAT_,
}

// Need to call PHYSDEVOP_eoi when the IRQ has been serviced?

// IRQ shared by multiple guests?

//
// Set the current VCPU's I/O privilege level.
// @arg == pointer to physdev_set_iopl structure.
//
pub const PHYSDEVOP_set_iopl: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_set_iopl {
// IN
    pub iopl: u32,
}

//
// Set the current VCPU's I/O-port permissions bitmap.
// @arg == pointer to physdev_set_iobitmap structure.
//
pub const PHYSDEVOP_set_iobitmap: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_set_iobitmap {
// IN
    pub bitmap: *mut *mut u8,
    pub nr_ports: u32,
}

//
// Read or write an IO-APIC register.
// @arg == pointer to physdev_apic structure.
//
pub const PHYSDEVOP_apic_read: c_int = 8;
pub const PHYSDEVOP_apic_write: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_apic {
// IN
    pub apic_physbase: c_ulong,
    pub reg: u32,
// IN or OUT
    pub value: u32,
}

//
// Allocate or free a physical upcall vector for the specified IRQ line.
// @arg == pointer to physdev_irq structure.
//
pub const PHYSDEVOP_alloc_irq_vector: c_int = 10;
pub const PHYSDEVOP_free_irq_vector: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_irq {
// IN
    pub irq: u32,
// IN or OUT
    pub vector: u32,
}

pub const MAP_PIRQ_TYPE_MSI: c_uint = 0x0;
pub const MAP_PIRQ_TYPE_GSI: c_uint = 0x1;
pub const MAP_PIRQ_TYPE_UNKNOWN: c_uint = 0x2;
pub const MAP_PIRQ_TYPE_MSI_SEG: c_uint = 0x3;
pub const MAP_PIRQ_TYPE_MULTI_MSI: c_uint = 0x4;
pub const PHYSDEVOP_map_pirq: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_map_pirq {
    pub domid: domid_t,
// IN
    pub type: c_int,
// IN
    pub index: c_int,
// IN or OUT
    pub pirq: c_int,
// IN - high 16 bits hold segment for ..._MSI_SEG and ..._MULTI_MSI
    pub bus: c_int,
// IN
    pub devfn: c_int,
// IN
// - For MSI-X contains entry number.
// - For MSI with ..._MULTI_MSI contains number of vectors.
// OUT (..._MULTI_MSI only)
// - Number of vectors allocated.
//
    pub entry_nr: c_int,
// IN
    pub table_base: u64,
}

pub const PHYSDEVOP_unmap_pirq: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_unmap_pirq {
    pub domid: domid_t,
// IN
    pub pirq: c_int,
}

pub const PHYSDEVOP_manage_pci_add: c_int = 15;
pub const PHYSDEVOP_manage_pci_remove: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_manage_pci {
// IN
    pub bus: u8,
    pub devfn: u8,
}

pub const PHYSDEVOP_restore_msi: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_restore_msi {
// IN
    pub bus: u8,
    pub devfn: u8,
}

pub const PHYSDEVOP_manage_pci_add_ext: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_manage_pci_ext {
// IN
    pub bus: u8,
    pub devfn: u8,
    pub is_extfn: unsigned,
    pub is_virtfn: unsigned,
    pub bus: u8,
    pub devfn: u8,
    pub physfn: },
}

//
// Argument to physdev_op_compat() hypercall. Superceded by new physdev_op()
// hypercall since 0x00030202.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_op {
    pub cmd: u32,
    pub irq_status_query: physdev_irq_status_query,
    pub set_iopl: physdev_set_iopl,
    pub set_iobitmap: physdev_set_iobitmap,
    pub apic_op: physdev_apic,
    pub irq_op: physdev_irq,
    pub u: },
}

pub const PHYSDEVOP_setup_gsi: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_setup_gsi {
    pub gsi: c_int,
// IN
    pub triggering: u8,
// IN
    pub polarity: u8,
// IN
}

pub const PHYSDEVOP_get_nr_pirqs: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_nr_pirqs {
// OUT
    pub nr_pirqs: u32,
}

// type is MAP_PIRQ_TYPE_GSI or MAP_PIRQ_TYPE_MSI
// the hypercall returns a free pirq
pub const PHYSDEVOP_get_free_pirq: c_int = 23;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_get_free_pirq {
// IN
    pub type: c_int,
// OUT
    pub pirq: u32,
}

pub const XEN_PCI_DEV_EXTFN: c_uint = 0x1;
pub const XEN_PCI_DEV_VIRTFN: c_uint = 0x2;
pub const XEN_PCI_DEV_PXM: c_uint = 0x4;
pub const XEN_PCI_MMCFG_RESERVED: c_uint = 0x1;
pub const PHYSDEVOP_pci_mmcfg_reserved: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_mmcfg_reserved {
    pub address: u64,
    pub segment: u16,
    pub start_bus: u8,
    pub end_bus: u8,
    pub flags: u32,
}

pub const PHYSDEVOP_pci_device_add: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_device_add {
// IN
    pub seg: u16,
    pub bus: u8,
    pub devfn: u8,
    pub flags: u32,
    pub bus: u8,
    pub devfn: u8,
    pub physfn: },
    pub optarr: [u32; ],    pub optarr: [u32; 0],
}

pub const PHYSDEVOP_pci_device_remove: c_int = 26;
pub const PHYSDEVOP_restore_msi_ext: c_int = 27;
//
// Dom0 should use these two to announce MMIO resources assigned to
// MSI-X capable devices won't (prepare) or may (release) change.
//
pub const PHYSDEVOP_prepare_msix: c_int = 30;
pub const PHYSDEVOP_release_msix: c_int = 31;
//
// Notify the hypervisor that a PCI device has been reset, so that any
// internally cached state is regenerated.  Should be called after any
// device reset performed by the hardware domain.
//
pub const PHYSDEVOP_pci_device_reset: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_pci_device {
// IN
    pub seg: u16,
    pub bus: u8,
    pub devfn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_device_reset {
    pub dev: physdev_pci_device,
pub const PCI_DEVICE_RESET_COLD: c_uint = 0x0;
pub const PCI_DEVICE_RESET_WARM: c_uint = 0x1;
pub const PCI_DEVICE_RESET_HOT: c_uint = 0x2;
pub const PCI_DEVICE_RESET_FLR: c_uint = 0x3;
pub const PCI_DEVICE_RESET_MASK: c_uint = 0x3;
    pub flags: u32,
}

pub const PHYSDEVOP_DBGP_RESET_PREPARE: c_int = 1;
pub const PHYSDEVOP_DBGP_RESET_DONE: c_int = 2;
pub const PHYSDEVOP_DBGP_BUS_UNKNOWN: c_int = 0;
pub const PHYSDEVOP_DBGP_BUS_PCI: c_int = 1;
pub const PHYSDEVOP_dbgp_op: c_int = 29;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct physdev_dbgp_op {
// IN
    pub op: u8,
    pub bus: u8,
    pub pci: physdev_pci_device,
    pub u: },
}

//
// Notify that some PIRQ-bound event channels have been unmasked.
// ** This command is obsolete since interface version 0x00030202 and is
// ** unsupported by newer versions of Xen.
//
pub const PHYSDEVOP_IRQ_UNMASK_NOTIFY: c_int = 4;
//
// These all-capitals physdev operation names are superceded by the new names
// (defined above) since interface version 0x00030202.
//

