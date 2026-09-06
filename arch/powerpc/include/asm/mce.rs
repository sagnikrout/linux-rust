//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mce.h
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
// Machine check exception header file.
//
// Copyright 2013 IBM Corporation
// Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_Version {
    MCE_V1 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_Severity {
    MCE_SEV_NO_ERROR = 0,
    MCE_SEV_WARNING = 1,
    MCE_SEV_SEVERE = 2,
    MCE_SEV_FATAL = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_Disposition {
    MCE_DISPOSITION_RECOVERED = 0,
    MCE_DISPOSITION_NOT_RECOVERED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_Initiator {
    MCE_INITIATOR_UNKNOWN = 0,
    MCE_INITIATOR_CPU = 1,
    MCE_INITIATOR_PCI = 2,
    MCE_INITIATOR_ISA = 3,
    MCE_INITIATOR_MEMORY= 4,
    MCE_INITIATOR_POWERMGM = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_ErrorType {
    MCE_ERROR_TYPE_UNKNOWN = 0,
    MCE_ERROR_TYPE_UE = 1,
    MCE_ERROR_TYPE_SLB = 2,
    MCE_ERROR_TYPE_ERAT = 3,
    MCE_ERROR_TYPE_TLB = 4,
    MCE_ERROR_TYPE_USER = 5,
    MCE_ERROR_TYPE_RA = 6,
    MCE_ERROR_TYPE_LINK = 7,
    MCE_ERROR_TYPE_DCACHE = 8,
    MCE_ERROR_TYPE_ICACHE = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_ErrorClass {
    MCE_ECLASS_UNKNOWN = 0,
    MCE_ECLASS_HARDWARE,
    MCE_ECLASS_HARD_INDETERMINATE,
    MCE_ECLASS_SOFTWARE,
    MCE_ECLASS_SOFT_INDETERMINATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_UeErrorType {
    MCE_UE_ERROR_INDETERMINATE = 0,
    MCE_UE_ERROR_IFETCH = 1,
    MCE_UE_ERROR_PAGE_TABLE_WALK_IFETCH = 2,
    MCE_UE_ERROR_LOAD_STORE = 3,
    MCE_UE_ERROR_PAGE_TABLE_WALK_LOAD_STORE = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_SlbErrorType {
    MCE_SLB_ERROR_INDETERMINATE = 0,
    MCE_SLB_ERROR_PARITY = 1,
    MCE_SLB_ERROR_MULTIHIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_EratErrorType {
    MCE_ERAT_ERROR_INDETERMINATE = 0,
    MCE_ERAT_ERROR_PARITY = 1,
    MCE_ERAT_ERROR_MULTIHIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_TlbErrorType {
    MCE_TLB_ERROR_INDETERMINATE = 0,
    MCE_TLB_ERROR_PARITY = 1,
    MCE_TLB_ERROR_MULTIHIT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_UserErrorType {
    MCE_USER_ERROR_INDETERMINATE = 0,
    MCE_USER_ERROR_TLBIE = 1,
    MCE_USER_ERROR_SCV = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_RaErrorType {
    MCE_RA_ERROR_INDETERMINATE = 0,
    MCE_RA_ERROR_IFETCH = 1,
    MCE_RA_ERROR_IFETCH_FOREIGN = 2,
    MCE_RA_ERROR_PAGE_TABLE_WALK_IFETCH = 3,
    MCE_RA_ERROR_PAGE_TABLE_WALK_IFETCH_FOREIGN = 4,
    MCE_RA_ERROR_LOAD = 5,
    MCE_RA_ERROR_STORE = 6,
    MCE_RA_ERROR_PAGE_TABLE_WALK_LOAD_STORE = 7,
    MCE_RA_ERROR_PAGE_TABLE_WALK_LOAD_STORE_FOREIGN = 8,
    MCE_RA_ERROR_LOAD_STORE_FOREIGN = 9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MCE_LinkErrorType {
    MCE_LINK_ERROR_INDETERMINATE = 0,
    MCE_LINK_ERROR_IFETCH_TIMEOUT = 1,
    MCE_LINK_ERROR_PAGE_TABLE_WALK_IFETCH_TIMEOUT = 2,
    MCE_LINK_ERROR_LOAD_TIMEOUT = 3,
    MCE_LINK_ERROR_STORE_TIMEOUT = 4,
    MCE_LINK_ERROR_PAGE_TABLE_WALK_LOAD_STORE_TIMEOUT = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct machine_check_event {
    pub version:8: MCE_Version,
    pub in_use: u8,
    pub severity:8: MCE_Severity,
    pub initiator:8: MCE_Initiator,
    pub error_type:8: MCE_ErrorType,
    pub error_class:8: MCE_ErrorClass,
    pub disposition:8: MCE_Disposition,
    pub sync_error: bool,
    pub cpu: u16,
    pub gpr3: u64,
    pub srr0: u64,
    pub srr1: u64,
    pub ue_error_type:8: MCE_UeErrorType,
    pub effective_address_provided: u8,
    pub physical_address_provided: u8,
    pub ignore_event: u8,
    pub reserved_1: [u8; 4],
    pub effective_address: u64,
    pub physical_address: u64,
    pub reserved_2: [u8; 8],
    pub ue_error: },
    pub slb_error_type:8: MCE_SlbErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub slb_error: },
    pub erat_error_type:8: MCE_EratErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub erat_error: },
    pub tlb_error_type:8: MCE_TlbErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub tlb_error: },
    pub user_error_type:8: MCE_UserErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub user_error: },
    pub ra_error_type:8: MCE_RaErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub ra_error: },
    pub link_error_type:8: MCE_LinkErrorType,
    pub effective_address_provided: u8,
    pub reserved_1: [u8; 6],
    pub effective_address: u64,
    pub reserved_2: [u8; 16],
    pub link_error: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_error_info {
    pub error_type:8: MCE_ErrorType,
    pub ue_error_type:8: MCE_UeErrorType,
    pub slb_error_type:8: MCE_SlbErrorType,
    pub erat_error_type:8: MCE_EratErrorType,
    pub tlb_error_type:8: MCE_TlbErrorType,
    pub user_error_type:8: MCE_UserErrorType,
    pub ra_error_type:8: MCE_RaErrorType,
    pub link_error_type:8: MCE_LinkErrorType,
    pub u: },
    pub severity:8: MCE_Severity,
    pub initiator:8: MCE_Initiator,
    pub error_class:8: MCE_ErrorClass,
    pub sync_error: bool,
    pub ignore_event: bool,
}

pub const MAX_MC_EVT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_info {
    pub mce_nest_count: c_int,
    pub mce_event: [machine_check_event; MAX_MC_EVT],
// Queue for delayed MCE events.
    pub mce_queue_count: c_int,
    pub mce_event_queue: [machine_check_event; MAX_MC_EVT],
// Queue for delayed MCE UE events.
    pub mce_ue_count: c_int,
    pub mce_ue_event_queue: [machine_check_event; MAX_MC_EVT],
}

// Release flags for get_mce_event()

extern "C" {
    pub fn get_mce_event(mce: *mut machine_check_event, release: bool) -> c_int;
}
extern "C" {
    pub fn release_mce_event();
}
extern "C" {
    pub fn machine_check_queue_event();
}
extern "C" {
    pub fn addr_to_pfn(regs: *mut pt_regs, addr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn mce_irq_work_queue();
}
extern "C" {
    pub fn mce_register_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn mce_unregister_notifier(nb: *mut notifier_block) -> c_int;
}

extern "C" {
    pub fn mce_run_irq_context_handlers();
}

extern "C" {
    pub fn set_mce_pending_irq_work();
}
extern "C" {
    pub fn clear_mce_pending_irq_work();
}

extern "C" {
    pub fn flush_and_reload_slb();
}
extern "C" {
    pub fn flush_erat();
}
extern "C" {
    pub fn __machine_check_early_realmode_p7(regs: *mut pt_regs) -> c_long;
}
extern "C" {
    pub fn __machine_check_early_realmode_p8(regs: *mut pt_regs) -> c_long;
}
extern "C" {
    pub fn __machine_check_early_realmode_p9(regs: *mut pt_regs) -> c_long;
}
extern "C" {
    pub fn __machine_check_early_realmode_p10(regs: *mut pt_regs) -> c_long;
}

extern "C" {
    pub fn mce_init();
}

