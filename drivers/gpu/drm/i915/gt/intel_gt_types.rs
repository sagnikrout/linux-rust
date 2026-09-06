//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/intel_gt_types.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_mmio_range {
    pub start: u32,
    pub end: u32,
}

//
// The hardware has multiple kinds of multicast register ranges that need
// special register steering (and future platforms are expected to add
// additional types).
//
// During driver startup, we initialize the steering control register to
// direct reads to a slice/subslice that are valid for the 'subslice' class
// of multicast registers.  If another type of steering does not have any
// overlap in valid steering targets with 'subslice' style registers, we will
// need to explicitly re-steer reads of registers of the other type.
//
// Only the replication types that may need additional non-default steering
// are listed here.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_steering_type {
    L3BANK,
    MSLICE,
    LNCF,
    GAM,
    DSS,
    OADDRM,

//
// On some platforms there are multiple types of MCR registers that
// will always return a non-terminated value at instance (0, 0).  We'll
// lump those all into a single category to keep things simple.
//
    INSTANCE0,

    NUM_STEERING_TYPES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_submission_method {
    INTEL_SUBMISSION_RING,
    INTEL_SUBMISSION_ELSP,
    INTEL_SUBMISSION_GUC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gt_defaults {
    pub min_freq: u32,
    pub max_freq: u32,
    pub rps_up_threshold: u8,
    pub rps_down_threshold: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gt_type {
    GT_PRIMARY,
    GT_TILE,
    GT_MEDIA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt {
    pub i915: *mut drm_i915_private,
    pub name: *const c_char,
    pub type: intel_gt_type,
    pub uncore: *mut intel_uncore,
    pub ggtt: *mut i915_ggtt,
    pub uc: intel_uc,
    pub gsc: intel_gsc,
    pub wopcm: intel_wopcm,
// Serialize global tlb invalidations
    pub invalidate_lock: mutex,
//
// Batch TLB invalidations
//
// After unbinding the PTE, we need to ensure the TLB
// are invalidated prior to releasing the physical pages.
// But we only need one such invalidation for all unbinds,
// so we track how many TLB invalidations have been
// performed since unbind the PTE and only emit an extra
// invalidate if no full barrier has been passed.
//
    pub seqno: seqcount_mutex_t,
    pub tlb: },
    pub wa_list: i915_wa_list,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt_timelines {
    pub /: *mut *mut spinlock_t lock; / protects active_list,
    pub active_list: list_head,
    pub timelines: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt_requests {
//
// We leave the user IRQ off as much as possible,
// but this means that requests will finish and never
// be retired once the system goes idle. Set a timer to
// fire periodically while the ring is running. When it
// fires, go retire requests.
//
    pub retire_work: delayed_work,
    pub requests: },
    pub list: llist_head,
    pub work: work_struct,
    pub watchdog: },
    pub wakeref: intel_wakeref,
    pub user_wakeref: core::sync::atomic::AtomicI32,
    pub closed_vma: list_head,
    pub /: *mut *mut spinlock_t closed_lock; / guards the list of closed_vma,
    pub last_init_time: ktime_t,
    pub reset: intel_reset,
//
// Is the GPU currently considered idle, or busy executing
// userspace requests? Whilst idle, we allow runtime power
// management to power down the hardware and display clocks.
// In order to reduce the effect on performance, there
// is a slight delay before we do so.
//
    pub awake: intel_wakeref_t,
    pub clock_frequency: u32,
    pub clock_period_ns: u32,
    pub llc: intel_llc,
    pub rc6: intel_rc6,
    pub rps: intel_rps,
    pub irq_lock: *mut spinlock_t,
    pub gt_imr: u32,
    pub pm_ier: u32,
    pub pm_imr: u32,
    pub pm_guc_events: u32,
    pub active: bool,
//
// @lock: Lock protecting the below fields.
//
    pub lock: seqcount_mutex_t,
//
// @total: Total time this engine was busy.
//
// Accumulated time not counting the most recent block in cases
// where engine is currently busy (active > 0).
//
    pub total: ktime_t,
//
// @start: Timestamp of the last idle to active transition.
//
// Idle is defined as active == 0, active is active > 0.
//
    pub start: ktime_t,
    pub stats: },
    pub engine: [*mut intel_engine_cs; I915_NUM_ENGINES],
    pub 1]: [MAX_ENGINE_INSTANCE +,
    pub submission_method: intel_submission_method,
//
// Mask of the non fused CCS slices
// to be used for the load balancing
//
    pub cslices: intel_engine_mask_t,
    pub ccs: },
//
// Default address space (either GGTT or ppGTT depending on arch).
//
// Reserved for exclusive use by the kernel.
//
    pub vm: *mut i915_address_space,
//
// A pool of objects to use as shadow copies of client batch buffers
// when the command parser is enabled. Prevents the client from
// modifying the batch contents after software parsing.
//
// Buffers older than 1s are periodically reaped from the pool,
// or may be reclaimed by the shrinker before then.
//
    pub buffer_pool: intel_gt_buffer_pool,
    pub scratch: *mut i915_vma,
    pub migrate: intel_migrate,
    pub steering_table: [*const intel_mmio_range; NUM_STEERING_TYPES],
    pub groupid: u8,
    pub instanceid: u8,
    pub default_steering: },
//
// @mcr_lock: Protects the MCR steering register
//
// Protects the MCR steering register (e.g., GEN8_MCR_SELECTOR).
// Should be taken before uncore->lock in cases where both are desired.
//
    pub mcr_lock: spinlock_t,
//
// Base of per-tile GTTMMADR where we can derive the MMIO and the GGTT.
//
    pub phys_addr: phys_addr_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt_info {
    pub id: c_uint,
    pub engine_mask: intel_engine_mask_t,
    pub l3bank_mask: u32,
    pub num_engines: u8,
// General presence of SFC units
    pub sfc_mask: u8,
// Media engine access to SFC per instance
    pub vdbox_sfc_access: u8,
// Slice/subslice/EU info
    pub sseu: sseu_dev_info,
    pub mslice_mask: c_ulong,
// @hwconfig: hardware configuration data
    pub hwconfig: intel_hwconfig,
    pub info: },
    pub uc_index: u8,
    pub /: *mut *mut u8 wb_index; / Only used on HAS_L3_CCS_READ() platforms,
    pub mocs: },
// gt/gtN sysfs
    pub sysfs_gt: kobject,
// sysfs defaults per gt
    pub defaults: gt_defaults,
    pub sysfs_defaults: *mut kobject,
    pub wedge: work_struct,
    pub perf: i915_perf_gt,
// link: &ggtt.gt_list
    pub ggtt_link: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt_definition {
    pub type: intel_gt_type,
    pub name: *mut c_char,
    pub mapping_base: u32,
    pub gsi_offset: u32,
    pub engine_mask: intel_engine_mask_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_gt_scratch_field {
// 8 bytes
    INTEL_GT_SCRATCH_FIELD_DEFAULT = 0,

// 8 bytes
    INTEL_GT_SCRATCH_FIELD_RENDER_FLUSH = 128,

// 8 bytes
    INTEL_GT_SCRATCH_FIELD_COHERENTL3_WA = 256,
}

