//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dmar.h
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
// Copyright (c) 2006, Intel Corporation.
//
// Copyright (C) Ashok Raj <ashok.raj@intel.com>
// Copyright (C) Shaohua Li <shaohua.li@intel.com>
//

pub const DMAR_UNITS_SUPPORTED: c_int = 1024;
// DMAR Flags
pub const DMAR_INTR_REMAP: c_uint = 0x1;
pub const DMAR_X2APIC_OPT_OUT: c_uint = 0x2;
pub const DMAR_PLATFORM_OPT_IN: c_uint = 0x4;
pub const DMAR_REMAP_OPT_OUT: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmar_dev_scope {
    pub dev: *mut device __rcu,
    pub bus: u8,
    pub devfn: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmar_drhd_unit {
    pub /: *mut *mut list_head list; / list of drhd units,
    pub /: *mut *mut *mut acpi_dmar_header hdr; / ACPI header,
    pub address*/: *mut *mut u64 reg_base_addr; / register base,
    pub /: *mut *mut unsigned long reg_size; / size of register set,
    pub /: *mut *mut *mut dmar_dev_scope devices;/ target device array,
    pub /: *mut *mut int devices_cnt; / target device count,
    pub /: *mut *mut u16 segment; / PCI domain,
    pub /: *mut *mut u8 ignored:1; / ignore drhd,
    pub include_all:1: u8,
    pub /: *mut *mut u8 gfx_dedicated:1; / graphic dedicated,
    pub iommu: *mut intel_iommu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmar_pci_path {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmar_pci_notify_info {
    pub dev: *mut pci_dev,
    pub event: c_ulong,
    pub bus: c_int,
    pub seg: u16,
    pub level: u16,
    pub path: [dmar_pci_path; ],
    pub __attribute__((packed)): },
    pub dmar_global_lock: extern struct rw_semaphore,
    pub dmar_drhd_units: extern struct list_head,

    pub SYSTEM_BOOTING: system_state ==,

    pub \: for ((i) = 0; ((tmp) = (i) < (cnt) ?,
    pub \: dmar_rcu_dereference((devs)[(i)].dev) : NULL, (i) < (cnt));,

    pub else: if (!(tmp)) { continue; },
    pub dmar_table_init(void): extern int,
    pub dmar_dev_scope_init(void): extern int,
    pub dmar_register_bus_notifier(void): extern void,
    pub cnt): *mut *mut *mut *mut extern void dmar_alloc_dev_scope(void start, void end, int,
    pub cnt): *mut *mut *mut extern void dmar_free_dev_scope(struct dmar_dev_scope devices, int,
    pub devices_cnt): c_int,
    pub count): c_int,
// Intel IOMMU detection
    pub detect_intel_iommu(void): c_void,
    pub cpu): extern int enable_drhd_fault_handling(unsigned int,
    pub handle): extern int dmar_device_add(acpi_handle,
    pub handle): extern int dmar_device_remove(acpi_handle,
    pub 0: return,

    pub pasid): unsigned long long addr, u32,

    pub no_iommu: extern int iommu_detected,,
    pub intel_iommu_init(void): extern int,
    pub intel_iommu_shutdown(void): extern void,
    pub arg): *mut *mut extern int dmar_parse_one_rmrr(struct acpi_dmar_header header, void,
    pub arg): *mut *mut extern int dmar_parse_one_atsr(struct acpi_dmar_header header, void,
    pub arg): *mut *mut extern int dmar_check_one_atsr(struct acpi_dmar_header hdr, void,
    pub arg): *mut *mut extern int dmar_parse_one_satc(struct acpi_dmar_header hdr, void,
    pub arg): *mut *mut extern int dmar_release_one_atsr(struct acpi_dmar_header hdr, void,
    pub insert): *mut *mut extern int dmar_iommu_hotplug(struct dmar_drhd_unit dmaru, bool,
    pub info): *mut extern int dmar_iommu_notify_scope_dev(struct dmar_pci_notify_info,

    pub }: static inline int intel_iommu_init(void) { return -ENODEV;,

    pub 0: return,
    pub 0: return,

    pub insert): *mut *mut extern int dmar_ir_hotplug(struct dmar_drhd_unit dmaru, bool,

    pub }: { return 0;,

    pub dmar_platform_optin(void): extern bool,

    pub 0: return,
    pub 0: return,
    pub false: return,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irte {
// Shared between remapped and posted mode
    pub /: *mut *mut __res2 : 40; / 24 - 63,
}

// Remapped mode
// Posted mode
// Shared between remapped and posted mode
// Posted mode

pub const PDA_LOW_BIT: c_int = 26;
pub const PDA_HIGH_BIT: c_int = 32;
// Can't use the common MSI interrupt functions
// since DMAR is not a pci device
//
extern "C" {
    pub fn dmar_msi_unmask(data: *mut irq_data);
}
extern "C" {
    pub fn dmar_msi_mask(data: *mut irq_data);
}
extern "C" {
    pub fn dmar_msi_write(irq: c_int, msg: *mut msi_msg);
}
extern "C" {
    pub fn dmar_set_interrupt(iommu: *mut intel_iommu) -> c_int;
}
extern "C" {
    pub fn dmar_fault(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn dmar_alloc_hwirq(id: c_int, node: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dmar_free_hwirq(irq: c_int);
}
