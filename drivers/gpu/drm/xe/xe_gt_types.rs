//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_types.h
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
// Copyright © 2022-2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_gt_type {
    XE_GT_TYPE_UNINITIALIZED,
    XE_GT_TYPE_MAIN,
    XE_GT_TYPE_MEDIA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_gt_eu_type {
    XE_GT_EU_TYPE_SIMD8,
    XE_GT_EU_TYPE_SIMD16,
}

pub const XE_MAX_DSS_FUSE_REGS: c_int = 4;

pub const XE_MAX_EU_FUSE_REGS: c_int = 1;

pub const XE_MAX_L3_BANK_MASK_BITS: c_int = 64;
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
pub enum xe_steering_type {
    L3BANK,
    NODE,
    MSLICE,
    LNCF,
    DSS,
    OADDRM,
    SQIDI_PSMI,

//
// Although most GAM ranges must be steered to (0,0) and thus use the
// INSTANCE0 type farther down, some platforms have special rules
// for specific subtypes that require steering to (1,0) instead.
//
    GAM1,

//
// On some platforms there are multiple types of MCR registers that
// will always return a non-terminated value at instance (0, 0).  We'll
// lump those all into a single category to keep things simple.
//
    INSTANCE0,

//
// Register ranges that don't need special steering for each register:
// it's sufficient to keep the HW-default for the selector, or only
// change it once, on GT initialization. This needs to be the last
// steering type.
//
    IMPLICIT_STEERING,
    NUM_STEERING_TYPES
}

//
// struct xe_gt - A "Graphics Technology" unit of the GPU
//
// A GT ("Graphics Technology") is the subset of a GPU primarily responsible
// for implementing the graphics, compute, and/or media IP.  It encapsulates
// the hardware engines, programmable execution units, and GuC.   Each GT has
// its own handling of power management (RC6+forcewake) and multicast register
// steering.
//
// A GPU/tile may have a single GT that supplies all graphics, compute, and
// media functionality, or the graphics/compute and media may be split into
// separate GTs within a tile.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt {
// @tile: Backpointer to GT's tile
    pub tile: *mut xe_tile,
// @info: GT info
// @info.type: type of GT
    pub type: xe_gt_type,
// @info.reference_clock: clock frequency
    pub reference_clock: u32,
// @info.timestamp_base: GT timestamp base
    pub timestamp_base: u32,
//
// @info.engine_mask: mask of engines present on GT. Some of
// them may be reserved in runtime and not available for user.
// See @user_engines.mask
//
    pub engine_mask: u64,
// @info.gmdid: raw GMD_ID value from hardware
    pub gmdid: u32,
//
// @info.multi_queue_engine_class_mask: Bitmask of engine classes with
// multi queue support enabled.
//
    pub multi_queue_engine_class_mask: u16,
// @info.id: Unique ID of this GT within the PCI Device
    pub id: u8,
// @info.has_indirect_ring_state: GT has indirect ring state support
    pub has_indirect_ring_state:1: u8,
//
// @info.has_uncorrectable_error_reporting: GT has uncorrectable
// error reporting support
//
    pub has_uncorrectable_error_reporting:1: u8,
//
// @info.has_xe2_blt_instructions: GT supports Xe2-style MEM_SET
// and MEM_COPY blitter functionality.  Note that despite the
// name, some Xe1 platforms may also support this "Xe2-style"
// feature.
//
    pub has_xe2_blt_instructions:1: u8,
//
// @info.num_geometry_xecore_fuse_regs: Number of 32b-bit fuse
// registers the geometry XeCore mask spans.
//
    pub num_geometry_xecore_fuse_regs: u8,
//
// @info.num_compute_xecore_fuse_regs: Number of 32b-bit fuse
// registers the compute XeCore mask spans.
//
    pub num_compute_xecore_fuse_regs: u8,
    pub info: },

// @stats: GT stats
    pub stats: *mut xe_gt_stats __percpu,

//
// @mmio: mmio info for GT.  All GTs within a tile share the same
// register space, but have their own copy of GSI registers at a
// specific offset.
//
    pub mmio: xe_mmio,
//
// @pm: power management info for GT.  The driver uses the GT's
// "force wake" interface to wake up specific parts of the GT hardware
// from C6 sleep states and ensure the hardware remains awake while it
// is being actively used.
//
// @pm.fw: force wake for GT
    pub fw: xe_force_wake,
    pub pm: },
// @sriov: virtualization data related to GT
// @sriov.pf: PF data. Valid only if driver is running as PF
    pub pf: xe_gt_sriov_pf,
// @sriov.vf: VF data. Valid only if driver is running as VF
    pub vf: xe_gt_sriov_vf,
    pub sriov: },
//
// @reg_sr: table with registers to be restored on GT init/resume/reset
//
    pub reg_sr: xe_reg_sr,
// @reset: state for GT resets
//
// @reset.worker: work so GT resets can done async allowing to reset
// code to safely flush all code paths
//
    pub worker: work_struct,
    pub reset: },
// @tlb_inval: TLB invalidation state
    pub tlb_inval: xe_tlb_inval,
//
// @ccs_mode: Number of compute engines enabled.
// Allows fixed mapping of available compute slices to compute engines.
// By default only the first available compute engine is enabled and all
// available compute slices are allocated to it.
//
    pub ccs_mode: u32,
// @usm: unified shared memory state
//
// @usm.bb_pool: Pool from which batchbuffers, for USM operations
// (e.g. migrations, fixing page tables), are allocated.
// Dedicated pool needed so USM operations do not get blocked
// behind any user operations which may have resulted in a
// fault.
//
    pub bb_pool: *mut xe_sa_manager,
//
// @usm.paging_hwe0: The first designated paging engine.
// This is some reserved BCS instance used for USM operations
// (e.g. migrations, fixing page tables)
//
    pub paging_hwe0: *mut xe_hw_engine,
//
// @usm.paging_logical_mask: logical mask of paging engines.
// Should be densely populated.
//
    pub paging_logical_mask: u32,
    pub usm: },
// @ordered_wq: used to serialize GT resets and TDRs
    pub ordered_wq: *mut workqueue_struct,
// @uc: micro controllers on the GT
    pub uc: xe_uc,
// @gtidle: idle properties of GT
    pub gtidle: xe_gt_idle,
// @exec_queue_ops: submission backend exec queue operations
    pub exec_queue_ops: *const xe_exec_queue_ops,
//
// @ring_ops: ring operations for this hw engine (1 per engine class)
//
    pub ring_ops: [*const xe_ring_ops; XE_ENGINE_CLASS_MAX],
// @fence_irq: fence IRQs (1 per engine class)
    pub fence_irq: [xe_hw_fence_irq; XE_ENGINE_CLASS_MAX],
// @default_lrc: default LRC state
    pub default_lrc: [*mut c_void; XE_ENGINE_CLASS_MAX],
// @hw_engines: hardware engines on the GT
    pub hw_engines: [xe_hw_engine; XE_NUM_HW_ENGINES],
// @eclass: per hardware engine class interface on the GT
    pub eclass: [xe_hw_engine_class_intf; XE_ENGINE_CLASS_MAX],
// @sysfs: sysfs' kobj used by xe_gt_sysfs
    pub sysfs: *mut kobject,
// @freq: Main GT freq sysfs control
    pub freq: *mut kobject,
// @mocs: info
// @mocs.uc_index: UC index
    pub uc_index: u8,
// @mocs.wb_index: WB index, only used on L3_CCS platforms
    pub wb_index: u8,
    pub mocs: },
// @fuse_topo: GT topology reported by fuse registers
// @fuse_topo.g_dss_mask: dual-subslices usable by geometry
    pub g_dss_mask: xe_dss_mask_t,
// @fuse_topo.c_dss_mask: dual-subslices usable by compute
    pub c_dss_mask: xe_dss_mask_t,
// @fuse_topo.eu_mask_per_dss: EU mask per DSS
    pub eu_mask_per_dss: xe_eu_mask_t,
// @fuse_topo.l3_bank_mask: L3 bank mask
    pub l3_bank_mask: xe_l3_bank_mask_t,
//
// @fuse_topo.eu_type: type/width of EU stored in
// fuse_topo.eu_mask_per_dss
//
    pub eu_type: xe_gt_eu_type,
    pub fuse_topo: },
// @steering: register steering for individual HW units
// @steering.ranges: register ranges used for this steering type
    pub ranges: *const xe_mmio_range,
// @steering.group_target: target to steer accesses to
    pub group_target: u16,
// @steering.instance_target: instance to steer accesses to
    pub instance_target: u16,
// @steering.initialized: Whether this steering range is initialized
    pub initialized: bool,
    pub steering: [}; NUM_STEERING_TYPES],
//
// @steering_dss_per_grp: number of DSS per steering group (gslice,
// cslice, etc.).
//
    pub steering_dss_per_grp: c_uint,
//
// @mcr_lock: protects the MCR_SELECTOR register for the duration
// of a steered operation
//
    pub mcr_lock: spinlock_t,
//
// @global_invl_lock: protects the register for the duration
// of a global invalidation of l2 cache
//
    pub global_invl_lock: spinlock_t,
// @wa_active: keep track of active workarounds
// @wa_active.gt: bitmap with active GT workarounds
    pub gt: *mut c_ulong,
// @wa_active.engine: bitmap with active engine workarounds
    pub engine: *mut c_ulong,
// @wa_active.lrc: bitmap with active LRC workarounds
    pub lrc: *mut c_ulong,
// @wa_active.oob: bitmap with active OOB workarounds
    pub oob: *mut c_ulong,
//
// @wa_active.oob_initialized: mark oob as initialized to help
// detecting misuse of XE_GT_WA() - it can only be called on
// initialization after OOB WAs have been processed
//
    pub oob_initialized: bool,
    pub wa_active: },
// @tuning_active: keep track of active tunings
// @tuning_active.gt: bitmap with active GT tunings
    pub gt: *mut c_ulong,
// @tuning_active.engine: bitmap with active engine tunings
    pub engine: *mut c_ulong,
// @tuning_active.lrc: bitmap with active LRC tunings
    pub lrc: *mut c_ulong,
    pub tuning_active: },
// @user_engines: engines present in GT and available to userspace
//
// @user_engines.mask: like @info.engine_mask, but take in
// consideration only engines available to userspace
//
    pub mask: u64,
//
// @user_engines.instances_per_class: aggregate per class the
// number of engines available to userspace
//
    pub instances_per_class: [u8; XE_ENGINE_CLASS_MAX],
    pub user_engines: },
// @oa: oa observation subsystem per gt info
    pub oa: xe_oa_gt,
// @eu_stall: EU stall counters subsystem per gt info
    pub eu_stall: *mut xe_eu_stall_gt,
}
