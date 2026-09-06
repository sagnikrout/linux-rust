//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/resctrl/internal.h
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

pub const L3_QOS_CDP_ENABLE: c_uint = 0x01ULL;
pub const L2_QOS_CDP_ENABLE: c_uint = 0x01ULL;
pub const MBM_CNTR_WIDTH_BASE: c_int = 24;
pub const MBA_IS_LINEAR: c_uint = 0x4;
pub const MBM_CNTR_WIDTH_OFFSET_AMD: c_int = 20;
// Hygon MBM counter width as an offset from MBM_CNTR_WIDTH_BASE
pub const MBM_CNTR_WIDTH_OFFSET_HYGON: c_int = 8;

//
// With the above fields in use 62 bits remain in MSR_IA32_QM_CTR for
// data to be returned. The counter width is discovered from the hardware
// as an offset from MBM_CNTR_WIDTH_BASE.
//

//
// struct arch_mbm_state - values used to compute resctrl_arch_rmid_read()s
// return value.
// @chunks:	Total data moved (multiply by rdt_group.mon_scale to get bytes)
// @prev_msr:	Value of IA32_QM_CTR last time it was read for the RMID used to
// find this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_mbm_state {
    pub chunks: u64,
    pub prev_msr: u64,
}

// Setting bit 0 in L3_QOS_EXT_CFG enables the ABMC feature.
pub const ABMC_ENABLE_BIT: c_int = 0;
//
// Qos Event Identifiers.
//

// Setting bit 1 in MSR_IA32_L3_QOS_EXT_CFG enables the SDCIAE feature.
pub const SDCIAE_ENABLE_BIT: c_int = 1;
//
// struct rdt_hw_ctrl_domain - Arch private attributes of a set of CPUs that share
// a resource for a control function
// @d_resctrl:	Properties exposed to the resctrl file system
// @ctrl_val:	array of cache or mem ctrl values (indexed by CLOSID)
//
// Members of this structure are accessed via helpers that provide abstraction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_hw_ctrl_domain {
    pub d_resctrl: rdt_ctrl_domain,
    pub ctrl_val: *mut u32,
}

//
// struct rdt_hw_l3_mon_domain - Arch private attributes of a set of CPUs sharing
// RDT_RESOURCE_L3 monitoring
// @d_resctrl:		Properties exposed to the resctrl file system
// @arch_mbm_states:	Per-event pointer to the MBM event's saved state.
// An MBM event's state is an array of struct arch_mbm_state
// indexed by RMID on x86.
//
// Members of this structure are accessed via helpers that provide abstraction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_hw_l3_mon_domain {
    pub d_resctrl: rdt_l3_mon_domain,
    pub arch_mbm_states: [*mut arch_mbm_state; QOS_NUM_L3_MBM_EVENTS],
}

extern "C" {
    pub fn container_of(_arg: r, rdt_hw_ctrl_domain: struct, _arg: d_resctrl) -> return;
}
extern "C" {
    pub fn container_of(_arg: r, rdt_hw_l3_mon_domain: struct, _arg: d_resctrl) -> return;
}
//
// struct rdt_perf_pkg_mon_domain - CPUs sharing an package scoped resctrl monitor resource
// @hdr:	common header for different domain types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_perf_pkg_mon_domain {
    pub hdr: rdt_domain_hdr,
}

//
// struct msr_param - set a range of MSRs from a domain
// @res:       The resource to use
// @dom:       The domain to update
// @low:       Beginning index from base MSR
// @high:      End index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_param {
    pub res: *mut rdt_resource,
    pub dom: *mut rdt_ctrl_domain,
    pub low: u32,
    pub high: u32,
}

//
// struct rdt_hw_resource - arch private attributes of a resctrl resource
// @r_resctrl:		Attributes of the resource used directly by resctrl.
// @num_closid:		Maximum number of closid this hardware can support,
// regardless of CDP. This is exposed via
// resctrl_arch_get_num_closid() to avoid confusion
// with struct resctrl_schema's property of the same name,
// which has been corrected for features like CDP.
// @msr_base:		Base MSR address for CBMs
// @msr_update:		Function pointer to update QOS MSRs
// @mon_scale:		cqm counter * mon_scale = occupancy in bytes
// @mbm_width:		Monitor width, to detect and correct for overflow.
// @cdp_enabled:	CDP state of this resource
// @mbm_cntr_assign_enabled:	ABMC feature is enabled
// @sdciae_enabled:	SDCIAE feature (backing "io_alloc") is enabled.
//
// Members of this structure are either private to the architecture
// e.g. mbm_width, or accessed via helpers that provide abstraction. e.g.
// msr_update and msr_base.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdt_hw_resource {
    pub r_resctrl: rdt_resource,
    pub num_closid: u32,
    pub msr_base: c_uint,
    pub m): *mut *mut void (msr_update)(struct msr_param,
    pub mon_scale: c_uint,
    pub mbm_width: c_uint,
    pub cdp_enabled: bool,
    pub mbm_cntr_assign_enabled: bool,
    pub sdciae_enabled: bool,
}

extern "C" {
    pub fn container_of(_arg: r, rdt_hw_resource: struct, _arg: r_resctrl) -> return;
}
extern "C" {
    pub fn arch_mon_domain_online(r: *mut rdt_resource, d: *mut rdt_l3_mon_domain);
}
// CPUID.(EAX=10H, ECX=ResID=1).EAX
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid_0x10_1_eax {
    pub cbm_len:5: c_uint,
    pub split: },
    pub full: c_uint,
}

// CPUID.(EAX=10H, ECX=ResID=3).EAX
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid_0x10_3_eax {
    pub max_delay:12: c_uint,
    pub split: },
    pub full: c_uint,
}

// CPUID.(EAX=10H, ECX=ResID).ECX
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid_0x10_x_ecx {
    pub reserved:3: c_uint,
    pub noncont:1: c_uint,
    pub split: },
    pub full: c_uint,
}

// CPUID.(EAX=10H, ECX=ResID).EDX
#[repr(C)]
#[derive(Copy, Clone)]
pub union cpuid_0x10_x_edx {
    pub cos_max:16: c_uint,
    pub split: },
    pub full: c_uint,
}

//
// ABMC counters are configured by writing to MSR_IA32_L3_QOS_ABMC_CFG.
//
// @bw_type		: Event configuration that represents the memory
// transactions being tracked by the @cntr_id.
// @bw_src		: Bandwidth source (RMID or CLOSID).
// @reserved1		: Reserved.
// @is_clos		: @bw_src field is a CLOSID (not an RMID).
// @cntr_id		: Counter identifier.
// @reserved		: Reserved.
// @cntr_en		: Counting enable bit.
// @cfg_en		: Configuration enable bit.
//
// Configuration and counting:
// Counter can be configured across multiple writes to MSR. Configuration
// is applied only when @cfg_en = 1. Counter @cntr_id is reset when the
// configuration is applied.
// @cfg_en = 1, @cntr_en = 0 : Apply @cntr_id configuration but do not
// count events.
// @cfg_en = 1, @cntr_en = 1 : Apply @cntr_id configuration and start
// counting events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union l3_qos_abmc_cfg {
    pub 1: cfg_en :,
    pub split: },
    pub full: c_ulong,
}

extern "C" {
    pub fn rdt_ctrl_update(arg: *mut c_void);
}
extern "C" {
    pub fn rdt_get_l3_mon_config(r: *mut rdt_resource) -> c_int;
}
extern "C" {
    pub fn rdt_cpu_has(flag: c_int) -> bool;
}
extern "C" {
    pub fn intel_rdt_mbm_apply_quirk() -> void __init;
}
extern "C" {
    pub fn rdt_domain_reconfigure_cdp(r: *mut rdt_resource);
}
extern "C" {
    pub fn resctrl_arch_mbm_cntr_assign_set_one(r: *mut rdt_resource);
}

extern "C" {
    pub fn intel_aet_get_events() -> bool;
}
extern "C" {
    pub fn intel_aet_exit() -> void __exit;
}
extern "C" {
    pub fn intel_aet_read_event(domid: c_int, rmid: u32, arch_priv: *mut c_void, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn intel_handle_aet_option(force_off: bool, tok: *mut c_char) -> bool;
}

