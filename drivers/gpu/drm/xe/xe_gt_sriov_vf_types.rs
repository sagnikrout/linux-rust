//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_sriov_vf_types.h
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
// Copyright © 2023-2024 Intel Corporation
//

//
// struct xe_gt_sriov_vf_selfconfig - VF configuration data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_vf_selfconfig {
// @num_ctxs: assigned number of GuC submission context IDs.
    pub num_ctxs: u16,
// @num_dbs: assigned number of GuC doorbells IDs.
    pub num_dbs: u16,
}

//
// struct xe_gt_sriov_vf_runtime - VF runtime data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_vf_runtime {
// @gmdid: cached value of the GDMID register.
    pub gmdid: u32,
// @uses_sched_groups: whether PF enabled sched groups or not.
    pub uses_sched_groups: bool,
//
// @num_paging_engine_instances: number of configured paging engines.
//
    pub num_paging_engine_instances: u32,
// @regs_size: size of runtime register array.
    pub regs_size: u32,
// @num_regs: number of runtime registers in the array.
    pub num_regs: u32,
// @regs: pointer to array of register offset/value pairs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf_runtime_reg {
// @regs.offset: register offset.
    pub offset: u32,
// @regs.value: register value.
    pub value: u32,
    pub regs: *mut },
}

//
// struct xe_gt_sriov_vf_migration - VF migration data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_vf_migration {
// @worker: VF migration recovery worker
    pub worker: work_struct,
// @lock: Protects recovery_queued, teardown
    pub lock: spinlock_t,
// @wq: wait queue for migration fixes
    pub wq: wait_queue_head_t,
// @scratch: Scratch memory for VF recovery
    pub scratch: *mut c_void,
// @fixups_complete_count: Counts completed fixups stages
    pub fixups_complete_count: core::sync::atomic::AtomicI32,
// @debug: Debug hooks for delaying migration
//
// @debug.resfix_stoppers: Stop and wait at different stages
// during post migration recovery
//
    pub resfix_stoppers: u8,
    pub debug: },
//
// @resfix_marker: Marker sent on start and on end of post-migration
// steps.
//
    pub resfix_marker: u8,
// @recovery_teardown: VF post migration recovery is being torn down
    pub recovery_teardown: bool,
// @recovery_queued: VF post migration recovery in queued
    pub recovery_queued: bool,
// @recovery_inprogress: VF post migration recovery in progress
    pub recovery_inprogress: bool,
// @ggtt_need_fixes: VF GGTT and references to it need fixes
    pub ggtt_need_fixes: bool,
}

//
// struct xe_gt_sriov_vf - GT level VF virtualization data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_gt_sriov_vf {
// @wanted_guc_version: minimum wanted GuC ABI version.
    pub wanted_guc_version: xe_uc_fw_version,
// @guc_version: negotiated GuC ABI version.
    pub guc_version: xe_uc_fw_version,
// @self_config: resource configurations.
    pub self_config: xe_gt_sriov_vf_selfconfig,
// @runtime: runtime data retrieved from the PF.
    pub runtime: xe_gt_sriov_vf_runtime,
// @migration: migration data for the VF.
    pub migration: xe_gt_sriov_vf_migration,
}
