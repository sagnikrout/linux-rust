//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/arm-gic-v4.h
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
// Copyright (C) 2016,2017 ARM Limited, All Rights Reserved.
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Maximum number of ITTs when GITS_TYPER.VMOVP == 0, using the
// ITSList mechanism to perform inter-ITS synchronization.
//
pub const GICv4_ITS_LIST_MAX: c_int = 16;
// Embedded in kvm.arch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_vm {
    pub fwnode: *mut fwnode_handle,
    pub domain: *mut irq_domain,
    pub vprop_page: *mut page,
    pub vpes: *mut its_vpe,
    pub nr_vpes: c_int,
    pub db_lpi_base: irq_hw_number_t,
    pub db_bitmap: *mut c_ulong,
    pub nr_db_lpis: c_int,
//
// Ensures mutual exclusion between updates to vlpi_count[]
// and map/unmap when using the ITSList mechanism.
//
// The lock order for any sequence involving the ITSList is
// vmapp_lock -> vpe_lock ->vmovp_lock.
//
    pub vmapp_lock: raw_spinlock_t,
    pub vlpi_count: [u32; GICv4_ITS_LIST_MAX],
}

// Embedded in kvm_vcpu.arch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_vpe {
    pub vpt_page: *mut page,
    pub its_vm: *mut its_vm,
// per-vPE VLPI tracking
    pub vlpi_count: core::sync::atomic::AtomicI32,
// Doorbell interrupt
    pub irq: c_int,
    pub vpe_db_lpi: irq_hw_number_t,
// VPE resident
    pub resident: bool,
// VPT parse complete
    pub ready: bool,
// GICv4.0 implementations
// VPE proxy mapping
    pub vpe_proxy_event: c_int,
// Implementation Defined Area Invalid
    pub idai: bool,
}

// GICv4.1 implementations
// Track the VPE being mapped
//
// Ensures mutual exclusion between affinity setting of the
// vPE and vLPI operations using vpe->col_idx.
//
// This collection ID is used to indirect the target
// redistributor for this VPE. The ID itself isn't involved in
// programming of the ITS.
//
// Unique (system-wide) VPE identifier
// Pending VLPIs on schedule out?
//
// struct its_vlpi_map: structure describing the mapping of a
// VLPI. Only to be interpreted in the context of a physical interrupt
// it complements.  To be used as the vcpu_info passed to
// irq_set_vcpu_affinity().
//
// @vm:		Pointer to the GICv4 notion of a VM
// @vpe:	Pointer to the GICv4 notion of a virtual CPU (VPE)
// @vintid:	Virtual LPI number
// @properties:	Priority and enable bits (as written in the prop table)
// @db_enabled:	Is the VPE doorbell to be generated?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_vlpi_map {
    pub vm: *mut its_vm,
    pub vpe: *mut its_vpe,
    pub vintid: u32,
    pub properties: u8,
    pub db_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum its_vcpu_info_cmd_type {
    MAP_VLPI,
    GET_VLPI,
    PROP_UPDATE_VLPI,
    PROP_UPDATE_AND_INV_VLPI,
    SCHEDULE_VPE,
    DESCHEDULE_VPE,
    COMMIT_VPE,
    INVALL_VPE,
    PROP_UPDATE_VSGI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct its_cmd_info {
    pub cmd_type: its_vcpu_info_cmd_type,
    pub map: *mut its_vlpi_map,
    pub config: u8,
    pub req_db: bool,
    pub g0en: bool,
    pub g1en: bool,
}

extern "C" {
    pub fn its_alloc_vcpu_irqs(vm: *mut its_vm) -> c_int;
}
extern "C" {
    pub fn its_free_vcpu_irqs(vm: *mut its_vm);
}
extern "C" {
    pub fn its_make_vpe_resident(vpe: *mut its_vpe, g0en: bool, g1en: bool) -> c_int;
}
extern "C" {
    pub fn its_make_vpe_non_resident(vpe: *mut its_vpe, db: bool) -> c_int;
}
extern "C" {
    pub fn its_commit_vpe(vpe: *mut its_vpe) -> c_int;
}
extern "C" {
    pub fn its_invall_vpe(vpe: *mut its_vpe) -> c_int;
}
extern "C" {
    pub fn its_map_vlpi(irq: c_int, map: *mut its_vlpi_map) -> c_int;
}
extern "C" {
    pub fn its_get_vlpi(irq: c_int, map: *mut its_vlpi_map) -> c_int;
}
extern "C" {
    pub fn its_unmap_vlpi(irq: c_int);
}
extern "C" {
    pub fn its_prop_update_vlpi(irq: c_int, config: u8, inv: bool) -> c_int;
}
extern "C" {
    pub fn its_prop_update_vsgi(irq: c_int, priority: u8, group: bool) -> c_int;
}
extern "C" {
    pub fn gic_cpuif_has_vsgi() -> bool;
}
