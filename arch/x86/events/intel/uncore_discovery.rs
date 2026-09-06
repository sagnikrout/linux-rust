//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/events/intel/uncore_discovery.h
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
// Store the full address of the global discovery table
pub const UNCORE_DISCOVERY_MSR: c_uint = 0x201e;
// Base address of uncore perfmon discovery table for CBB domain
pub const CBB_UNCORE_DISCOVERY_MSR: c_uint = 0x710;
// Base address of uncore perfmon discovery table for the package
pub const PACKAGE_UNCORE_DISCOVERY_MSR: c_uint = 0x711;
// Generic device ID of a discovery table device
pub const UNCORE_DISCOVERY_TABLE_DEVICE: c_uint = 0x09a7;
// Device ID used on DMR
pub const DMR_UNCORE_DISCOVERY_TABLE_DEVICE: c_uint = 0x09a1;
// Capability ID for a discovery table device
pub const UNCORE_EXT_CAP_ID_DISCOVERY: c_uint = 0x23;
// First DVSEC offset
pub const UNCORE_DISCOVERY_DVSEC_OFFSET: c_uint = 0x8;
// Mask of the supported discovery entry type
pub const UNCORE_DISCOVERY_DVSEC_ID_MASK: c_uint = 0xffff;
// PMON discovery entry type ID
pub const UNCORE_DISCOVERY_DVSEC_ID_PMON: c_uint = 0x1;
// Second DVSEC offset
pub const UNCORE_DISCOVERY_DVSEC2_OFFSET: c_uint = 0xc;
// Mask of the discovery table BAR offset
pub const UNCORE_DISCOVERY_DVSEC2_BIR_MASK: c_uint = 0x7;
// Discovery table BAR base offset
pub const UNCORE_DISCOVERY_BIR_BASE: c_uint = 0x10;
// Discovery table BAR step
pub const UNCORE_DISCOVERY_BIR_STEP: c_uint = 0x4;
// Global discovery table size
pub const UNCORE_DISCOVERY_GLOBAL_MAP_SIZE: c_uint = 0x20;
pub const UNCORE_DISCOVERY_PCI_DOMAIN_OFFSET: c_int = 28;

pub const UNCORE_DISCOVERY_PCI_BUS_OFFSET: c_int = 20;

pub const UNCORE_DISCOVERY_PCI_DEVFN_OFFSET: c_int = 12;

pub const GENERIC_PMON_CTL_EV_SEL_MASK: c_uint = 0x000000ff;
pub const GENERIC_PMON_CTL_UMASK_MASK: c_uint = 0x0000ff00;

pub const GENERIC_PMON_CTL_TRESH_MASK: c_uint = 0xff000000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uncore_access_type {
    UNCORE_ACCESS_MSR	= 0,
    UNCORE_ACCESS_MMIO,
    UNCORE_ACCESS_PCI,

    UNCORE_ACCESS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_global_discovery {
    pub table1: u64,
    pub 2: access_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_unit_discovery {
    pub table1: u64,
    pub 2: access_type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_discovery_unit {
    pub node: rb_node,
    pub /: *mut *mut unsigned int pmu_idx; / The idx of the corresponding PMU,
    pub /: *mut *mut unsigned int id; / Unit ID,
    pub /: *mut *mut unsigned int die; / Die ID,
    pub /: *mut *mut u64 addr; / Unit Control Address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_discovery_type {
    pub node: rb_node,
    pub access_type: uncore_access_type,
    pub /: *mut *mut rb_root units; / Unit ctrl addr for all units,
    pub /: *mut *mut u16 type; / Type ID of the uncore block,
    pub num_counters: u8,
    pub counter_width: u8,
    pub /: *mut *mut u8 ctl_offset; / Counter Control 0 offset,
    pub /: *mut *mut u8 ctr_offset; / Counter 0 offset,
    pub /: *mut *mut u16 num_units; / number of units,
}

extern "C" {
    pub fn uncore_discovery(init: *mut uncore_plat_init) -> bool;
}
extern "C" {
    pub fn intel_uncore_clear_discovery_tables();
}
extern "C" {
    pub fn intel_uncore_generic_uncore_cpu_init();
}
extern "C" {
    pub fn intel_uncore_generic_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn intel_uncore_generic_uncore_mmio_init();
}
extern "C" {
    pub fn intel_generic_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int;
}
extern "C" {
    pub fn intel_generic_uncore_msr_disable_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn intel_generic_uncore_msr_enable_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn intel_generic_uncore_mmio_init_box(box: *mut intel_uncore_box) -> c_int;
}
extern "C" {
    pub fn intel_generic_uncore_mmio_disable_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn intel_generic_uncore_mmio_enable_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn intel_generic_uncore_pci_init_box(box: *mut intel_uncore_box) -> c_int;
}
extern "C" {
    pub fn intel_generic_uncore_pci_disable_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn intel_generic_uncore_pci_enable_box(box: *mut intel_uncore_box);
}
