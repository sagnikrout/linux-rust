//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/resctrl/mpam_internal.h
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
// Copyright (C) 2025 Arm Ltd.

pub const MPAM_MSC_MAX_NUM_RIS: c_int = 16;

// Macro flag: #define PACKED_FOR_KUNIT

//
// This 'mon' values must not alias an actual monitor, so must be larger than
// U16_MAX, but not be confused with an errno value, so smaller than
// (u32)-SZ_4K.
// USE_PRE_ALLOCATED is used to avoid confusion with an actual monitor.
//

extern "C" {
    pub fn static_branch_likely(_arg: &mpam_enabled) -> return;
}
//
// Structures protected by SRCU may not be freed for a surprising amount of
// time (especially if perf is running). To ensure the MPAM error interrupt can
// tear down all the structures, build a list of objects that can be garbage
// collected once synchronize_srcu() has returned.
// If pdev is non-NULL, use devm_kfree().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_garbage {
// member of mpam_garbage
    pub llist: llist_node,
    pub to_free: *mut c_void,
    pub pdev: *mut platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_msc {
// member of mpam_all_msc
    pub all_msc_list: list_head,
    pub id: c_int,
    pub pdev: *mut platform_device,
// Not modified after mpam_is_enabled() becomes true
    pub iface: mpam_msc_iface,
    pub nrdy_usec: u32,
    pub accessibility: cpumask_t,
    pub has_extd_esr: bool,
    pub reenable_error_ppi: c_int,
    pub error_dev_id: *mut *mut mpam_msc  __percpu,
    pub online_refs: core::sync::atomic::AtomicI32,
//
// probe_lock is only taken during discovery. After discovery these
// properties become read-only and the lists are protected by SRCU.
//
    pub probe_lock: mutex,
    pub probed: bool,
    pub partid_max: u16,
    pub pmg_max: u8,
    pub ris_idxs: c_ulong,
    pub ris_max: u32,
    pub iidr: u32,
    pub quirks: u16,
//
// error_irq_lock is taken when registering/unregistering the error
// interrupt and maniupulating the below flags.
//
    pub error_irq_lock: mutex,
    pub error_irq_req: bool,
    pub error_irq_hw_enabled: bool,
// mpam_msc_ris of this component
    pub ris: list_head,
//
// part_sel_lock protects access to the MSC hardware registers that are
// affected by MPAMCFG_PART_SEL. (including the ID registers that vary
// by RIS).
// If needed, take msc->probe_lock first.
//
    pub part_sel_lock: mutex,
//
// cfg_lock protects the msc configuration and guards against mbwu_state
// save and restore racing.
//
    pub cfg_lock: mutex,
//
// mon_sel_lock protects access to the MSC hardware registers that are
// affected by MPAMCFG_MON_SEL, and the mbwu_state.
// Access to mon_sel is needed from both process and interrupt contexts,
// but is complicated by firmware-backed platforms that can't make any
// access unless they can sleep.
// Always use the mpam_mon_sel_lock() helpers.
// Accesses to mon_sel need to be able to fail if they occur in the wrong
// context.
// If needed, take msc->probe_lock first.
//
    pub _mon_sel_lock: raw_spinlock_t,
    pub _mon_sel_flags: c_ulong,
    pub mapped_hwpage: *mut void __iomem,
    pub mapped_hwpage_sz: usize,
// Values only used on some platforms for quirks
    pub t241_id: u32,
    pub garbage: mpam_garbage,
}

// Returning false here means accesses to mon_sel must fail and report an error.
// Locking will require updating to support a firmware backed interface
// Bits for mpam features bitmaps
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpam_device_features {
    mpam_feat_cpor_part,
    mpam_feat_cmax_softlim,
    mpam_feat_cmax_cmax,
    mpam_feat_cmax_cmin,
    mpam_feat_cmax_cassoc,
    mpam_feat_mbw_part,
    mpam_feat_mbw_min,
    mpam_feat_mbw_max,
    mpam_feat_mbw_prop,
    mpam_feat_intpri_part,
    mpam_feat_intpri_part_0_low,
    mpam_feat_dspri_part,
    mpam_feat_dspri_part_0_low,
    mpam_feat_msmon,
    mpam_feat_msmon_csu,
    mpam_feat_msmon_csu_capture,
    mpam_feat_msmon_csu_xcl,
    mpam_feat_msmon_mbwu,
    mpam_feat_msmon_mbwu_31counter,
    mpam_feat_msmon_mbwu_44counter,
    mpam_feat_msmon_mbwu_63counter,
    mpam_feat_msmon_mbwu_capture,
    mpam_feat_msmon_mbwu_rwbw,
    mpam_feat_partid_nrw,
    MPAM_FEATURE_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_props {
    pub MPAM_FEATURE_LAST): DECLARE_BITMAP(features,,
    pub cpbm_wd: u16,
    pub mbw_pbm_bits: u16,
    pub bwa_wd: u16,
    pub cmax_wd: u16,
    pub cassoc_wd: u16,
    pub intpri_wd: u16,
    pub dspri_wd: u16,
    pub num_csu_mon: u16,
    pub num_mbwu_mon: u16,
//
// Kunit tests use memset() to set up feature combinations that should be
// removed, and will false-positive if the compiler introduces padding that
// isn't cleared during sanitisation.
//
    pub PACKED_FOR_KUNIT: },

//
// The non-atomic get/set operations are used because if struct mpam_props is
// packed, the alignment requirements for atomics aren't met.
//

// Workaround bits for msc->quirks
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpam_device_quirks {
    T241_SCRUB_SHADOW_REGS,
    T241_FORCE_MBW_MIN_TO_ONE,
    T241_MBW_COUNTER_SCALE_64,
    IGNORE_CSU_NRDY,
    MPAM_QUIRK_LAST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_quirk {
    pub quirk): *const *const *const int (init)(struct mpam_msc msc, struct mpam_quirk,
    pub iidr: u32,
    pub iidr_mask: u32,
    pub workaround: mpam_device_quirks,
}

// The values for MSMON_CFG_MBWU_FLT.RWBW
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mon_filter_options {
    COUNT_BOTH	= 0,
    COUNT_WRITE	= 1,
    COUNT_READ	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mon_cfg {
//
// mon must be large enough to hold out of range values like
// USE_PRE_ALLOCATED
//
    pub mon: u32,
    pub pmg: u8,
    pub match_pmg: bool,
    pub csu_exclude_clean: bool,
    pub partid: u32,
    pub opts: mon_filter_options,
}

// Changes to msmon_mbwu_state are protected by the msc's mon_sel_lock.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msmon_mbwu_state {
    pub enabled: bool,
    pub reset_on_next_read: bool,
    pub cfg: mon_cfg,
//
// The value to add to the new reading to account for power management,
// and overflow.
//
    pub correction: u64,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_class {
// mpam_components in this class
    pub components: list_head,
    pub affinity: cpumask_t,
    pub props: mpam_props,
    pub nrdy_usec: u32,
    pub quirks: u16,
    pub level: u8,
    pub type: mpam_class_types,
// member of mpam_classes
    pub classes_list: list_head,
    pub ida_csu_mon: ida,
    pub ida_mbwu_mon: ida,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_config {
// Which configuration values are valid.
    pub MPAM_FEATURE_LAST): DECLARE_BITMAP(features,,
    pub cpbm: u32,
    pub mbw_pbm: u32,
    pub mbw_max: u16,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_component {
    pub comp_id: u32,
// mpam_vmsc in this component
    pub vmsc: list_head,
    pub affinity: cpumask_t,
//
// Array of configuration values, indexed by partid.
// Read from cpuhp callbacks, hold the cpuhp lock when writing.
//
    pub cfg: *mut mpam_config,
// member of mpam_class:components
    pub class_list: list_head,
// parent:
    pub class: *mut mpam_class,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_vmsc {
// member of mpam_component:vmsc_list
    pub comp_list: list_head,
// mpam_msc_ris in this vmsc
    pub ris: list_head,
    pub props: mpam_props,
// All RIS in this vMSC are members of this MSC
    pub msc: *mut mpam_msc,
// parent:
    pub comp: *mut mpam_component,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_msc_ris {
    pub ris_idx: u8,
    pub idr: u64,
    pub props: mpam_props,
    pub in_reset_state: bool,
    pub affinity: cpumask_t,
// member of mpam_vmsc:ris
    pub vmsc_list: list_head,
// member of mpam_msc:ris
    pub msc_list: list_head,
// parent:
    pub vmsc: *mut mpam_vmsc,
// msmon mbwu configuration is preserved over reset
    pub mbwu_state: *mut msmon_mbwu_state,
    pub garbage: mpam_garbage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_resctrl_dom {
    pub ctrl_comp: *mut mpam_component,
//
// There is no single mon_comp because different events may be backed
// by different class/components. mon_comp is indexed by the event
// number.
//
    pub mon_comp: [*mut mpam_component; QOS_NUM_EVENTS],
    pub resctrl_ctrl_dom: rdt_ctrl_domain,
    pub resctrl_mon_dom: rdt_l3_mon_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_resctrl_res {
    pub class: *mut mpam_class,
    pub resctrl_res: rdt_resource,
    pub cdp_enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpam_resctrl_mon {
    pub class: *mut mpam_class,
// Array of allocated MBWU monitors, indexed by (closid, rmid).
    pub mbwu_idx_to_mon: *mut c_int,
// Array of assigned MBWU monitors, indexed by resctrl's cntr_id.
    pub assigned_counters: *mut c_int,
}

// List of all classes - protected by srcu
// System wide partid/pmg values
// Scheduled work callback to enable mpam once all MSC have been probed
extern "C" {
    pub fn mpam_enable(work: *mut work_struct);
}
extern "C" {
    pub fn mpam_disable(work: *mut work_struct);
}
// Reset all the RIS in a class under cpus_read_lock()
extern "C" {
    pub fn mpam_reset_class_locked(class: *mut mpam_class);
}
extern "C" {
    pub fn mpam_msmon_reset_mbwu(comp: *mut mpam_component, ctx: *mut mon_cfg);
}

extern "C" {
    pub fn mpam_resctrl_setup() -> c_int;
}
extern "C" {
    pub fn mpam_resctrl_exit();
}
extern "C" {
    pub fn mpam_resctrl_online_cpu(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn mpam_resctrl_offline_cpu(cpu: c_uint);
}
extern "C" {
    pub fn mpam_resctrl_teardown_class(class: *mut mpam_class);
}

//
// MPAM MSCs have the following register layout. See:
// Arm Memory System Resource Partitioning and Monitoring (MPAM) System
// Component Specification.
// https://developer.arm.com/documentation/ihi0099/aa
//
pub const MPAM_ARCHITECTURE_V1: c_uint = 0x10;
// Memory mapped control pages
// ID Register offsets in the memory mapped page
pub const MPAMF_IDR: c_uint = 0x0000  /* features id register */;
pub const MPAMF_IIDR: c_uint = 0x0018  /* implementer id register */;
pub const MPAMF_AIDR: c_uint = 0x0020  /* architectural id register */;
pub const MPAMF_IMPL_IDR: c_uint = 0x0028  /* imp-def partitioning */;
pub const MPAMF_CPOR_IDR: c_uint = 0x0030  /* cache-portion partitioning */;
pub const MPAMF_CCAP_IDR: c_uint = 0x0038  /* cache-capacity partitioning */;
pub const MPAMF_MBW_IDR: c_uint = 0x0040  /* mem-bw partitioning */;
pub const MPAMF_PRI_IDR: c_uint = 0x0048  /* priority partitioning */;
pub const MPAMF_MSMON_IDR: c_uint = 0x0080  /* performance monitoring features */;
pub const MPAMF_CSUMON_IDR: c_uint = 0x0088  /* cache-usage monitor */;
pub const MPAMF_MBWUMON_IDR: c_uint = 0x0090  /* mem-bw usage monitor */;
pub const MPAMF_PARTID_NRW_IDR: c_uint = 0x0050  /* partid-narrowing */;
// Configuration and Status Register offsets in the memory mapped page
pub const MPAMCFG_PART_SEL: c_uint = 0x0100  /* partid to configure */;
pub const MPAMCFG_CPBM: c_uint = 0x1000  /* cache-portion config */;
pub const MPAMCFG_CMAX: c_uint = 0x0108  /* cache-capacity config */;
pub const MPAMCFG_CMIN: c_uint = 0x0110  /* cache-capacity config */;
pub const MPAMCFG_CASSOC: c_uint = 0x0118  /* cache-associativity config */;
pub const MPAMCFG_MBW_MIN: c_uint = 0x0200  /* min mem-bw config */;
pub const MPAMCFG_MBW_MAX: c_uint = 0x0208  /* max mem-bw config */;
pub const MPAMCFG_MBW_WINWD: c_uint = 0x0220  /* mem-bw accounting window config */;
pub const MPAMCFG_MBW_PBM: c_uint = 0x2000  /* mem-bw portion bitmap config */;
pub const MPAMCFG_PRI: c_uint = 0x0400  /* priority partitioning config */;
pub const MPAMCFG_MBW_PROP: c_uint = 0x0500  /* mem-bw stride config */;
pub const MPAMCFG_INTPARTID: c_uint = 0x0600  /* partid-narrowing config */;
pub const MSMON_CFG_MON_SEL: c_uint = 0x0800  /* monitor selector */;
pub const MSMON_CFG_CSU_FLT: c_uint = 0x0810  /* cache-usage monitor filter */;
pub const MSMON_CFG_CSU_CTL: c_uint = 0x0818  /* cache-usage monitor config */;
pub const MSMON_CFG_MBWU_FLT: c_uint = 0x0820  /* mem-bw monitor filter */;
pub const MSMON_CFG_MBWU_CTL: c_uint = 0x0828  /* mem-bw monitor config */;
pub const MSMON_CSU: c_uint = 0x0840  /* current cache-usage */;
pub const MSMON_CSU_CAPTURE: c_uint = 0x0848  /* last cache-usage value captured */;
pub const MSMON_MBWU: c_uint = 0x0860  /* current mem-bw usage value */;
pub const MSMON_MBWU_CAPTURE: c_uint = 0x0868  /* last mem-bw value captured */;
pub const MSMON_MBWU_L: c_uint = 0x0880  /* current long mem-bw usage value */;
pub const MSMON_MBWU_L_CAPTURE: c_uint = 0x0890  /* last long mem-bw value captured */;
pub const MSMON_CAPT_EVNT: c_uint = 0x0808  /* signal a capture event */;
pub const MPAMF_ESR: c_uint = 0x00F8  /* error status register */;
pub const MPAMF_ECR: c_uint = 0x00F0  /* error control register */;
// MPAMF_IDR - MPAM features ID register

// MPAMF_MSMON_IDR - MPAM performance monitoring ID register

// MPAMF_CPOR_IDR - MPAM features cache portion partitioning ID register

// MPAMF_CCAP_IDR - MPAM features cache capacity partitioning ID register

// MPAMF_MBW_IDR - MPAM features memory bandwidth partitioning ID register

// MPAMF_PRI_IDR - MPAM features priority partitioning ID register

// MPAMF_CSUMON_IDR - MPAM cache storage usage monitor ID register

// MPAMF_MBWUMON_IDR - MPAM memory bandwidth usage monitor ID register

// MPAMF_PARTID_NRW_IDR - MPAM PARTID narrowing ID register

// MPAMF_IIDR - MPAM implementation ID register

// MPAMF_AIDR - MPAM architecture ID register

// MPAMCFG_PART_SEL - MPAM partition configuration selection register

// MPAMCFG_CASSOC - MPAM cache maximum associativity partition configuration register

// MPAMCFG_CMAX - MPAM cache capacity configuration register

// MPAMCFG_CMIN - MPAM cache capacity configuration register

//
// MPAMCFG_MBW_MIN - MPAM memory minimum bandwidth partitioning configuration
// register
//

//
// MPAMCFG_MBW_MAX - MPAM memory maximum bandwidth partitioning configuration
// register
//

//
// MPAMCFG_MBW_WINWD - MPAM memory bandwidth partitioning window width
// register
//

// MPAMCFG_PRI - MPAM priority partitioning configuration register

//
// MPAMCFG_MBW_PROP - Memory bandwidth proportional stride partitioning
// configuration register
//

//
// MPAMCFG_INTPARTID - MPAM internal partition narrowing configuration register
//

// MSMON_CFG_MON_SEL - Memory system performance monitor selection register

// MPAMF_ESR - MPAM Error Status Register

// MPAMF_ECR - MPAM Error Control Register

// Error conditions in accessing memory mapped registers
pub const MPAM_ERRCODE_NONE: c_int = 0;
pub const MPAM_ERRCODE_PARTID_SEL_RANGE: c_int = 1;
pub const MPAM_ERRCODE_REQ_PARTID_RANGE: c_int = 2;
pub const MPAM_ERRCODE_MSMONCFG_ID_RANGE: c_int = 3;
pub const MPAM_ERRCODE_REQ_PMG_RANGE: c_int = 4;
pub const MPAM_ERRCODE_MONITOR_RANGE: c_int = 5;
pub const MPAM_ERRCODE_INTPARTID_RANGE: c_int = 6;
pub const MPAM_ERRCODE_UNEXPECTED_INTERNAL: c_int = 7;
pub const MPAM_ERRCODE_UNDEFINED_RIS_PART_SEL: c_int = 8;
pub const MPAM_ERRCODE_RIS_NO_CONTROL: c_int = 9;
pub const MPAM_ERRCODE_UNDEFINED_RIS_MON_SEL: c_int = 10;
pub const MPAM_ERRCODE_RIS_NO_MONITOR: c_int = 11;
//
// MSMON_CFG_CSU_CTL - Memory system performance monitor configure cache storage
// usage monitor control register
// MSMON_CFG_MBWU_CTL - Memory system performance monitor configure memory
// bandwidth usage monitor control register
//

pub const MSMON_CFG_MBWU_CTL_TYPE_MBWU: c_uint = 0x42;
pub const MSMON_CFG_CSU_CTL_TYPE_CSU: c_uint = 0x43;
//
// MSMON_CFG_CSU_FLT -  Memory system performance monitor configure cache storage
// usage monitor filter register
// MSMON_CFG_MBWU_FLT - Memory system performance monitor configure memory
// bandwidth usage monitor filter register
//

//
// MSMON_CSU - Memory system performance monitor cache storage usage monitor
// register
// MSMON_CSU_CAPTURE -  Memory system performance monitor cache storage usage
// capture register
// MSMON_MBWU  - Memory system performance monitor memory bandwidth usage
// monitor register
// MSMON_MBWU_CAPTURE - Memory system performance monitor memory bandwidth usage
// capture register
//

//
// MSMON_CAPT_EVNT - Memory system performance monitoring capture event
// generation register
//

