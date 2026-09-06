//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/mshyperv.h
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
// Linux-specific definitions for managing interactions with Microsoft's
// Hyper-V hypervisor. The definitions in this file are architecture
// independent. See arch/<arch>/include/asm/mshyperv.h for definitions
// that are specific to architecture <arch>.
//
// Definitions that are derived from Hyper-V code or headers should not go in
// this file, but should instead go in the relevant files in include/hyperv.
//
// Copyright (C) 2019, Microsoft, Inc.
//
// Author : Michael Kelley <mikelley@microsoft.com>
//

pub const VTPM_BASE_ADDRESS: c_uint = 0xfed40000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_partition_type {
    HV_PARTITION_TYPE_GUEST,
    HV_PARTITION_TYPE_ROOT,
    HV_PARTITION_TYPE_L1VH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ms_hyperv_info {
    pub features: u32,
    pub priv_high: u32,
    pub ext_features: u32,
    pub misc_features: u32,
    pub hints: u32,
    pub nested_features: u32,
    pub max_vp_index: u32,
    pub max_lp_index: u32,
    pub vtl: u8,
    pub isolation_config_a: u32,
    pub 1: u32 paravisor_present :,
    pub 31: u32 reserved_a1 :,
}

extern "C" {
    pub fn hv_do_hypercall(control: u64, inputaddr: *mut c_void, outputaddr: *mut c_void) -> u64;
}
extern "C" {
    pub fn hv_do_fast_hypercall8(control: u16, input8: u64) -> u64;
}
extern "C" {
    pub fn hv_do_fast_hypercall16(control: u16, input1: u64, input2: u64) -> u64;
}
extern "C" {
    pub fn hv_isolation_type_snp() -> bool;
}
extern "C" {
    pub fn hv_isolation_type_tdx() -> bool;
}
//
// On architectures where Hyper-V doesn't support AEOI (e.g., ARM64),
// it doesn't provide a recommendation flag and AEOI must be disabled.
//

// Helper functions that provide a consistent pattern for checking Hyper-V hypercall status.
// Bits [43:32] of status have 'Reps completed' data.
//
// Rep hypercalls. Callers of this functions are supposed to ensure that
// rep_count, varhead_size, and rep_start comply with Hyper-V hypercall
// definition.
//
// For the typical case where rep_start is 0
// Generate the guest OS identifier as described in the Hyper-V TLFS
extern "C" {
    pub fn hv_get_hypervisor_version(info: *mut hv_hypervisor_version_info) -> c_int;
}
extern "C" {
    pub fn hv_setup_vmbus_handler((*handler)(void): *mut c_void);
}
extern "C" {
    pub fn hv_remove_vmbus_handler();
}
extern "C" {
    pub fn hv_setup_stimer0_handler((*handler)(void): *mut c_void);
}
extern "C" {
    pub fn hv_remove_stimer0_handler();
}
extern "C" {
    pub fn hv_setup_kexec_handler((*handler)(void): *mut c_void);
}
extern "C" {
    pub fn hv_remove_kexec_handler();
}
extern "C" {
    pub fn hv_setup_crash_handler(regs): *mut *mut void (handler)(struct pt_regs);
}
extern "C" {
    pub fn hv_remove_crash_handler();
}
extern "C" {
    pub fn hv_setup_mshv_handler((*handler)(void): *mut c_void);
}

//
// Hypervisor's notion of virtual processor ID is different from
// Linux' notion of CPU ID. This information can only be retrieved
// in the context of the calling CPU. Setup a map for easy access
// to this information.
//
extern "C" {
    pub fn u64(_arg: *mut hv_read_reference_counter)(void) -> extern;
}
// Sentinel value for an uninitialized entry in hv_vp_index array

extern "C" {
    pub fn hv_common_init() -> int __init;
}
extern "C" {
    pub fn hv_get_partition_id() -> void __init;
}
extern "C" {
    pub fn hv_common_free() -> void __init;
}
extern "C" {
    pub fn ms_hyperv_late_init() -> void __init;
}
extern "C" {
    pub fn hv_common_cpu_init(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn hv_common_cpu_die(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn hv_identify_partition_type();
}
//
// hv_cpu_number_to_vp_number() - Map CPU to VP.
// @cpu_number: CPU number in Linux terms
//
// This function returns the mapping between the Linux processor
// number and the hypervisor's virtual processor number, useful
// in making hypercalls and such that talk about specific
// processors.
//
// Return: Virtual processor number in Hyper-V terms
//
// vpset.valid_bank_mask can represent up to HV_MAX_SPARSE_VCPU_BANKS banks
//
// Clear all banks up to the maximum possible bank as hv_tlb_flush_ex
// structs are not cleared between calls, we risk flushing unneeded
// vCPUs otherwise.
//
// Some banks may end up being empty but this is acceptable.
//
// Convert a Linux cpumask into a Hyper-V VPset. In the _skip variant,
// 'func' is called for each CPU present in cpumask.  If 'func' returns
// true, that CPU is skipped -- i.e., that CPU from cpumask is *not
// added to the Hyper-V VPset. If 'func' is NULL, no CPUs are
// skipped.
//
extern "C" {
    pub fn __cpumask_to_vpset(_arg: vpset, _arg: cpus, _arg: NULL) -> return;
}
extern "C" {
    pub fn __cpumask_to_vpset(_arg: vpset, _arg: cpus, _arg: func) -> return;
}

extern "C" {
    pub fn hv_result_to_errno(status: u64) -> c_int;
}
extern "C" {
    pub fn hyperv_report_panic(regs: *mut pt_regs, err: c_long, in_die: bool);
}
extern "C" {
    pub fn hv_is_hyperv_initialized() -> bool;
}
extern "C" {
    pub fn hv_is_hibernation_supported() -> bool;
}
extern "C" {
    pub fn hv_get_isolation_type() -> hv_isolation_type;
}
extern "C" {
    pub fn hv_is_isolation_supported() -> bool;
}
extern "C" {
    pub fn hv_isolation_type_snp() -> bool;
}
extern "C" {
    pub fn hv_ghcb_hypercall(control: u64, input: *mut c_void, output: *mut c_void, input_size: u32) -> u64;
}
extern "C" {
    pub fn hv_tdx_hypercall(control: u64, param1: u64, param2: u64) -> u64;
}
extern "C" {
    pub fn hv_enable_coco_interrupt(cpu: c_uint, vector: c_uint, set: bool);
}
extern "C" {
    pub fn hv_para_set_sint_proxy(enable: bool);
}
extern "C" {
    pub fn hv_para_get_synic_register(reg: c_uint) -> u64;
}
extern "C" {
    pub fn hv_para_set_synic_register(reg: c_uint, val: u64);
}
extern "C" {
    pub fn hyperv_cleanup();
}
extern "C" {
    pub fn hv_query_ext_cap(cap_query: u64) -> bool;
}
extern "C" {
    pub fn hv_setup_dma_ops(dev: *mut device, coherent: bool);
}

extern "C" {
    pub fn hv_root_partition(hv_l1vh_partition(: ) ||) -> return;
}
extern "C" {
    pub fn hv_result_needs_memory(status: u64) -> bool;
}
extern "C" {
    pub fn hv_deposit_memory_node(node: c_int, partition_id: u64, status: u64) -> c_int;
}
extern "C" {
    pub fn hv_call_deposit_pages(node: c_int, partition_id: u64, num_pages: u32) -> c_int;
}
extern "C" {
    pub fn hv_call_add_logical_proc(node: c_int, lp_index: u32, acpi_id: u32) -> c_int;
}
extern "C" {
    pub fn hv_call_notify_all_processors_started() -> c_int;
}
extern "C" {
    pub fn hv_lp_exists(lp_index: u32) -> bool;
}
extern "C" {
    pub fn hv_call_create_vp(node: c_int, partition_id: u64, vp_index: u32, flags: u32) -> c_int;
}

extern "C" {
    pub fn hv_deposit_memory_node(_arg: NUMA_NO_NODE, _arg: partition_id, _arg: status) -> return;
}

extern "C" {
    pub fn get_vtl() -> u8 __init;
}

